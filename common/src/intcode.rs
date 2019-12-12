//! Reasonably quick Intcode virtual machine.

/// This is just an arbitrary size that seems to be able to hold any
/// program Advent of Code gives.
const MEMORY_SIZE: usize = 2048;

/// The Intcode virtual machine itself.  
/// `memory` represents the machine's memory.
/// `pointer` is the instruction pointer.
/// `log_level` is used to determine how much output is given.
pub struct IntcodeVM {
    memory: [i32; MEMORY_SIZE],
    pointer: usize,
    log_level: u8,
}

impl IntcodeVM {

    /// Create a new Intcode virtual machine that will run on `input`.
    pub fn new<S: Into<String>>(input: S) -> IntcodeVM {
        let mut me = IntcodeVM {
            memory: [0; MEMORY_SIZE],
            pointer: 0,
            log_level: 0,
        };
        me.set_input(input);

        me
    }

    /// Set the logging level; 0: Nothing, 1: Errors, 2: Everything.
    pub fn log_level(mut self, level: u8) -> IntcodeVM {
        self.log_level = level;

        self
    }

    /// Set the machine's input, overwriting existing memory.
    /// Perhaps worth noting is that the machine memory is merely overwritten,
    /// not cleared then rewritten.
    /// 
    pub fn set_input<S: Into<String>>(&mut self, input: S) {
        let ints: Vec<i32> = input.into().trim().split(',')
            .map(|s| s.parse::<i32>().unwrap()).collect();
        
        for i in 0..ints.len() {
            self.memory[i] = ints[i];
        }

        self.pointer = 0;
    }

    pub fn run(&mut self) {
        self.pointer = 0;

        'processor: loop {
            let (opcode, modes) = self.read_instr();

            match opcode {
                01 => { // ADD R R W
                    self.info("ADD");

                    let p1 = self.read_param(modes[0]);
                    let p2 = self.read_param(modes[1]);
                    
                    self.write_param(p1 + p2);
                },
                02 => { // MUL R R W
                    self.info("MUL");

                    let p1 = self.read_param(modes[0]);
                    let p2 = self.read_param(modes[1]);

                    self.write_param(p1 * p2);
                },
                03 => { // NPT W
                    self.info("NPT");

                    let int = match super::read_stdin().parse::<i32>() {
                        Ok(i) => i,
                        Err(_) => {
                            self.error("input not a valid integer");
                            break 'processor;
                        }
                    };

                    self.write_param(int);
                },
                04 => { // OPT R
                    self.info("OPT");

                    println!("{}", self.read_param(modes[0]));
                },
                99 => {
                    self.info("HLT");
                    break 'processor;
                },
                __ => {
                    self.error("invalid opcode");
                    break 'processor;
                }
            };
        }
    }

    /// Reads an instruction from memory at the instruction pointer
    /// as well as returns the modes for each parameter.
    fn read_instr(&mut self) -> (u8, [u8; 3]) {
        let value = self.memory[self.pointer];
        self.pointer += 1;
        
        let instr = (value % 100) as u8;
        let modes = [
            ((value / 100) % 10) as u8,
            ((value / 1000) % 10) as u8,
            ((value / 10000) % 10) as u8,
        ];
            
        (instr, modes)
    }

    /// Reads a parameter from memory.
    fn read_param(&mut self, mode: u8) -> i32 {
        let param = self.memory[self.pointer];
        self.pointer += 1;

        match mode {
            0 => self.memory[param as usize],
            1 => param,
            _ => std::i32::MAX,
        }
    }

    /// Write a value to memory.
    /// The position in memory is determined by the value currently
    /// under the instruction pointer.
    fn write_param(&mut self, value: i32) {
        let param = self.memory[self.pointer];
        self.pointer += 1;
        
        self.memory[param as usize] = value;
    }

    fn info(&self, msg: &'static str) {
        if self.log_level >= 2 {
            println!("[ INFO_{:04}::{:<8}] {}", self.pointer - 1, self.memory[self.pointer - 1], msg);
        }
    }

    fn error(&self, msg: &'static str) {
        if self.log_level >= 1 {
            eprintln!("[ERROR_{:04}::{:<8}] {}", self.pointer - 1, self.memory[self.pointer - 1], msg);
        }
    }
}

#[test]
fn day02_examples() {
    let mut vm = IntcodeVM::new("1,0,0,0,99").log_level(2);
    vm.run();
    assert_eq!([2,0,0,0,99], vm.memory[..5]);

    vm.set_input("2,3,0,3,99");
    vm.run();
    assert_eq!([2,3,0,6,99], vm.memory[..5]);

    vm.set_input("2,4,4,5,99,0");
    vm.run();
    assert_eq!([2,4,4,5,99,9801], vm.memory[..6]);

    vm.set_input("1,1,1,4,99,5,6,0,99");
    vm.run();
    assert_eq!([30,1,1,4,2,5,6,0,99], vm.memory[..9]);
}

#[test]
fn day05_examples() {
    let mut vm = IntcodeVM::new("1002,4,3,4,33").log_level(2);
    vm.run();
    assert_eq!([1002,4,3,4,99], vm.memory[..5]);
}