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
    let game_thread = thread::spawn(move || vm.run());
    let (width, height) = (44, 19);
    let mut tiles = [0; 881];
    let mut ball_x: i64 = 0;
    let mut paddle_x: i64 = 0;
    let mut score: i64 = 0;
    let mut game_running = true;
    while game_running {
        // Quite processing if the game has stopped running;
        // Send the paddle movement direction if input is needed.
        match game_messenger.recv() {
            Some(IntcodeMessage::HaltTerminate) => game_running = false,
            Some(IntcodeMessage::HaltNeedInput) => game_io.send((ball_x - paddle_x).signum()),
            None => {}
        }
        
        // Print the game.
        let mut print_buffer = "\n".repeat(30);
        for y in 0..height {
            print_buffer.push_str("\n");
            for x in 0..width {
                match tiles[(width * y + x) as usize] {
                    0 => print_buffer.push_str(" "),
                    1 => print_buffer.push_str("#"),
                    2 => print_buffer.push_str("="),
                    3 => print_buffer.push_str("_"),
                    4 => print_buffer.push_str("o"),
                    _ => break,
                }
            }
        }
        print_buffer.push_str(format!("\n{:^45}", score).as_str());
        print_buffer.push_str(format!("\n{:^45}\n", "Score").as_str());
        
        print!("{}\r", print_buffer);

        // Update game information.
        while game_io.count_output() >= 3 {
            if let Some(x) = game_io.recv() {
                if let Some(y) = game_io.recv() {
                    if let Some(tile) = game_io.recv() {
                        match (x, y) {
                            (-1, 0) => score = tile,
                            _______ => tiles[(width * y + x) as usize] = tile,
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
    }

    let _ = game_thread.join();
}