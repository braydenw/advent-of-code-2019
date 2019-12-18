use common::*;

use std::thread;
use std::collections::HashMap;

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&input, part_one, part_two);
}

fn part_one(input: &String) {
    let mut vm = IntcodeVM::new()
        .with_logging(1)
        .with_program(&input);
    
    let game_io = vm.io();
    let game_messenger = vm.messenger();
    let game_system = thread::spawn(move || vm.run());
    // let mut tiles = [0; 4096]; // width * y + x
    let mut tiles: HashMap<(i64, i64), i64> = HashMap::new();
    while let None = game_messenger.recv() {
        if let Some(x) = game_io.wait_recv() {
            if let Some(y) = game_io.wait_recv() {
                if let Some(tile) = game_io.wait_recv() {
                    tiles.insert((x, y), tile);
                }
            }
        }
    }

    let _ = game_system.join();
    println!("[Part 1] Total blocks: {:?}", tiles.values().filter(|&t| *t == 2).count());
}

fn part_two(input: &String) {
    let mut vm = IntcodeVM::new()
        .with_logging(1)
        .with_program(&input.replacen("1", "2", 1));
    
    let game_io = vm.io();
    let game_messenger = vm.messenger();
    let game_system = thread::spawn(move || {
        loop {
            match vm.step() {
                Some(IntcodeMessage::HaltTerminate) => {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    break;
                },
                Some(IntcodeMessage::HaltNeedInput) => continue,
                None => {},//std::thread::sleep(std::time::Duration::from_micros(500)),
            }
        }
        // vm.run()
    });
    let mut tiles = [0; 3600]; // width * y + x
    // let mut tiles: HashMap<(i64, i64), i64> = HashMap::new();
    let mut ball_x: i64 = 0;
    let mut paddle_x: i64 = 0;
    let mut score: i64 = 0;
    'game: loop {
        if let Some(IntcodeMessage::HaltTerminate) = game_messenger.recv() {
            break 'game;
        }

        while game_io.count_output() >= 3 {
            if let Some(x) = game_io.recv() {
                if let Some(y) = game_io.recv() {
                    if let Some(tile) = game_io.recv() {
                        match (x, y) {
                            (-1, 0) => score = tile,
                            _______ => tiles[(45 * y + x) as usize] = tile,
                        }

                        if tile == 3 {
                            paddle_x = x;
                        } else if tile == 4 {
                            ball_x = x;
                        }
                    }
                }
            }

        }
        let movement = (ball_x - paddle_x).signum();
        game_io.send(movement);

        let mut print_buffer = String::new();
        for y in 0..20 {
            print_buffer.push_str("\n");
            for x in 0..45 {
                match tiles[45 * y + x] {
                    0 => print_buffer.push_str(" "),
                    1 => print_buffer.push_str("#"),
                    2 => print_buffer.push_str("="),
                    3 => print_buffer.push_str("_"),
                    4 => print_buffer.push_str("o"),
                    _ => break,
                }
            }
        }
        print_buffer.push_str(format!("\n{:^45}\n", score).as_str());
        
        print!("{}\r", print_buffer);
    }

    let _ = game_system.join();
    println!("[Part 2] Score: {:?}", score);
}

#[test]
fn part_one_examples() {
    // TODO
}

#[test]
fn part_two_examples() {
    // TODO
}