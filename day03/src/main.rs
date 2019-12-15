use common::*;

type Point = (i32, i32);
type Line = (Point, Point);

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&input, part_one, part_two);
}

fn part_one(input: &String) {
    let wires = input.lines()
        .filter(|l| l.len() > 0)
        .collect::<Vec<&str>>();
    
    let crosses = find_crosses(find_path(wires[0]), find_path(wires[1]))
        .iter()
        .map(|((a, b), _)| (*a, *b))
        .collect();
    let closest = nearest_cross_dist(crosses);
    
    println!("[Part 1] Distance from origin to closest intersection: {}", closest);
}

fn part_two(input: &String) {
    let wires = input.lines()
        .filter(|l| l.len() > 0)
        .collect::<Vec<&str>>();
    
    let crosses = find_crosses(find_path(wires[0]), find_path(wires[1]));
    let closest = nearest_cross_steps(crosses);
    
    println!("[Part 2] Fewest steps to reach an intersection: {}", closest);
}

/// Parse a `String` wire path into a `Vec` containing the start and end points
/// of each straight section of the wire as well as the steps required to traverse
/// the section.
fn find_path<S: Into<String>>(input: S) -> Vec<(Line, i32)> {
    let mut lines = Vec::new();

    let mut last_point = (0, 0);
    let mut total_distance = 0;
    for next in input.into().split(',') {
        let (direction, distance) = next.split_at(1);
        let distance = distance.trim().parse::<i32>().unwrap();

        let (last_x, last_y) = last_point;
        let point = match direction {
            "U" => (last_x, last_y + distance),
            "D" => (last_x, last_y - distance),
            "L" => (last_x - distance, last_y),
            "R" => (last_x + distance, last_y),
            _ => panic!("invalid direction")
        };

        total_distance += distance;
        lines.push((((last_x, last_y), point), total_distance));
        last_point = point;
    }

    lines
}

/// Find all the intersections from two vectors of lines.
fn find_crosses(a: Vec<(Line, i32)>, b: Vec<(Line, i32)>) -> Vec<(Point, i32)> {
    let mut intersections = Vec::new();

    for ia in 0..a.len() {
        let (a_line, a_steps) = a[ia];

        for ib in 0..b.len() {
            let (b_line, b_steps) = b[ib];

            if let Some(point) = intersect(a_line, b_line) {
                if point != (0, 0) {
                    let a_dist = a_steps - point_distance(point, a_line.1);
                    let b_dist = b_steps - point_distance(point, b_line.1);

                    intersections.push((point, a_dist + b_dist));
                }
            }
        }
    }

    intersections
}

/// Adapted from: https://stackoverflow.com/a/1968345
/// Finds an intersection, if any, between two lines.
fn intersect(s1: Line, s2: Line) -> Option<Point> {
    let (p1, p2) = s1;
    let (p3, p4) = s2;

    let s1 = (p2.0 - p1.0, p2.1 - p1.1);
    let s2 = (p4.0 - p3.0, p4.1 - p3.1);
    let det = -s2.0 * s1.1 + s1.0 * s2.1;

    if det == 0 {
        return None;
    }

    let s = (-s1.1 * (p1.0 - p3.0) + s1.0 * (p1.1 - p3.1)) as f32 / det as f32;
    let t = (s2.0 * (p1.1 - p3.1) - s2.1 * (p1.0 - p3.0)) as f32 / det as f32;

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        return Some((p1.0 + (t * s1.0 as f32) as i32, p1.1 + (t * s1.1 as f32) as i32));
    }

    None
}

/// Find the nearest intersection, based on distance alone.
fn nearest_cross_dist(crosses: Vec<Point>) -> i32 {
    crosses.iter()
        .map(|(a, b)| a.abs() + b.abs())
        .min()
        .unwrap()
}

/// Find the smallest number of steps to get to an intersection.
fn nearest_cross_steps(crosses: Vec<(Point, i32)>) -> i32 {
    crosses.iter()
        .map(|(_, s)| *s)
        .min()
        .unwrap()
}

/// Find the distance between two points.
fn point_distance(a: Point, b: Point) -> i32 {
    // Was mistakenly doing a more accurate calculation than necessary.
    // (((a.1 - a.0).pow(2) + (b.1 - b.0).pow(2)) as f32).sqrt() as i32
    if a.0 == b.0 {
        (a.1 - b.1).abs()
    } else {
        (a.0 - b.0).abs()
    }
}

#[test]
fn part_one_examples() {
    let mut wire1 = find_path("R8,U5,L5,D3");
    let mut wire2 = find_path("U7,R6,D4,L4");
    let mut crosses = find_crosses(wire1, wire2)
        .iter()
        .map(|((a, b), _)| (*a, *b))
        .collect();
    let mut closest_cross = nearest_cross_dist(crosses);
    assert_eq!(6, closest_cross);

    wire1 = find_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    wire2 = find_path("U62,R66,U55,R34,D71,R55,D58,R83");
    crosses = find_crosses(wire1, wire2)
        .iter()
        .map(|((a, b), _)| (*a, *b))
        .collect();
    closest_cross = nearest_cross_dist(crosses);
    assert_eq!(159, closest_cross);

    wire1 = find_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    wire2 = find_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    crosses = find_crosses(wire1, wire2)
        .iter()
        .map(|((a, b), _)| (*a, *b))
        .collect();
    closest_cross = nearest_cross_dist(crosses);
    assert_eq!(135, closest_cross);
}

#[test]
fn part_two_examples() {
    let mut wire1 = find_path("R8,U5,L5,D3");
    let mut wire2 = find_path("U7,R6,D4,L4");
    let mut crosses = find_crosses(wire1, wire2);
    let mut closest_cross = nearest_cross_steps(crosses);
    assert_eq!(30, closest_cross);
    
    wire1 = find_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    wire2 = find_path("U62,R66,U55,R34,D71,R55,D58,R83");
    crosses = find_crosses(wire1, wire2);
    closest_cross = nearest_cross_steps(crosses);
    assert_eq!(610, closest_cross);

    wire1 = find_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    wire2 = find_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    crosses = find_crosses(wire1, wire2);
    closest_cross = nearest_cross_steps(crosses);
    assert_eq!(410, closest_cross);
}