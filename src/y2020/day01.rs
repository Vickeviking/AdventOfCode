use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day01.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day01.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        1
    }
}

fn solve_part_a(input: &str) -> u32 {
    let expenses: Vec<u32> = input
        .lines()
        .map(|s| {
            s.parse()
                .unwrap_or_else(|_| panic!("tried to unwrap {:?}", s))
        })
        .collect();
    let mut found = false;
    let mut operand1 = 0;
    let mut operand2 = 0;
    for (head_idx, head) in expenses.iter().enumerate() {
        for (tail_idx, tail) in expenses.iter().rev().enumerate() {
            if tail_idx == head_idx {
                continue;
            } else if head + tail == 2020 {
                operand1 = *head;
                operand2 = *tail;
                found = true;
            }
        }
        if found {
            break;
        }
    }

    operand1 * operand2
}

fn solve_part_b(input: &str) -> u32 {
    let expenses: Vec<u32> = input
        .lines()
        .map(|s| {
            s.parse()
                .unwrap_or_else(|_| panic!("tried to unwrap {:?}", s))
        })
        .collect();
    let mut found = false;
    let mut operand1 = 0;
    let mut operand2 = 0;
    let mut operand3 = 0;
    for (head_idx, head) in expenses.iter().enumerate() {
        for (tail_idx, tail) in expenses.iter().rev().enumerate() {
            for (idx, i) in expenses.iter().rev().enumerate() {
                if tail_idx == head_idx || tail_idx == idx || idx == head_idx {
                    continue;
                } else if head + tail + i == 2020 {
                    operand1 = *head;
                    operand2 = *tail;
                    operand3 = *i;
                    found = true;
                }
            }
        }
        if found {
            break;
        }
    }

    operand1 * operand2 * operand3
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
        let input = "1721
979
366
299
675
1456";
        assert_eq!(solve_part_b(input), 241861950);
    }
}
