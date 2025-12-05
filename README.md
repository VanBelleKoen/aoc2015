# Advent of Code 2015

Rust solutions for [Advent of Code 2015](https://adventofcode.com/2015).

## Project Structure

```
.
├── src/
│   ├── days/           # Daily solutions
│   │   ├── day01.rs
│   │   └── ...
│   ├── utils/          # Helper functions
│   │   ├── input.rs    # Input parsing utilities
│   │   ├── grid.rs     # Grid navigation and algorithms
│   │   └── math.rs     # Mathematical utilities
│   └── main.rs         # CLI runner
├── inputs/             # Puzzle inputs
├── examples/           # Example inputs for testing
└── scripts/            # Automation scripts
```

## Setup

1. Install Rust: https://rustup.rs/

2. Clone this repository and navigate to it:
```bash
cd /path/to/aoc2015
```

3. Set your Advent of Code session cookie (optional, for automatic input download):
```bash
# Copy the example .env file
cp .env.example .env

# Edit .env and add your session cookie
# Get it from https://adventofcode.com after logging in:
# - Open DevTools (F12)
# - Go to Application/Storage → Cookies
# - Copy the value of the 'session' cookie
```

Alternatively, you can set it as an environment variable:
```bash
export AOC_SESSION=your_session_cookie_here
```

## Usage

### Quick Setup for a New Day

The easiest way to start a new day:

```bash
./scripts/setup_day.sh <day_number>
```

This will:
- Create the day template file
- Create empty input/example files
- Download the puzzle input (if AOC_SESSION is set)
- Update the module structure

### Manual Setup

If you prefer to set things up manually:

```bash
# Create day template
./scripts/new_day.sh <day_number>

# Download input (requires AOC_SESSION)
./scripts/download_input.sh <day_number>
```

### Running Solutions

```bash
# Run both parts of a day
cargo run -- --day 1

# Run only part 1
cargo run -- --day 1 --part 1

# Run with example input
cargo run -- --day 1 --example

# Run with benchmarking (100 iterations)
cargo run --release -- --day 1 --bench
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for a specific day
cargo test day01

# Run tests with output
cargo test -- --nocapture
```

### Building for Release

```bash
cargo build --release
./target/release/aoc2015 --day 1
```

## Helper Utilities

### Input Parsing (`utils::input`)

```rust
use crate::utils::input;

// Read puzzle input
let input = input::read_input(1);

// Parse as lines
let lines = input::lines(&input);

// Parse as integers
let numbers = input::parse_integers(&input);

// Split by blank lines
let groups = input::split_by_blank_lines(&input);
```

### Grid Navigation (`utils::grid`)

```rust
use crate::utils::grid::{Grid, Position, Direction};

// Create a grid from input
let grid = Grid::new(input::parse_char_grid(&input));

// Navigate positions
let pos = Position::new(0, 0);
let neighbors = pos.neighbors();
let next = pos.move_in(Direction::North);

// Check validity and get values
if grid.is_valid(next) {
    let value = grid.get(next);
}
```

### Math Utilities (`utils::math`)

```rust
use crate::utils::math;

// GCD and LCM
let g = math::gcd(12, 18);
let l = math::lcm(12, 18);

// Check primality
if math::is_prime(17) { }

// Get divisors
let divs = math::divisors(24);
```

## Tips

- Add example inputs to `examples/dayXX.txt` for testing
- Use `--example` flag to test with example input before running on real input
- Write tests in each day module using the example cases from the puzzle
- Use `--bench` flag with `--release` to measure performance

## License

MIT
