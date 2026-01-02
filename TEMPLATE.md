# Template

```rust
use crate::Solution;

pub struct DayXX;

impl Solution for DayXX {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/YYYY/dayXX.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/YYYY/dayXX.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        XX
    }
}

fn solve_part_a(_input: &str) -> i32 {
    0
}

fn solve_part_b(_input: &str) -> i32 {
    0
}
```

    0  // Replace with actual result
}

fn solve_part_b(input: &str) -> i32 {
    // Parse input
    
    // Solve part B
    
    0  // Replace with actual result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "sample input from problem";
        assert_eq!(solve_part_a(input), 42);  // Replace with expected value
    }

    #[test]
    fn test_part_b() {
        let input = "sample input from problem";
        assert_eq!(solve_part_b(input), 84);  // Replace with expected value
    }
}
```

## Steps to add a new day:

1. Copy this template to `src/yYYYY/dayXX.rs`
2. Replace `XX` with the day number and `YYYY` with the year
3. Add `pub mod dayXX;` to `src/yYYYY/mod.rs`
4. Add your input to `inputs/YYYY/dayXX.txt`
5. Register in `src/main.rs`:
   ```rust
   fn get_YYYY_solution(day: u8) -> Option<Box<dyn Solution>> {
       match day {
           // ... existing days
           XX => Some(Box::new(yYYYY::dayXX::DayXX)),
           _ => None,
       }
   }
   ```
6. Implement `solve_part_a()` and `solve_part_b()`
7. Add test cases
8. Run: `cargo test yYYYY::dayXX`
9. Run: `cargo run YYYY XX`
