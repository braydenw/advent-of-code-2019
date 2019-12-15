use common::*;

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&input, part_one, part_two);
}

fn part_one(input: &String) {
    let mut vm = IntcodeVM::new()
        .with_logging(1)
        .with_program(input);
    let io_handle = vm.io();

    io_handle.send(1);
    vm.run();

    assert_eq!(1, io_handle.count_output());

    println!("[Part 1] BOOST keycode: {}", io_handle.recv().unwrap());
}

fn part_two(input: &String) {
    let mut vm = IntcodeVM::new()
        .with_logging(1)
        .with_program(input);
    let io_handle = vm.io();

    io_handle.send(2);
    vm.run();

    assert_eq!(1, io_handle.count_output());

    println!("[Part 2] Distress signal coordinates: {}", io_handle.recv().unwrap());
}

#[test]
fn part_one_examples() {
    let mut vm = IntcodeVM::new()
        .with_logging(2)
        .with_program(&"109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string());
    let io_handle = vm.io();
    vm.run();
    assert_eq!([109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], &io_handle.dump()[..]);

    vm.load_program(&"1102,34915192,34915192,7,4,7,99,0".to_string());
    vm.run();
    assert_eq!(16, count_digits(io_handle.recv().unwrap()));

    vm.load_program(&"104,1125899906842624,99".to_string());
    vm.run();
    assert_eq!(1125899906842624, io_handle.recv().unwrap());
}

#[test]
fn part_two_examples() {
    // TODO
}

/// Count the number of digits in a number.
#[cfg(test)]
fn count_digits(n: i64) -> usize {
    let mut n = n.abs();
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n = n / 10;
    }
    digits
}