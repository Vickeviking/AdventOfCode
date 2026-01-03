use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2016/day01.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2016/day01.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        1
    }
}

fn solve_part_a(_input: &str) -> i32 {
    // TODO: Implement part A
    0
}

fn solve_part_b(_input: &str) -> i32 {
    // TODO: Implement part B
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "test input";
        assert_eq!(solve_part_a(input), 0);
    }

    #[test]
    fn test_part_b() {
        let input = "test input";
        assert_eq!(solve_part_b(input), 0);
    }
}
