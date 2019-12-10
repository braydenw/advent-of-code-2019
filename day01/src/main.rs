//! For now, both parts of a problem pair are in their own function.
//! As days progress, I'll work on reducing code duplication.
//! I kind of like the `part_selector` function right now though.

use shared::*;

fn main() {
    let mut buffer = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&mut buffer, part_one, part_two);
}

fn part_one(buffer: &mut Buffer) {
    let mut sum = 0;
    for line in buffer.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();

        sum += get_fuel(mass);
    }

    println!("[Part 1] Sum of fuel requirements: {}", sum);
}

fn part_two(buffer: &mut Buffer) {
    let mut sum = 0;
    for line in buffer.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();

        sum += get_fuel_recursive(mass);
    }

    println!("[Part 2] Sum of fuel requirements: {}", sum);
}

/// Use the formula provided to calculate the fuel needed for a given mass.
/// Integer division truncates, i.e. rounds down.
fn get_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

/// Calculate the fuel needed for a given mass plus its fuel (until no more fuel needed).
/// As the function name suggests, recursion could be used. However, simple iteration works too.
fn get_fuel_recursive(mass: i32) -> i32 {
    let mut fuel = get_fuel(mass);

    let mut sum = 0;
    while fuel > 0 {
        sum += fuel;
        fuel = get_fuel(fuel);
    }

    sum
}

#[test]
fn part_one_examples() {
    assert_eq!(2, get_fuel(12));
    assert_eq!(2, get_fuel(14));
    assert_eq!(654, get_fuel(1969));
    assert_eq!(33583, get_fuel(100756));
}

#[test]
fn part_two_examples() {
    assert_eq!(2, get_fuel_recursive(14));
    assert_eq!(966, get_fuel_recursive(1969));
    assert_eq!(50346, get_fuel_recursive(100756));
}