pub use std::io::Read;
pub use std::fs::File;

use std::path::Path;

/// Setup a BufReader on the provided `input.txt`.
pub fn get_input(p: &'static str) -> String {
    let mut input = String::new();
    let mut file = File::open(Path::new(p).join("input.txt"))
        .expect("bad path");
    
    file.read_to_string(&mut input)
        .expect("failed to read input to String");
    
    input
}

/// Runs a specified part.
pub fn part_selector(input: String, a: fn(String), b: fn(String)) {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && args[1].as_str() == "1" {
        a(input);
    } else if args.len() == 2 && args[1].as_str() == "2" {
        b(input);
    } else {
        a(input.clone());
        b(input);
    }
}

/// Simple Intcode machine.
/// Takes some memory and processes it.
fn run_intcode(memory: &mut Vec<u32>) {
    let mut pointer = 0;
    
    'processor: loop {
        if pointer + 3 >= memory.len() {
            break 'processor;
        }

        let instr = memory[pointer];
        let param_1 = memory[pointer + 1] as usize;
        let param_2 = memory[pointer + 2] as usize;
        let param_3 = memory[pointer + 3] as usize;

        match instr {
            1 => {
                memory[param_3] = memory[param_1] + memory[param_2];
            },
            2 => {
                memory[param_3] = memory[param_1] * memory[param_2];
            },
            99 => break 'processor,
            _ => break 'processor
        }

        pointer += 4;
    }
}