use common::*;

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(input, part_one, part_two);
}

fn part_one(input: String) {
    let layers = create_layers(&input, 25, 6);

    // Uses iterators, but filters and counts each layer twice.
    let layer: (usize, usize) = layers.iter()
        .map(|l| {
            (l.iter().filter(|n| **n == 0).count(),
             l.iter().filter(|n| **n == 1).count())
        })
        .min_by_key(|k| k.0).unwrap();
    let ones = layer.1;
    let twos = 150 - ones - layer.0;

    println!("[Part 1] Number of 1 digits * number of 2 digits: {:?}", ones * twos);
}

fn part_two(input: String) {
    let (w, h) = (25, 6);
    let layers = create_layers(&input, w, h);
    let image = flatten_layers(&layers, w, h);

    println!("[Part 2]");
    draw_image(&image, w, h);
}

/// Turns `data` into a `Vec` of `Vec<u32>`, or a `Vec` of flattened layers.
fn create_layers(data: &String, width: usize, height: usize) -> Vec<Vec<u32>> {
    let pixels: Vec<u32> = data.trim().chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut layers = Vec::with_capacity(pixels.len() / (width * height));
    let layer_length = width * height;
    let mut pixel = 0;
    while pixel < pixels.len() {
        let mut layer = Vec::with_capacity(width * height);

        for _ in 0..layer_length {
            layer.push(pixels[pixel]);
            pixel += 1;
        }

        layers.push(layer);
    }

    layers
}

/// Flatten all the layers into one final image.
fn flatten_layers(layers: &Vec<Vec<u32>>, width: usize, height: usize) -> Vec<u32> {
    let mut image: Vec<u32> = vec![2; width * height];

    for layer in layers {
        for i in 0..(width * height) {
            if image[i] == 2 {
                image[i] = layer[i];
            }
        }
    }

    image
}

/// Print a given `Vec` to stdout, ensuring proper dimensions.
fn draw_image(image: &Vec<u32>, width: usize, height: usize) {
    let mut buffer = String::new();

    for y in 0..height {
        for x in 0..width {
            let index = width * y + x;
            match image[index] {
                0 => buffer.push(' '),
                1 => buffer.push('█'),
                _ => buffer.push(' ')
            }
        }
        buffer.push('\n');
    }

    print!("{}", buffer);
}

#[test]
fn part_one_examples() {
    let layers = create_layers(&"123456789012".to_string(), 3, 2);
    assert_eq!(vec![1, 2, 3, 4, 5, 6], layers[0]);
    assert_eq!(vec![7, 8, 9, 0, 1, 2], layers[1]);
}

#[test]
fn part_two_examples() {
    let layers = create_layers(&"0222112222120000".to_string(), 2, 2);
    assert_eq!(vec![0, 2, 2, 2], layers[0]);
    assert_eq!(vec![1, 1, 2, 2], layers[1]);
    assert_eq!(vec![2, 2, 1, 2], layers[2]);
    assert_eq!(vec![0, 0, 0, 0], layers[3]);
    assert_eq!(vec![0, 1, 1, 0], flatten_layers(&layers, 2, 2));
}