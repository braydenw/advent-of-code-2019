use common::*;

use std::collections::HashMap;

/// Break apart an `f64` into a custom `Float` structure.
/// This eases comparisons a little and allows for precision configuration.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Float(i64, u64);

impl From<&f64> for Float {
    fn from(other: &f64) -> Float {
        Float(other.trunc() as i64, (other.fract() * 1e12) as u64)
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

/// A point in space.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    /// Create a `Point` from given x and y coordinates.
    fn new(x: usize, y: usize) -> Point {
        Point {
            x: x as i16,
            y: y as i16,
        }
    }

    /// Compute the distance between `self` and `other`.
    fn distance_to(&self, other: &Point) -> Float {
        let x = (other.x - self.x) as f64;
        let y = (other.y - self.y) as f64;

        Float::from(&(x.powi(2) + y.powi(2)).sqrt())
    }

    /// Compute the angle of `other` in relation to `self`.
    /// Up is 0deg, right is 90deg, down is 180deg, and left is 270deg.
    fn angle_to(&self, other: &Point) -> Float {
        let x = (other.x - self.x) as f64;
        let y = (other.y - self.y) as f64;
        let mut deg = y.atan2(x).to_degrees();

        if deg < 0.0 {
            deg += 360.0;
        }

        Float::from(&((deg + 90.0) % 360.0))
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

/// This part could probably be adapted to use `find_visible` from part 2, 
/// but it seems to actually be very slightly slower when doing so.
fn part_one(input: &String) {
    let asteroids = parse_input(input).1;
    let best_point = find_best_loc(&asteroids);

    println!("[Part 1] Best location for a monitoring station is at {}, which detects {} other asteroids.",
        best_point.0, best_point.1);
}

/// Quicker than part 1 since it doesn't have to iterate every asteroid 
/// point and find all its visible asteroids.
fn part_two(input: &String) {
    let (station, asteroids) = parse_input(input);
    
    let station = match station {
        Some(s) => s,
        None => Point::new(11, 13),
    };
    let vaporized = vaporize(station, &asteroids);
    
    println!("[Part 2] 200th vaporized asteroid was at {} ({}).",
        vaporized[199], vaporized[199].x * 100 + vaporized[199].y);
}

/// Parse an input map containing asteroids into a `Vec` of coordinates.
fn parse_input(input: &String) -> (Option<Point>, Vec<Point>) {
    let mut asteroids = Vec::with_capacity(input.len());

    let mut station = None;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => asteroids.push(Point::new(x, y)),
                'X' => station = Some(Point::new(x, y)),
                ___ => {}
            }
        }
    }

    (station, asteroids)
}

/// Find all points with asteroids visible from a given origin.
fn find_visible(origin: Point, targets: &Vec<Point>) -> Vec<Point> {
    let mut visible: HashMap<Float, (Float, Float, Point)> = HashMap::with_capacity(targets.len());
    for target in targets.iter() {
        if origin == *target {
            continue;
        }

        let dist = origin.distance_to(target);
        let angl = origin.angle_to(target);
        if let Some(t) = visible.get(&Float::from(angl)) {
            if dist < (*t).0 {
                visible.insert(angl, (dist, angl, *target));
            }
        } else {
            visible.insert(angl, (dist, angl, *target));
        }
    }

    let mut points: Vec<&(Float, Float, Point)> = visible.values().collect();
    points.sort_by_key(|(_, a, _)| a);

    points.iter().map(|(_, _, p)| *p).collect()
}

/// Finds the asteroid with the most visible other asteroids.
/// Returns a tuple with the point and number of other
/// visible asteroids.
fn find_best_loc(map: &Vec<Point>) -> (Point, usize) {
    let mut num_visible = Vec::with_capacity(map.len());
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

/// Vaporize asteroids from the given field, shooting from the specified station.
/// Returns an ordered `Vec` of all the vaporized asteroids.
fn vaporize(station: Point, asteroids: &Vec<Point>) -> Vec<Point> {
    let mut vaporized = Vec::with_capacity(asteroids.len());
    let mut asteroids = asteroids.clone();
    asteroids.sort_by_key(|p| p.x);

    if let Ok(i) = asteroids.binary_search(&station) {
        asteroids.remove(i);
    }

    while asteroids.len() > 0 {
        let targets = find_visible(station, &asteroids);

        for target in targets {
            if target == station {
                continue;
            }

            if let Ok(i) = asteroids.binary_search(&target) {
                vaporized.push(asteroids.remove(i));
            }
        }
    }

    vaporized
}

#[test]
fn parsing_test() {
    let asteroids = ".#..#\n.....\n#####\n....#\n...##".to_string();
    let expected = vec![
        Point::new(1, 0), Point::new(4, 0), Point::new(0, 2), Point::new(1, 2),
        Point::new(2, 2), Point::new(3, 2), Point::new(4, 2), Point::new(4, 3),
        Point::new(3, 4), Point::new(4, 4)];
    assert_eq!(expected, parse_input(&asteroids).1);
}

#[test]
fn angle_offset_test() {
    let origin = Point::new(0, 0);
    let p1 = Point::new(12, 5);
    let p2 = Point::new(4, 7);

    // These are after adjusting the axes for Part 2.
    let mut angle = origin.angle_to(&p1);
    assert_eq!(Float::from(&112.619864948040426), angle);

    angle = origin.angle_to(&p2);
    assert_eq!(Float::from(&150.25511870305778), angle);

    angle = p1.angle_to(&p2);
    assert_eq!(Float::from(&255.96375653207352), angle);
}

#[test]
fn part_one_examples() {
    let inputs = [
        parse_input(&".#..#\n.....\n#####\n....#\n...##".to_string()).1,
        parse_input(&"......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####".to_string()).1,
        parse_input(&"#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.".to_string()).1,
        parse_input(&".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..".to_string()).1,
        parse_input(&".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##".to_string()).1,
    ];

    assert_eq!((Point::new(3, 4), 8), find_best_loc(&inputs[0]));
    assert_eq!((Point::new(5, 8), 33), find_best_loc(&inputs[1]));
    assert_eq!((Point::new(1, 2), 35), find_best_loc(&inputs[2]));
    assert_eq!((Point::new(6, 3), 41), find_best_loc(&inputs[3]));
    assert_eq!((Point::new(11, 13), 210), find_best_loc(&inputs[4]));
}

#[test]
fn part_two_examples() {
    let input = parse_input(&".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##".to_string()).1;
    let station = Point::new(11, 13);
    let vapor = vaporize(station, &input);

    assert_eq!(vapor[0], Point::new(11, 12));
    assert_eq!(vapor[1], Point::new(12, 1));
    assert_eq!(vapor[2], Point::new(12, 2));
    assert_eq!(vapor[9], Point::new(12, 8));
    assert_eq!(vapor[19], Point::new(16, 0));
    assert_eq!(vapor[49], Point::new(16, 9));
    assert_eq!(vapor[99], Point::new(10, 16));
    assert_eq!(vapor[198], Point::new(9, 6));
    assert_eq!(vapor[199], Point::new(8, 2));
    assert_eq!(vapor[200], Point::new(10, 9));
    assert_eq!(vapor[298], Point::new(11, 1));
}