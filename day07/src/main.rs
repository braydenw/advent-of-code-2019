use common::*;

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&input, part_one, part_two);
}

fn part_one(program: &String) {
    let mut max_signal = 0;
    let mut amplifiers = create_amplifiers(5, &program);

    for p in permutations([0, 1, 2, 3, 4]) {
        let signal = process_sequence(&mut amplifiers, &p);
        max_signal = max_signal.max(signal); // Hah
        reset_amplifiers(&mut amplifiers, &program);
    }
    
    println!("[Part 1] Maximum signal: {}", max_signal);
}

fn part_two(program: &String) {
    let mut max_signal = 0;
    let mut amplifiers = create_amplifiers(5, &program);

    for p in permutations([5, 6, 7, 8, 9]) {
        let signal = process_sequence_loop(&mut amplifiers, &p);
        max_signal = max_signal.max(signal);
        reset_amplifiers(&mut amplifiers, &program);
    }
    
    println!("[Part 2] Maximum signal: {}", max_signal);
}

/// Runs a sequence of integers (the `phase_settings`)
/// through a number of amplifiers, returning the final signal.
fn process_sequence(amplifiers: &mut Vec<IntcodeVM>, phase_settings: &[i64; 5]) -> i64 {
    let mut output = 0;

    for i in 0..5 {
        let io_handle = amplifiers[i].io();
        let messenger = amplifiers[i].messenger();

        // First input
        io_handle.send(phase_settings[i]);

        // Second input
        io_handle.send(output);

        // Run the program until halting or it needs input.
        while let None = messenger.recv() {
            amplifiers[i].step();
        }

        // Store the last output.
        output = io_handle.recv().unwrap();
    }

    output
}

/// Same as `process_sequence`, but wraps the amplifiers into a feedback loop.
/// How it works:
/// - Runs the first machine until it halts or requires input.
/// - Pushes the most recent output to the machine's input queue.
/// - Runs the next machine until it halts or requires input.
/// - ...
/// - Loops back to and runs the first machine until it halts or requires input.
/// - ...
/// - Stops and returns the final output once all machines have halted.
fn process_sequence_loop(amplifiers: &mut Vec<IntcodeVM>, phase_settings: &[i64; 5]) -> i64 {
    let mut output = process_sequence(amplifiers, phase_settings);

    let mut halted = [false; 5];
    'cycle: for i in (0..5).cycle() {
        let io_handle = amplifiers[i].io();
        let messenger = amplifiers[i].messenger();

        io_handle.send(output);

        'processor: loop {
            match messenger.recv() {
                Some(IntcodeMessage::HaltTerminate) => {
                    halted[i] = true;
                    break 'processor;
                },
                Some(IntcodeMessage::HaltNeedInput) => {
                    break 'processor;
                },
                None => {}
            }

            amplifiers[i].step();
        }

        output = io_handle.recv().unwrap();

        if halted.iter().fold(true, |acc, b| acc & b) {
            break 'cycle;
        }
    }

    output
}

/// Calculate permutations of numbers from the 5 given.
/// Adapted from: https://rosettacode.org/wiki/Permutations#Rust
fn permutations(set: [i64; 5]) -> Vec<[i64; 5]> {
    let mut perms: Vec<[i64; 5]> = Vec::new();

    let mut ints: [i64; 5] = set;
    let mut swaps: [i64; 5] = [0; 5];
    let mut current = 0;
    'outer: loop {
        if current > 0 {
            'inner: loop {
                if current >= swaps.len() {
                    break 'outer;
                }
                if swaps[current] < current as i64 {
                    break 'inner;
                }

                swaps[current] = 0;
                current += 1;
            }
            
            ints.swap(current, (current & 1) * swaps[current] as usize);
            swaps[current] += 1;
        }
        
        current = 1;

        let perm = [ints[0], ints[1], ints[2], ints[3], ints[4]];
        if let Err(i) = perms.binary_search(&perm) {
            perms.insert(i, perm);
        }
    }
    
    perms
}

/// Creates and returns a number of `IntcodeVM`s.
fn create_amplifiers(amount: usize, program: &String) -> Vec<IntcodeVM> {
    let mut amplifiers = Vec::with_capacity(amount);
    for _ in 0..5 {
        let vm = IntcodeVM::new()
            .with_logging(0)
            .with_program(&program);
        amplifiers.push(vm);
    }

    amplifiers
}

/// Resets a number of `IntcodeVM`s to a given program.
fn reset_amplifiers(amplifiers: &mut Vec<IntcodeVM>, program: &String) {
    for vm in amplifiers {
        vm.reset();
        vm.load_program(&program);
    }
}

#[test]
fn permutations_test() {
    assert_eq!(120, permutations([0, 1, 2, 3, 4]).len());
    assert_eq!(120, permutations([5, 6, 7, 8, 9]).len());
}

#[test]
fn part_one_examples() {
    let mut program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let mut amplifiers = create_amplifiers(5, &program.to_string());
    assert_eq!(43210, process_sequence(&mut amplifiers, &[4,3,2,1,0]));
    
    program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    reset_amplifiers(&mut amplifiers, &program.to_string());
    assert_eq!(54321, process_sequence(&mut amplifiers, &[0,1,2,3,4]));

    program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    reset_amplifiers(&mut amplifiers, &program.to_string());
    assert_eq!(65210, process_sequence(&mut amplifiers, &[1,0,4,3,2]));
}

#[test]
fn part_two_examples() {
    let mut program = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    let mut amplifiers = create_amplifiers(5, &program.to_string());
    assert_eq!(139629729, process_sequence_loop(&mut amplifiers, &[9,8,7,6,5]));

    program = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    reset_amplifiers(&mut amplifiers, &program.to_string());
    assert_eq!(18216, process_sequence_loop(&mut amplifiers, &[9,7,8,5,6]));
}