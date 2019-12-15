use common::*;

use std::collections::HashMap;

type Objects = HashMap<usize, Object>;

#[derive(Debug, PartialEq)]
struct Object {
    parent: Option<usize>,
    children: Vec<usize>,
}

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&input, part_one, part_two);
}

fn part_one(input: &String) {
    let orbits = parse_orbits(input);
    let total = total_orbits(&orbits);

    println!("[Part 1] Total number of direct and indirect orbits: {}", total);
}

fn part_two(input: &String) {
    let orbits = parse_orbits(input);
    let total = min_transfers(&orbits);

    println!("[Part 2] Total number of orbital transfers required: {}", total);
}

/// Converts an object string into a `usize`.
fn parse_id(s: &str) -> usize {
    let bytes: Vec<u8> = s.bytes().collect();
    let mut id = 0;
    for i in 0..bytes.len() {
        id |= (bytes[i] as usize) << (i * 8)
    }

    id
}

/// Parses all the orbits into an easier to traverse format.
fn parse_orbits(input: &String) -> Objects {
    let mut universe = HashMap::with_capacity(2048);

    for line in input.lines() {
        let mut split = line.trim().split(')');
        let pin = parse_id(split.next().unwrap());
        let obj = parse_id(split.next().unwrap());

        universe
            .entry(pin)
            .or_insert(Object {
                parent: None, 
                children: Vec::with_capacity(1024)
            })
            .children
            .push(obj);
        
        universe
            .entry(obj)
            .and_modify(|o| {
                if let None = o.parent {
                    o.parent = Some(pin);
                }
            })
            .or_insert(Object {
                parent: Some(pin),
                children: Vec::with_capacity(1024),
            });
    }

    universe
}

/// Finds the total number of direct and indirect orbits.
fn total_orbits(orbits: &Objects) -> usize {
    let mut total = 0;

    // Iterative depth-first traversal
    let mut stack = Vec::with_capacity(256);
    stack.push((parse_id("COM"), 0));
    'dfs: loop {
        if let Some((current, inc)) = stack.pop() {
            total += inc;
            for child in &orbits[&current].children {
                stack.push((*child, inc + 1));
            }
        } else {
            break 'dfs;
        }
    }

    total
}

/// Finds the minimum number of orbital transfers to get from YOU to SANta.
fn min_transfers(orbits: &Objects) -> usize {
    let mut transfers = 0;

    // A rather specialized version of what's used in `total_orbits`.
    let mut visited = Vec::with_capacity(256);
    let mut stack = Vec::with_capacity(256);
    stack.push((parse_id("YOU"), 0));
    loop {
        if let Some((current, dist)) = stack.pop() {
            if current == parse_id("SAN") {
                transfers = dist;
                break;
            }
            
            // Add parent to the stack, if a parent exists and it has not yet been visited.
            if let Some(parent) = &orbits[&current].parent {
                if !visited.contains(parent) {
                    stack.push((*parent, dist + 1));
                }
            }
            
            // Add all children to teh stack, if not already visited.
            for child in &orbits[&current].children {
                if !visited.contains(child) {
                    stack.push((*child, dist + 1));
                }
            }

            visited.push(current);
        } else {
            break;
        }
    }

    transfers - 2
}

/// Turns an object id back into a `String`.
#[cfg(test)]
fn id_as_str(id: usize) -> String {
    let bytes: Vec<u8> = id.to_le_bytes().iter()
        .filter(|b| **b != 0).map(|b| *b).collect();
    String::from_utf8_lossy(&bytes[..]).to_string()
}

#[test]
fn parse_id_tests() {
    assert_eq!(0x41, parse_id("A"));
    assert_eq!(0x4142, parse_id("BA"));
    assert_eq!(0x414243, parse_id("CBA"));
}

#[test]
fn id_to_str_tests() {
    assert_eq!(String::from("A"), id_as_str(0x41));
    assert_eq!(String::from("BA"), id_as_str(0x4142));
    assert_eq!(String::from("CBA"), id_as_str(0x414243));
}

#[test]
fn part_one_examples() {
    let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    assert_eq!(42, total_orbits(&parse_orbits(&input.to_string())));
}

#[test]
fn part_two_examples() {
    let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
    assert_eq!(4, min_transfers(&parse_orbits(&input.to_string())));
}