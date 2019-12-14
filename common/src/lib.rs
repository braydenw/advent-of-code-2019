mod intcode;
pub use intcode::{IntcodeVM, State as IntcodeState};

pub use std::io::Read;
pub use std::fs::File;

use std::path::Path;
use std::time::Instant;

/// Setup a BufReader on the provided `input.txt`.
pub fn get_input(p: &'static str) -> String {
    let mut input = String::new();
    let mut file = File::open(Path::new(p).join("input.txt"))
        .expect("bad path");
    
    file.read_to_string(&mut input)
        .expect("failed to read input to String");
    
    input
}

/// Runs a specified part, or both.
pub fn part_selector(input: String, a: fn(String), b: fn(String)) {
    let args: Vec<String> = std::env::args().collect();

    let start = Instant::now();
    if args.len() == 2 && args[1].as_str() == "1" {
        a(input);
    } else if args.len() == 2 && args[1].as_str() == "2" {
        b(input);
    } else {
        a(input.clone());
        b(input);
    }

    let time = Instant::now().duration_since(start);
    println!("Time: {}ms ({}us)", time.as_millis(), time.as_micros());
}

/// Read a line from stdin.
pub fn read_stdin() -> String {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");

    line.trim().to_string()
}