# Advent of Code

## Usage

```bash
# Create new day
cargo run --bin add_day <year> <day>

# Run solution
cargo run <year> <day> <part>
```

## Examples

```bash
cargo run --bin add_day 2025 10
cargo run 2025 10 a
cargo run --release 2025 10 b
```

## Template

Use this template for new days:

```rust
use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2025/day01.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2025/day01.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        1
    }
}

fn solve_part_a(input: &str) -> i32 {
    // Your solution here
    0
}

fn solve_part_b(input: &str) -> i32 {
    // Your solution here
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "test input";
        assert_eq!(solve_part_a(input), 42);
    }

    #[test]
    fn test_part_b() {
        let input = "test input";
        assert_eq!(solve_part_b(input), 84);
    }
}
```