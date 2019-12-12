//! Most of this code from this day in the Intcode virtual mashine
//! within the `common` library.

use common::*;

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(input, part_one, part_two);
}

/// Seems to average ~0.73ms.
fn part_one(input: String) {
    let mut vm = IntcodeVM::new(input).log_level(1);
    vm.run();
}

/// Seems to average ~0.21ms.
fn part_two(input: String) {
    let mut vm = IntcodeVM::new(input).log_level(1);
    vm.run();
}

/// To be run from workspace root with 
/// `cat day05/test_inputs.txt | cargo test -p day05`.
/// Also, this is a bad way to do tests. They should really be split
/// into many smaller tests if this were for anything serious.
#[test]
fn part_two_examples() {
    // Less than and equal tests
    let mut vm = IntcodeVM::new("3,9,8,9,10,9,4,9,99,-1,8");
    vm.run();
    assert_eq!(1, vm.outputs()[0]);

    vm.set_input("3,9,7,9,10,9,4,9,99,-1,8");
    vm.run();
    assert_eq!(0, vm.outputs()[0]);
    
    vm.set_input("3,3,1108,-1,8,3,4,3,99");
    vm.run();
    assert_eq!(1, vm.outputs()[0]);
    
    vm.set_input("3,3,1107,-1,8,3,4,3,99");
    vm.run();
    assert_eq!(0, vm.outputs()[0]);
    
    // Jump tests
    vm.set_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    vm.run();
    assert_eq!(0, vm.outputs()[0]);
    
    vm.set_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
    vm.run();
    assert_eq!(1, vm.outputs()[0]);
    
    // Combined
    vm.set_input("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    vm.run();
    assert_eq!(999, vm.outputs()[0]);
}