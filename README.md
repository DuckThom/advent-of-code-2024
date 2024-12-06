# Advent Of Code 2024: Rust Edition

## Running validation

`cargo test`

## Running real input

> **Note**: Replace `#` with the day number. For example `src/inputs/day_3/input` or `cargo run -- 3`

1. Get your input from [the website](https://adventofcode.com/2024)
2. Place the input string in `src/inputs/day_#>/input`
3. It's possible to run all days or a single one:
    1. All days: `cargo run`
    2. Single day: `cargo run -- #`

> **Tip**: Use `cargo run -r` or `cargo run -r -- #` to run an optimized (faster) release build!

### Auto downloading input

To enable the automatic downloading of your input files, set the env var `AOC_SESSION` to the value of your `session`
cookie.