use common::*;

use std::collections::HashMap;
use std::thread;

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&input, part_one, part_two);
}

fn part_one(input: &String) {
    let panels = paint_panels(&input, 0);

    println!("[Part 1] Number of painted panels: {:?}.", panels.len());
}

fn part_two(input: &String) {
    let panels = paint_panels(&input, 1);

    // These were found with `panels.keys().min_by_key(..)`
    // and `panels.keys().max_by_key(..)`.
    let (min_x, min_y) = (0, -5);
    let (max_x, max_y) = (42, 0);

    println!("[Part 2]");

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let color = panels.get(&(x, y)).unwrap_or(&0);
            if *color == 0 {
                print!(" ");
            } else {
                print!("█");
            }
        }
        print!("\n");
    }
}

fn paint_panels(input: &String, start_color: i64) -> HashMap<(i64, i64), i64> {
    let mut vm = IntcodeVM::new()
        .with_logging(1)
        .with_program(&input);
    
    let io_handle = vm.io();
    let messenger = vm.messenger();
    let robot = thread::spawn(move || vm.run());

    let mut panels: HashMap<(i64, i64), i64> = HashMap::with_capacity(4096);
    let mut panel = (0, 0);
    let mut face = (0, 1);
    panels.insert(panel, start_color);
    'processor: loop {
        // Color of current panel.
        let color = *panels.get(&panel).unwrap_or(&0);
        
        // Notify the robot.
        io_handle.send(color);

        // Wait for color of paint from robot.
        if let Some(paint) = io_handle.wait_recv() {

            // Wait for turn direction from robot.
            if let Some(turn) = io_handle.wait_recv() {
                panels.insert(panel, paint);

                // Turn
                if turn == 0 {
                    face = match face {
                        ( 0, 1) => (-1, 0),
                        (-1, 0) => ( 0,-1),
                        ( 0,-1) => ( 1, 0),
                        ( 1, 0) => ( 0, 1),
                        _______ => panic!("invalid left state")
                    };
                } else if turn == 1 {
                    face = match face {
                        ( 0, 1) => ( 1, 0),
                        (-1, 0) => ( 0, 1),
                        ( 0,-1) => (-1, 0),
                        ( 1, 0) => ( 0,-1),
                        _______ => panic!("invalid right state")
                    };
                }

                // Move
                panel = (panel.0 + face.0, panel.1 + face.1);
            }
        }

        // Stop the processor if the robot has stopped.
        if let Some(IntcodeMessage::HaltTerminate) = messenger.recv() {
            break 'processor;
        }
    }

    let _ = robot.join();

    panels
}