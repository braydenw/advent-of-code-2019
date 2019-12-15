use crossbeam_channel::{unbounded, Sender, Receiver};

const MEMORY_SIZE: usize = 8192;

pub enum Message {
    HaltTerminate,
    HaltNeedInput,
}

pub struct IOHandle<T, R>(Sender<T>, Receiver<R>);
impl<T, R> IOHandle<T, R> {
    pub fn send(&self, data: T) {
        self.0.send(data)
            .expect("unable to send input");
    }

    pub fn recv(&self) -> Option<R> {
        if let Ok(data) = self.1.try_recv() {
            Some(data)
        } else {
            None
        }
    }
}

pub struct Messenger<Message>(Sender<Message>, Receiver<Message>);
impl<Message> Messenger<Message> {
    pub fn send(&self, data: Message) {
        self.0.send(data)
            .expect("unable to send message");
    }

    pub fn recv(&self) -> Option<Message> {
        if let Ok(msg) = self.1.try_recv() {
            Some(msg)
        } else {
            None
        }
    }
}

/// The Intcode virtual machine itself.
pub struct IntcodeVM {
    memory: [i64; MEMORY_SIZE],
    instr_pointer: usize,
    relative_base: usize,
    input_sender: Sender<i64>,
    input_recver: Receiver<i64>,
    output_sender: Sender<i64>,
    output_recver: Receiver<i64>,
    message_sender: Sender<Message>,
    message_recver: Receiver<Message>,
    log_level: u8,
    input_fn: Option<fn() -> i64>,
    output_fn: Option<fn(i64)>,
}

impl IntcodeVM {

    /// Create a new Intcode virtual machine.
    pub fn new() -> IntcodeVM {
        let (input_s, input_r) = unbounded();
        let (output_s, output_r) = unbounded();
        let (msg_s, msg_r) = unbounded();

        IntcodeVM {
            memory: [0; MEMORY_SIZE],
            instr_pointer: 0,
            relative_base: 0,
            input_sender: input_s,
            input_recver: input_r,
            output_sender: output_s,
            output_recver: output_r,
            message_sender: msg_s,
            message_recver: msg_r,
            log_level: 0,
            input_fn: None,
            output_fn: None,
        }
    }

    /// Set the logging level; 0: Nothing, 1: Errors, 2: Everything.
    pub fn with_logging(mut self, level: u8) -> IntcodeVM {
        self.log_level = level;

        self
    }

    /// Load the specified program.
    pub fn with_program(mut self, program: &String) -> IntcodeVM {
        self.load_program(program);

        self
    }

    pub fn with_input(mut self, f: fn() -> i64) -> IntcodeVM {
        self.input_fn = Some(f);

        self
    }

    pub fn with_output(mut self, f: fn(i64)) -> IntcodeVM {
        self.output_fn = Some(f);

        self
    }

    /// Resets the Intcode virtual machine.
    pub fn reset(&mut self) {
        self.memory = [0; MEMORY_SIZE];
        self.instr_pointer = 0;
        self.relative_base = 0;

        // Drain the channels.
        self.input_recver.try_iter().for_each(drop);
        self.output_recver.try_iter().for_each(drop);
        self.message_recver.try_iter().for_each(drop);
    }

    /// Load a program into memory.
    pub fn load_program(&mut self, program: &String) {
        let program = parse_program(program);
        
        self.reset();

        for i in 0..program.len() {
            self.memory[i] = program[i];
        }
    }

    /// Step through the program until `State::Halted`.
    pub fn run(&mut self) {
        loop {
            if let Ok(msg) = self.message_recver.try_recv() {
                match msg {
                    Message::HaltTerminate => {
                        break;
                    },
                    Message::HaltNeedInput => {
                        // std::thread::sleep(std::time::Duration::from_millis(1));
                        continue;
                    },
                }
            }

            self.step();
        }
    }

    /// Process a single instruction.
    pub fn step(&mut self) {
        let (opcode, modes) = self.read_instr();

        match opcode {

            // Opcode: add OR mul
            // Params: read read write
            01 | 02 => {
                let p1 = self.read_param(modes[0]);
                let p2 = self.read_param(modes[1]);

                let val = if opcode == 1 {
                    let v = p1 + p2;
                    self.info(|| instr_encode("ADD", [Some(p1), Some(p2), Some(v)], modes));

                    v
                } else {
                    let v = p1 * p2;
                    self.info(|| instr_encode("MUL", [Some(p1), Some(p2), Some(v)], modes));
                    
                    v
                };

                self.write_param(val);
            },
            
            // Opcode: input
            // Params: write
            03 => {
                if let Some(f) = self.input_fn {
                    let val = (f)();
                    self.info(|| instr_encode("NPT", [Some(val), None, None], modes));
                    
                    self.write_param(val);
                } else if let Ok(int) = self.input_recver.try_recv() {
                    self.info(|| instr_encode("NPT", [Some(int), None, None], modes));

                    self.write_param(int);
                } else {
                    self.message_sender.send(Message::HaltNeedInput)
                        .expect("unable to send wait message");

                    // Rewind the instr_pointer
                    self.instr_pointer -= 1;
                }
            },

            // Opcode: output
            // Params: read
            04 => {
                let val = self.read_param(modes[0]);
                self.info(|| instr_encode("OPT", [Some(val), None, None], modes));

                if let Some(f) = self.output_fn {
                    (f)(val);
                } else {
                    self.output_sender.send(val)
                        .expect("unable to send output");
                }
            },
            
            // Opcode: jump-if-true OR jump-if-false
            // Params: read read
            05 | 06 => {
                let p1 = self.read_param(modes[0]);
                let p2 = self.read_param(modes[1]);

                let cond = if opcode == 5 {
                    self.info(|| instr_encode("JT", [Some(p1), Some(p2), None], modes));
                    p1 != 0
                } else {
                    self.info(|| instr_encode("JF", [Some(p1), Some(p2), None], modes));
                    p1 == 0
                };

                if cond {
                    self.instr_pointer = p2 as usize;
                }
            },
            
            // Opcode: less than OR equals
            // Params: read read write
            07 | 08 => {
                let p1 = self.read_param(modes[0]);
                let p2 = self.read_param(modes[1]);

                let cond = if opcode == 7 {
                    let val = p1 < p2;
                    self.info(|| instr_encode("LT", [Some(p1), Some(p2), Some(val as i64)], modes));

                    val
                } else {
                    let val = p1 == p2;
                    self.info(|| instr_encode("EQ", [Some(p1), Some(p2), Some(val as i64)], modes));

                    val
                };

                if cond {
                    self.write_param(1);
                } else {
                    self.write_param(0);
                }
            },
            
            // Opcode: halt
            // Params: none
            99 => {
                self.info(|| format!("HLT"));
                self.message_sender.send(Message::HaltTerminate)
                    .expect("unable to send message");
            },
            
            // Opcode: unknown
            // Params: perhaps many
            __ => {
                self.error(|| format!("unknown opcode: {}", opcode));
                self.message_sender.send(Message::HaltTerminate)
                    .expect("unable to send message");
            }
        }
    }

    /// Get a handle to the machines IO.
    pub fn io(&self) -> IOHandle<i64, i64> {
        IOHandle(self.input_sender.clone(), self.output_recver.clone())
    }

    pub fn messenger(&self) -> Messenger<Message> {
        Messenger(self.message_sender.clone(), self.message_recver.clone())
    }

    /// Get a reference to the machine's memory.
    pub fn dump_memory(&self, range: std::ops::Range<usize>) -> Vec<i64> {
        self.memory[range].to_vec()
    }

    /// Reads an instruction from memory at the instruction pointer
    /// as well as returns the modes for each parameter.
    fn read_instr(&mut self) -> (u8, [u8; 3]) {
        let value = self.memory[self.instr_pointer];
        self.instr_pointer += 1;
        
        let instr = (value % 100) as u8;
        let modes = [
            ((value / 100) % 10) as u8,
            ((value / 1000) % 10) as u8,
            ((value / 10000) % 10) as u8,
        ];
            
        (instr, modes)
    }

    /// Reads a parameter from memory.
    fn read_param(&mut self, mode: u8) -> i64 {
        let param = self.memory[self.instr_pointer];
        self.instr_pointer += 1;

        match mode {
            0 => self.memory[param as usize],
            1 => param,
            _ => std::i64::MAX,
        }
    }

    /// Write a value to memory.
    /// The position in memory is determined by the value currently
    /// under the instruction pointer.
    fn write_param(&mut self, value: i64) {
        let param = self.memory[self.instr_pointer];
        self.instr_pointer += 1;
        
        self.memory[param as usize] = value;
    }

    /// Prints an informative message, including the current instruction
    /// pointer and the value to which it points in memory.
    fn info<F: FnOnce() -> String>(&self, f: F) {
        if self.log_level >= 2 {
            println!("[{:>4}] {}", self.instr_pointer - 1, f());
        }
    }

    /// Prints an error message, including the current instruction
    /// pointer and the value to which it points in memory.
    fn error<F: FnOnce() -> String>(&self, f: F) {
        if self.log_level >= 1 {
            eprintln!("[{:>4}] #### {}", self.instr_pointer - 1, f());
        }
    }
}

/// Parse a program from a `String`, returning it as a `Vec<i64>`.
pub fn parse_program(program: &String) -> Vec<i64> {
    program.trim().split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}


pub fn default_input() -> i64 {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");

    line.trim().parse::<i64>()
        .expect("input was not an integer")
}

/// Turns an instruction into a String, for printing purposes.
fn instr_encode(opcode: &'static str, params: [Option<i64>; 3], modes: [u8; 3]) -> String {
    let mut out = String::new();
    out.push_str(&format!("{:8} ", opcode));

    for i in 0..3 {
        if let Some(p) = params[i] {
            match modes[i] {
                0 => out.push_str(&format!("[p]0x{:<16X} ", p)),
                1 => out.push_str(&format!("[i]0x{:<16X} ", p)),
                2 => out.push_str(&format!("[r]0x{:<16X} ", p)),
                _ => {}
            }
        }
    }

    out.trim().to_string()
}