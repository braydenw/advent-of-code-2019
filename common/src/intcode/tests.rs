//! These tests mostly just use the examples from a given day.  
//! They're used to ensure the virtual machine continues working
//! properly as changes get made.
//! Of course, one could just run `cargo test` from the workspace
//! root to run every test from every day.

use crate::IntcodeVM;

#[test]
fn day02_examples() {
    let mut vm = IntcodeVM::new()
        .with_logging(2)
        .with_program(&"1,0,0,0,99".to_string());
    vm.run();
    assert_eq!(vec![2,0,0,0,99], vm.dump_memory(0..5));

    vm.load_program(&"2,3,0,3,99".to_string());
    vm.run();
    assert_eq!(vec![2,3,0,6,99], vm.dump_memory(0..5));

    vm.load_program(&"2,4,4,5,99,0".to_string());
    vm.run();
    assert_eq!(vec![2,4,4,5,99,9801], vm.dump_memory(0..6));

    vm.load_program(&"1,1,1,4,99,5,6,0,99".to_string());
    vm.run();
    assert_eq!(vec![30,1,1,4,2,5,6,0,99], vm.dump_memory(0..9));
}

#[test]
fn day05_examples() {
    // #### PART 1 ####

    let mut vm = IntcodeVM::new()
        .with_logging(2)
        .with_program(&"1002,4,3,4,33".to_string());
    vm.run();
    assert_eq!(vec![1002,4,3,4,99], vm.dump_memory(0..5));

    // #### PART 2 ####

    // Less than and equal tests
    let io_handle = vm.io();
    vm.load_program(&"3,9,8,9,10,9,4,9,99,-1,8".to_string());
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

fn day07_examples() {
    // Just run `cargo test -p day07` for these.
}