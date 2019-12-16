use common::*;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd)]
struct Float(i64);

impl From<&f64> for Float {
    fn from(other: &f64) -> Float {
        Float((other * 1000000000.0) as i64)
        // Float {
        //     i: other.trunc() as i64,
        //     f: other.fract() as u64,
        // }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i8,
    y: i8,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x: x as i8,
            y: y as i8,
        }
    }

    fn distance_to(&self, other: &Point) -> Float {
        let x = (other.x - self.x) as f64;
        let y = (other.y - self.y) as f64;

        Float::from(&(x.powi(2) + y.powi(2)).sqrt())
    }

    fn angle_to(&self, other: &Point) -> Float {
        let x = (other.x - self.x) as f64;
        let y = (other.y - self.y) as f64;

        Float::from(&y.atan2(x).to_degrees())
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(&input, part_one, part_two);
}

fn part_one(input: &String) {
    let asteroids = parse_asteroids(input);
    let best_point = find_best_loc(&asteroids);

    println!("[Part 1] Best location for a monitoring station is at {}, which detects {} other asteroids.",
        best_point.0, best_point.1);
}

fn part_two(input: &String) {
    // TODO
}

/// Parse an input map containing asteroids into a `Vec` of coordinates.
fn parse_asteroids(input: &String) -> Vec<Point> {
    let mut asteroids = Vec::with_capacity(input.len());

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                asteroids.push(Point::new(x, y));
            }
        }
    }

    asteroids
}

/// Finds the asteroid with the most visible other asteroids 
/// and returns a tuple with the point and number of other
/// visible asteroids.
fn find_best_loc(map: &Vec<Point>) -> (Point, usize) {let mut num_visible = Vec::with_capacity(map.len());
    for (i, aste) in map.iter().enumerate() {
        let mut visible: HashMap<Float, (Float, usize)> = HashMap::with_capacity(map.len());
        for (j, target) in map.iter().enumerate() {
            if i == j {
                continue;
            }

            let target = (aste.distance_to(target), aste.angle_to(target));
            if let Some(t) = visible.get(&Float::from(target.1)) {
                if target.0 < (*t).0 {
                    visible.insert(target.1, (target.0, j));
                }
            } else {
                visible.insert(target.1, (target.0, j));
            }

        }

        num_visible.push((visible.len(), aste));
        visible.clear();
    }

    let result = *num_visible.iter().max_by_key(|(d, _)| d).unwrap();
    (*result.1, result.0)
}

#[test]
fn parsing_test() {
    let asteroids = ".#..#\n.....\n#####\n....#\n...##".to_string();
    let expected = vec![
        Point::new(1, 0), Point::new(4, 0), Point::new(0, 2), Point::new(1, 2),
        Point::new(2, 2), Point::new(3, 2), Point::new(4, 2), Point::new(4, 3),
        Point::new(3, 4), Point::new(4, 4)];
    assert_eq!(expected, parse_asteroids(&asteroids));
}

#[test]
fn angle_offset_test() {
    let origin = Point::new(0, 0);
    let p1 = Point::new(12, 5);
    let p2 = Point::new(4, 7);

    let mut angle = origin.angle_to(&p1);
    assert_eq!(Float::from(&22.619864948040426), angle);

    angle = origin.angle_to(&p2);
    assert_eq!(Float::from(&60.25511870305778), angle);

    angle = p1.angle_to(&p2);
    assert_eq!(Float::from(&165.96375653207352), angle);
}

#[test]
fn part_one_examples() {
    let inputs = [
        &".#..#\n.....\n#####\n....#\n...##".to_string(),
        &"......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####".to_string(),
        &"#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.".to_string(),
        &".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..".to_string(),
        &".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##".to_string(),
    ];

    assert_eq!((Point::new(3, 4), 8), find_best_loc(&parse_asteroids(inputs[0])));
    assert_eq!((Point::new(5, 8), 33), find_best_loc(&parse_asteroids(inputs[1])));
    assert_eq!((Point::new(1, 2), 35), find_best_loc(&parse_asteroids(inputs[2])));
    assert_eq!((Point::new(6, 3), 41), find_best_loc(&parse_asteroids(inputs[3])));
    assert_eq!((Point::new(11, 13), 210), find_best_loc(&parse_asteroids(inputs[4])));
}

#[test]
fn part_two_examples() {
    // TODO
}