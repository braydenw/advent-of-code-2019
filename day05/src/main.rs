//! Most of the code from this day is within
//! the `common` library's Intcode virtual machine.

use common::*;

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&input, part_one, part_two);
}

fn part_one(input: &String) {
    let mut vm = IntcodeVM::new()
        .with_logging(0)
        .with_program(input)
        .with_input(default_input);
    vm.run();

    let mut final_output = 0;
    let io_handle = vm.io();
    while let Some(out) = io_handle.recv() {
        final_output = out;
    }
    println!("[Part 1] Final output: {}", final_output);
}

fn part_two(input: &String) {
    let mut vm = IntcodeVM::new()
        .with_logging(0)
        .with_program(input)
        .with_input(default_input);
    vm.run();

    let output = vm.io().recv().unwrap();
    println!("[Part 2] Final output: {}", output);
}

/// This is a bad way to do tests. They should really be split
/// into many smaller tests if this were for anything serious.
#[test]
fn part_two_examples() {
    // Less than and equal tests
    let mut vm = IntcodeVM::new()
        .with_program(&"3,9,8,9,10,9,4,9,99,-1,8".to_string());
    let io_handle = vm.io();
    io_handle.send(8);
    vm.run();
    assert_eq!(1, io_handle.recv().unwrap());

    vm.load_program(&"3,9,7,9,10,9,4,9,99,-1,8".to_string());
    io_handle.send(8);
    vm.run();
    assert_eq!(0, io_handle.recv().unwrap());
    
    vm.load_program(&"3,3,1108,-1,8,3,4,3,99".to_string());
    io_handle.send(8);
    vm.run();
    assert_eq!(1, io_handle.recv().unwrap());
    
    vm.load_program(&"3,3,1107,-1,8,3,4,3,99".to_string());
    io_handle.send(8);
    vm.run();
    assert_eq!(0, io_handle.recv().unwrap());
    
    // Jump tests
    vm.load_program(&"3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".to_string());
    io_handle.send(0);
    vm.run();
    assert_eq!(0, io_handle.recv().unwrap());
    
    vm.load_program(&"3,3,1105,-1,9,1101,0,0,12,4,12,99,1".to_string());
    io_handle.send(1);
    vm.run();
    assert_eq!(1, io_handle.recv().unwrap());
    
    // Combined
    vm.load_program(&"3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string());
    io_handle.send(3);
    vm.run();
    assert_eq!(999, io_handle.recv().unwrap());
}