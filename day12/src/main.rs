use common::*;

type Moon = ([i64; 3], [i64; 3]);

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&input, part_one, part_two);
}

fn part_one(input: &String) {
    let moons = parse_moons(&input);

    println!("[Part 1] Total energy in the system after 1000 steps: {}",
        total_energy(&moons, 1000));
}

fn part_two(input: &String) {
    let mut moons = parse_moons(&input);

    println!("[Part 2] Necessary steps to reach a cycle: {:?}",
        find_cycle(&mut moons));
}

fn parse_moons(input: &String) -> Vec<Moon> {
    let mut moons = Vec::with_capacity(4);

    for line in input.lines() {
        let pos: Vec<i64> = line.split(',')
            .map(|v| v.parse::<i64>().unwrap())
            .collect();

        let moon_pos = [pos[0], pos[1], pos[2]];
        moons.push((moon_pos, [0; 3]));
    }

    moons
}

/// Perform a time step over a given dimension.
fn step(moons: &mut Vec<Moon>, d: usize) {
    let mut new_moons = [([0; 3], [0; 3]); 4];
    for m1 in 0..4 {
        let mut m1_pos = moons[m1].0;
        let mut m1_vel = moons[m1].1;

        for m2 in 0..4 {
            if m1 == m2 {
                continue;
            }

            let m2_pos = moons[m2].0;

            // Gravity
            m1_vel[d] += (m2_pos[d] - m1_pos[d]).signum();
        }
        
        // Velocity
        m1_pos[d] += m1_vel[d];

        // Update
        new_moons[m1] = (m1_pos, m1_vel);
    }

    moons.swap_with_slice(&mut new_moons);
}

fn total_energy(moons: &Vec<Moon>, steps: usize) -> i64 {
    let mut moons = moons.clone();

    let mut step_energy = 0;
    for _step in 0..steps {
        step(&mut moons, 0);
        step(&mut moons, 1);
        step(&mut moons, 2);

        // println!("After {} steps:", _step + 1);

        step_energy = 0;
        for m in 0..4 {
            let pot: i64 = moons[m].0.iter().map(|d| d.abs()).sum();
            let kin: i64 = moons[m].1.iter().map(|d| d.abs()).sum();

            // println!("pos=<x={:>3}, y={:>3}, z={:>3}>, vel=<x={:>3}, y={:>3}, z={:>3}>",
            //     moons[m].0[0], moons[m].0[1], moons[m].0[2],
            //     moons[m].1[0], moons[m].1[1], moons[m].1[2]);

            step_energy += pot * kin;
        }

        // println!("Total energy in the system: {}", step_energy);

        // println!();
    }

    step_energy
}

/// Converts 4 moons into an array that makes it easier to compare
/// state on a per dimension basis.
fn get_state(moons: &Vec<Moon>, d: usize) -> [i64; 8] {
    [
        moons[0].0[d], moons[0].1[d], moons[1].0[d], moons[1].1[d],
        moons[2].0[d], moons[2].1[d], moons[3].0[d], moons[3].1[d],
    ]
}

/// Calculate the Greatest Common Divisor of two given numbers using Euclid's Algorithm.
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Calculate the Least Common Multiple of two numbers using the Greatest Common Divisor.
fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn find_cycle(moons: &mut Vec<Moon>) -> i64 {
    let initial_state = [get_state(&moons, 0), get_state(&moons, 1), get_state(&moons, 2)];
    let mut cycles = [0; 3];
    for steps in 1.. {
        if cycles[0] != 0 && cycles[1] != 0 && cycles[2] != 0 {
            break;
        }

        for d in 0..3 {
            step(moons, d);

            if cycles[d] == 0 && get_state(&moons, d) == initial_state[d] {
                cycles[d] = steps;
            }
        }
    }

    lcm(cycles[0], lcm(cycles[1], cycles[2]))
}

#[test]
fn part_one_examples() {
    let mut moons = vec![
        ([-1, 0, 2], [0; 3]), ([2, -10, -7], [0; 3]),
        ([4, -8, 8], [0; 3]), ([3, 5, -1], [0; 3]),
    ];
    assert_eq!(179, total_energy(&moons, 10));

    moons = vec![
        ([-8, -10, 0], [0; 3]), ([5, 5, 10], [0; 3]),
        ([2, -7, 3], [0; 3]), ([9, -8, -3], [0; 3]),
    ];
    assert_eq!(1940, total_energy(&moons, 100));
}

#[test]
fn part_two_examples() {
    let mut moons = vec![
        ([-1, 0, 2], [0; 3]), ([2, -10, -7], [0; 3]),
        ([4, -8, 8], [0; 3]), ([3, 5, -1], [0; 3]),
    ];
    assert_eq!(2772, find_cycle(&mut moons));

    moons = vec![
        ([-8, -10, 0], [0; 3]), ([5, 5, 10], [0; 3]),
        ([2, -7, 3], [0; 3]), ([9, -8, -3], [0; 3]),
    ];
    assert_eq!(4686774924, find_cycle(&mut moons));
}