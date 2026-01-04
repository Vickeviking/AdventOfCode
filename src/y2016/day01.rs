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

fn solve_part_a(input: &str) -> i32 {
    let strings: Vec<&str> = input.split("/n").collect();
    println!("{:?}", strings);
    return strings.len() as i32;
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
        let input = "1721
979
366
299
675
1456";
        assert_eq!(solve_part_a(input), 514579);
    }

    #[test]
    fn test_part_b() {
        let input = "test input";
        assert_eq!(solve_part_b(input), 0);
    }
}
