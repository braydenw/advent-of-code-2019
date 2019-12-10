pub use std::io::{BufReader, BufRead, Seek};
pub use std::fs::File;

use std::path::Path;

pub type Buffer = BufReader<File>;

/// Setup a BufReader on the provided `input.txt`.
pub fn get_input(p: &'static str) -> Buffer {
    let file = File::open(Path::new(p).join("input.txt"))
        .expect("bad path");
    
    BufReader::new(file)
}

/// Runs a specified part.
pub fn part_selector(reader: &mut Buffer, a: fn(&mut Buffer), b: fn(&mut Buffer)) {
    use std::io::SeekFrom;

    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && args[1].as_str() == "1" {
        a(reader);
    } else if args.len() == 2 && args[1].as_str() == "2" {
        b(reader);
    } else {
        a(reader);
        reader.seek(SeekFrom::Start(0))
            .expect("failed to reset BufReader");
        b(reader);
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