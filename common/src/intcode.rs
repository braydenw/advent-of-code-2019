//! Reasonably quick Intcode virtual machine.

use std::collections::VecDeque;

/// This is just an arbitrary size that seems to be able to hold any
/// program Advent of Code gives.
const MEMORY_SIZE: usize = 2048;

pub enum State {
    Halted,
    Running,
}

/// The Intcode virtual machine itself.  
/// `memory` represents the machine's memory.
/// `pointer` is the instruction pointer.
/// `log_level` is used to determine how much output is given.
pub struct IntcodeVM {
    memory: [i32; MEMORY_SIZE],
    pointer: usize,
    log_level: u8,
    inputs: VecDeque<i32>,
    outputs: Vec<i32>,
}

impl IntcodeVM {

    /// Create a new Intcode virtual machine that will run on `input`.
    pub fn new<S: Into<String>>(input: S) -> IntcodeVM {
        let mut me = IntcodeVM {
            memory: [0; MEMORY_SIZE],
            pointer: 0,
            log_level: 0,
            inputs: VecDeque::with_capacity(32),
            outputs: Vec::with_capacity(32),
        };
        me.set_program(input);

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
    pub fn set_program<S: Into<String>>(&mut self, program: S) {
        let program = parse_program(program);
        
        for i in 0..program.len() {
            self.memory[i] = program[i];
        }
    }

    /// Step through the program until `State::Error` or `State::Halted`.
    pub fn run(&mut self) {
        self.pointer = 0;
        self.outputs.clear();

        while let State::Running = self.step() {
            // ???
        }

        // loop {
        //     match self.step() {
        //         State::Running => {
        //             self.step();
        //         },
        //         State::Halted => {
        //             break;
        //         },
        //         State::Error(msg) => {
        //             self.error(msg);
        //             break;
        //         }
        //     }
        // }
    }

    pub fn step(&mut self) -> State {
        let (opcode, modes) = self.read_instr();

        match opcode {
            01 | 02 => { // ADD R R W | MUL R R W
                let p1 = self.read_param(modes[0]);
                let p2 = self.read_param(modes[1]);

                let val = if opcode == 1 {
                    self.info("ADD");
                    p1 + p2
                } else {
                    self.info("MUL");
                    p1 * p2
                };

                self.write_param(val);
            },
            03 => { // NPT W
                self.info("NPT");

                if let Some(int) = self.inputs.pop_front() {
                    self.write_param(int);
                } else {
                    if let Ok(i) = super::read_stdin().parse::<i32>() {
                        self.write_param(i);
                    } else {
                        self.error("input was not a valid integer");
                        return State::Halted;
                    }
                }
            },
            04 => { // OPT R
                self.info("OPT");

                let val = self.read_param(modes[0]);
                self.outputs.push(val);
            },
            05 | 06 => { // JT R R | JF R R
                let p1 = self.read_param(modes[0]);
                let p2 = self.read_param(modes[1]);

                let cond = if opcode == 5 {
                    self.info("JT");
                    p1 != 0
                } else {
                    self.info("JF");
                    p1 == 0
                };

                if cond {
                    self.pointer = p2 as usize;
                }
            },
            07 | 08 => { // LT R R W | EQ R R W
                let p1 = self.read_param(modes[0]);
                let p2 = self.read_param(modes[1]);

                let cond = if opcode == 7 {
                    self.info("LT");
                    p1 < p2
                } else {
                    self.info("EQ");
                    p1 == p2
                };

                if cond {
                    self.write_param(1);
                } else {
                    self.write_param(0);
                }
            },
            99 => {
                self.info("HLT");
                return State::Halted;
            },
            __ => {
                self.error("invalid opcode");
                return State::Halted;
            }
        }

        return State::Running;
    }

    pub fn inputs(&self) -> Vec<&i32> {
        self.inputs.iter().collect()
    }

    pub fn push_input(&mut self, int: i32) {
        self.inputs.push_back(int);
    }

    /// The outputs of a program.
    /// Useful for comparing expected outputs programmatically.
    pub fn outputs(&self) -> &Vec<i32> {
        &self.outputs
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
            println!("[ INFO_{:04}::{:<8}] {}",
                self.pointer - 1, self.memory[self.pointer - 1], msg);
        }
    }

    fn error(&self, msg: &'static str) {
        if self.log_level >= 1 {
            eprintln!("[ERROR_{:04}::{:<8}] {}",
                self.pointer - 1, self.memory[self.pointer - 1], msg);
        }
    }
}

pub fn parse_program<S: Into<String>>(program: S) -> Vec<i32> {
    program.into().trim().split(',')
        .map(|s| s.parse::<i32>().unwrap()).collect()
}

#[test]
fn day02_examples() {
    let mut vm = IntcodeVM::new("1,0,0,0,99").log_level(2);
    vm.run();
    assert_eq!([2,0,0,0,99], vm.memory[..5]);

    vm.set_program("2,3,0,3,99");
    vm.run();
    assert_eq!([2,3,0,6,99], vm.memory[..5]);

    vm.set_program("2,4,4,5,99,0");
    vm.run();
    assert_eq!([2,4,4,5,99,9801], vm.memory[..6]);

    vm.set_program("1,1,1,4,99,5,6,0,99");
    vm.run();
    assert_eq!([30,1,1,4,2,5,6,0,99], vm.memory[..9]);
}

#[test]
fn day05_examples() {
    let mut vm = IntcodeVM::new("1002,4,3,4,33").log_level(2);
    vm.run();
    assert_eq!([1002,4,3,4,99], vm.memory[..5]);
}