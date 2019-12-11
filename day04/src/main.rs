use common::*;

/// Setup
fn main() {
    part_selector(String::new(), part_one, part_two);
}

fn part_one(_: String) {
    let valid = count_valid(246515, 739105);

    println!("[Part 1] Number of possible passwords within the given range: {}", valid);
}

fn part_two(_: String) {
    let valid = count_valid(246515, 739105);

    unimplemented!();

    println!("[Part 2] Number of possible passwords within the given range: {}", valid);
}

fn count_valid(a: u32, b: u32) -> usize {
    let mut valid = Vec::with_capacity(1000);

    let end = b;
    let mut num = a;
    'outer: while num <= end {
        let mut number = num;
        let mut last_digit = 10;

        // Ensure consequtive digits never decrease.
        for i in 1..=6 {
            let digit = number % 10;
            if last_digit < digit {
                num += 10u32.pow(i - 2);
                continue 'outer;
            }

            last_digit = digit;
            number = number / 10;
        }

        // Reset digit iteration variables.
        number = num;
        last_digit = 10;

        // Ensure at least two repeating digits.
        for _ in 1..=6 {
            let digit = number % 10;
            if last_digit == digit {
                valid.push(num);
                break;
            }

            last_digit = digit;
            number = number / 10;
        }
        
        num += 1;
    }

    valid.len()
}

#[test]
fn part_two_examples() {
    // TODO
}