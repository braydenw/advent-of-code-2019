//! Most of this code from this day in the Intcode virtual mashine
//! within the `common` library.

use common::*;

/// Setup
fn main() {
    let input = get_input(env!("CARGO_MANIFEST_DIR"));

    part_selector(input, part_one, part_two);
}

/// Seems to average ~0.75ms.
fn part_one(input: String) {
    let mut vm = IntcodeVM::new(input).log_level(1);
    vm.run();
}

fn part_two(_: String) {
    // TODO
}