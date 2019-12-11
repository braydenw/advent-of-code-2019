use common::*;

/// Setup
fn main() {
    part_selector(String::new(), part_one, part_two);
}

fn part_one(_: String) {
    let valid = count_valid(246515, 739105, None);

    println!("[Part 1] Number of possible passwords within the given range: {}", valid);
}

fn part_two(_: String) {
    let valid = count_valid(246515, 739105, Some(2));

    println!("[Part 2] Number of possible passwords within the given range: {}", valid);
}

/// Count the number of valid passwords contained in the `a..=b` range.
/// The `limit` ensures a group of repeated digits of `Some(..)` length
/// exists that isn't also part of a larger group.
fn count_valid(a: u32, b: u32, limit: Option<u8>) -> usize {
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
        let mut groups = [0u8; 10];
        let limit = limit.unwrap_or(0);
        for _ in 1..=6 {
            let digit = number % 10;
            if last_digit == digit {
                if limit > 0 {
                    groups[digit as usize] += 1;
                } else {
                    valid.push(num);
                    break;
                }
            }

            last_digit = digit;
            number = number / 10;
        }

        // Wasted cycles when passed `limit == None`.
        if groups.contains(&(limit - 1)) {
            valid.push(num);
        }
        
        num += 1;
    }

    valid.len()
}

#[test]
fn part_one_examples() {
    assert_eq!(1, count_valid(111111, 111111, None));
    assert_eq!(0, count_valid(223450, 223450, None));
    assert_eq!(0, count_valid(123789, 123789, None));
}

#[test]
fn part_two_examples() {
    assert_eq!(1, count_valid(112233, 112233, Some(2)));
    assert_eq!(0, count_valid(123444, 123444, Some(2)));
    assert_eq!(1, count_valid(111122, 111122, Some(2)));
}

#[test]
fn extra_tests() {
    assert_eq!(1, count_valid(123444, 123444, Some(3)));
    assert_eq!(0, count_valid(124444, 124444, Some(3)));
}