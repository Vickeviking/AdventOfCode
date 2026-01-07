use std::collections::HashMap;

use crate::Solution;

pub struct Day15;

impl Solution for Day15 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day15.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day15.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        15
    }
}

fn solve_part_a(input: &str) -> u16 {
    let numbers: Vec<u8> = input
        .trim_ascii_end()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    // number , last_spoken
    let mut mem: HashMap<u16, u16> = HashMap::new();

    for (idx, val) in numbers.iter().enumerate().take(numbers.len() - 1) {
        mem.insert(*val as u16, idx as u16 + 1);
    }

    let mut turn_number = numbers.len() as u16;
    let mut last_spoken = numbers[numbers.len() - 1] as u16;

    while turn_number < 2020 {
        turn_number += 1;

        if let Some(&turn_spoken) = mem.get(&last_spoken) {
            // not first time, update last spoken
            // insert last spoken
            mem.insert(last_spoken, turn_number - 1);

            // update last spoken (the number we choose to say)
            last_spoken = turn_number - 1 - turn_spoken;
        } else {
            // must be first time
            // we insert last token, and then say 0
            mem.insert(last_spoken, turn_number - 1);
            last_spoken = 0;
        }
    }
    last_spoken
}

fn solve_part_b(input: &str) -> u32 {
    let numbers: Vec<usize> = input
        .trim_ascii_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let size = 30_000_000;
    let mut last_seen = vec![0usize; size]; // 0 means not seen yet

    for (idx, &num) in numbers.iter().enumerate().take(numbers.len() - 1) {
        last_seen[num] = idx + 1;
    }

    let mut turn_number = numbers.len();
    let mut last_spoken = numbers[numbers.len() - 1];

    while turn_number < size {
        let last_turn = last_seen[last_spoken];
        last_seen[last_spoken] = turn_number;
        last_spoken = if last_turn == 0 {
            0
        } else {
            turn_number - last_turn
        };
        turn_number += 1;
    }

    last_spoken as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "0,3,6";
        assert_eq!(solve_part_a(input), 436);
    }

    #[test]
    fn test_part_b() {
        let input = "0,3,6";
        assert_eq!(solve_part_b(input), 175594);
    }
}
