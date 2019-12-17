//! Uses channels for input, output, and message passing.
//! Doing so hopefully simplifies future multithreading.

use crossbeam_channel::{unbounded, Sender, Receiver};

const MEMORY_SIZE: usize = 8192;

pub enum Message {
    HaltTerminate,
    HaltNeedInput,
}

pub struct IOHandle<T, R>(Sender<T>, pub Receiver<R>);
impl<T, R> IOHandle<T, R> {

    /// Send `data` as input.
    pub fn send(&self, data: T) {
        self.0.send(data)
            .expect("unable to send input");
    }

    /// Receive output data, or `None` if there are none in the output queue.
    pub fn recv(&self) -> Option<R> {
        if let Ok(data) = self.1.try_recv() {
            Some(data)
        } else {
            None
        }
    }

    /// Blocks the thread waiting for output data.
    pub fn wait_recv(&self) -> Option<R> {
        if let Ok(data) = self.1.recv() {
            Some(data)
        } else {
            None
        }
    }

    /// Collects all data from the output queue and returns it.
    pub fn dump(&self) -> Vec<R> {
        self.1.try_iter().collect()
    }

    /// Get the number of queued outputs.
    pub fn count_output(&self) -> usize {
        self.1.len()
    }
}

pub struct Messenger<Message>(Sender<Message>, Receiver<Message>);
impl<Message> Messenger<Message> {

    /// Send a `Message` through the `Messenger`.
    pub fn send(&self, msg: Message) {
        self.0.send(msg)
            .expect("unable to send message");
    }

    /// Receive a `Message` from the `Messenger`.
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
    relative_base: i64,
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

    /// Step through the program until a `Message::HaltTerminate` is received.
    pub fn run(&mut self) {
        loop {
            if let Ok(msg) = self.message_recver.try_recv() {
                match msg {
                    Message::HaltTerminate => {
                        break;
                    },
                    Message::HaltNeedInput => {
                        // std::thread::sleep(std::time::Duration::from_millis(100));
                        continue;
                    },
                }
            }

            if let Some(Message::HaltTerminate) = self.step() {
                break;
            }
        }
    }

    /// Process a single instruction.
    /// Returns a `Message` as well as sending it to the
    /// virtual machine's `Messenger`.
    /// The returned `Message` can be used for careful synchronous
    /// stepping while the `Messenger` is recommended for asynchronous
    /// stepping.
    pub fn step(&mut self) -> Option<Message> {
        let (opcode, modes) = self.read_instr();

        match opcode {

            // Opcode: add OR mul
            // Params: read read write
            01 | 02 => {
                let p1 = self.read_param(modes[0]);
                let p2 = self.read_param(modes[1]);

                let val = if opcode == 1 {
                    let v = p1 + p2;
                    self.info(self.instr_pointer - 4,
                        || instr_encode("ADD", [Some(p1), Some(p2), Some(v)], modes));

                    v
                } else {
                    let v = p1 * p2;
                    self.info(self.instr_pointer - 4,
                        || instr_encode("MUL", [Some(p1), Some(p2), Some(v)], modes));
                    
                    v
                };

                self.write_param(val, modes[2]);
            },
            
            // Opcode: input
            // Params: write
            03 => {
                if let Ok(int) = self.input_recver.try_recv() {
                    self.info(self.instr_pointer - 1,
                        || instr_encode("NPT", [Some(int), None, None], modes));

                    self.write_param(int, modes[0]);
                } else if let Some(f) = self.input_fn {
                    let val = (f)();
                    self.info(self.instr_pointer - 1,
                        || instr_encode("NPT", [Some(val), None, None], modes));
                    
                    self.write_param(val, modes[0]);
                } else {
                    self.message_sender.send(Message::HaltNeedInput)
                        .expect("unable to send wait message");

                    // Rewind the instr_pointer
                    self.instr_pointer -= 1;

                    return Some(Message::HaltNeedInput);
                }
            },

            // Opcode: output
            // Params: read
            04 => {
                let val = self.read_param(modes[0]);
                self.info(self.instr_pointer - 2,
                    || instr_encode("OPT", [Some(val), None, None], modes));

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
                    self.info(self.instr_pointer - 3,
                        || instr_encode("JT", [Some(p1), Some(p2), None], modes));

                    p1 != 0
                } else {
                    self.info(self.instr_pointer - 3,
                        || instr_encode("JF", [Some(p1), Some(p2), None], modes));

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
                    self.info(self.instr_pointer - 4,
                        || instr_encode("LT", [Some(p1), Some(p2), Some(val as i64)], modes));

                    val
                } else {
                    let val = p1 == p2;
                    self.info(self.instr_pointer - 4,
                        || instr_encode("EQ", [Some(p1), Some(p2), Some(val as i64)], modes));

                    val
                };

                if cond {
                    self.write_param(1, modes[2]);
                } else {
                    self.write_param(0, modes[2]);
                }
            },

            // Opcode: set relative base
            // Params: read
            9 => {
                let p1 = self.read_param(modes[0]);

                self.info(self.instr_pointer - 2,
                    || instr_encode("REL", [Some(p1), None, None], modes));

                self.relative_base += p1;
            }
            
            // Opcode: halt
            // Params: none
            99 => {
                self.info(self.instr_pointer - 1,
                    || format!("HLT"));

                self.message_sender.send(Message::HaltTerminate)
                    .expect("unable to send message");

                return Some(Message::HaltTerminate);
            },
            
            // Opcode: unknown
            // Params: perhaps many
            __ => {
                self.error(self.instr_pointer - 1,
                    || format!("unknown opcode: {}", opcode));

                self.message_sender.send(Message::HaltTerminate)
                    .expect("unable to send message");
            }
        }

        None
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
            // Position
            0 => self.memory[param as usize],

            // Immediate
            1 => param,

            // Relative
            2 => self.memory[(self.relative_base + param) as usize],

            // Invalid, return garbage.
            _ => std::i64::MAX,
        }
    }

    /// Write a value to memory.
    fn write_param(&mut self, value: i64, mode: u8) {
        let param = self.memory[self.instr_pointer];
        self.instr_pointer += 1;
        
        match mode {
            // Position
            0 => self.memory[param as usize] = value,

            // Relative
            2 => self.memory[(self.relative_base + param) as usize] = value,

            // Invalid, do nothing.
            _ => {},
        }
    }

    /// Prints an informative message, including the current instruction
    /// pointer and the value to which it points in memory.
    fn info<F: FnOnce() -> String>(&self, offset: usize, f: F) {
        if self.log_level >= 2 {
            println!("[{:>4}] {}", offset, f());
        }
    }

    /// Prints an error message, including the current instruction
    /// pointer and the value to which it points in memory.
    fn error<F: FnOnce() -> String>(&self, offset: usize, f: F) {
        if self.log_level >= 1 {
            eprintln!("[{:>4}] #### {}", offset, f());
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