## [Advent of Code 2019](https://adventofcode.com/2019)  
Each day is in its own folder within this workspace.  
Most code shared between days is broken out into the `common` library.

Work from a given day can be run with `cargo run [--release] -p <day##> [1|2]`.  
If a part is not specified (`1|2`), then both will be run.

Tests from a given day can be run with `cargo test [--release] -p <day##>`.

### External Crates
[crossbeam-channel](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel) since it's very nice.