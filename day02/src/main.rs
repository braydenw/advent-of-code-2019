use common::*;

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&input, part_one, part_two);
}

/// Parse the `input.txt` and convert to a `Vec<u32>`,
/// then process it using the simple Intcode machine.
fn part_one(input: &String) {
    let mut program = parse_program(input.trim());
    program[1] = 12;
    program[2] = 2;
    run_program(&mut program);

    println!("[Part 1] Value at position 0 after halting: {}", program[0]);
}

/// Same as Part 1, but brute-forces the input *noun* and *verb*
/// to get the desired value at memory address 0.
/// The method is inefficient, but reasonably quick over a small
/// solution space such as in this problem.
fn part_two(input: &String) {
    let mut program = parse_program(input.trim());
    'outer: for a in 0..100 {
        for b in 0..100 {
            program[1] = a;
            program[2] = b;
            run_program(&mut program);

            if program[0] == 19690720 {
                break 'outer;
            }

            program = parse_program(input.trim());
        }
    }

    println!("[Part 2] Noun is {} and verb is {}.", program[1], program[2]);
}

/// The Intcode machine.
/// A nicer looking version of this function can be found in the
/// `shared` library within this workspace.
fn run_program(program: &mut Vec<u32>) {
    let mut pointer = 0;
    
    'program: loop {
        if pointer + 3 >= program.len() {
            break 'program;
        }

        let a = program[pointer + 1] as usize;
        let b = program[pointer + 2] as usize;
        let c = program[pointer + 3] as usize;

        match program[pointer] {
            1 => {
                program[c] = program[a] + program[b];
            },
            2 => {
                program[c] = program[a] * program[b];
            },
            _ => break 'program
        }

        pointer += 4;
    }
}

/// Turns a comma seperated string into a vector of `u32`s.
/// Assumes the input is valid.
fn parse_program<S: Into<String>>(input: S) -> Vec<u32> {
    input.into()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[test]
fn intcode_examples() {
    let mut program = parse_program("1,0,0,0,99");
    run_program(&mut program);
    assert_eq!(parse_program("2,0,0,0,99"), program);

    program = parse_program("2,3,0,3,99");
    run_program(&mut program);
    assert_eq!(parse_program("2,3,0,6,99"), program);

    program = parse_program("2,4,4,5,99,0");
    run_program(&mut program);
    assert_eq!(parse_program("2,4,4,5,99,9801"), program);

    program = parse_program("1,1,1,4,99,5,6,0,99");
    run_program(&mut program);
    assert_eq!(parse_program("30,1,1,4,2,5,6,0,99"), program);
}