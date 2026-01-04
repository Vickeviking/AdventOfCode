use std::collections::BTreeSet;

use crate::Solution;

pub struct Day05;

impl Solution for Day05 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day05.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day05.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        5
    }
}

fn parse_boarding_pass(input: &str) -> (usize, usize, usize) {
    struct MetaAcc {
        start: usize,
        end: usize,
    }

    let row = input
        .chars()
        .take(7)
        .fold(MetaAcc { start: 0, end: 127 }, |mut acc, c| {
            match c {
                'F' => acc.end = (acc.end + acc.start) / 2,
                'B' => acc.start = (acc.end + acc.start).div_ceil(2),
                _ => {}
            }
            acc
        })
        .start;

    let col = input
        .chars()
        .skip(7)
        .fold(MetaAcc { start: 0, end: 7 }, |mut acc, c| {
            match c {
                'L' => acc.end = (acc.end + acc.start) / 2,
                'R' => acc.start = (acc.end + acc.start).div_ceil(2),
                _ => {}
            }
            acc
        })
        .start;

    (row, col, (row * 8) + col)
}

fn solve_part_a(input: &str) -> usize {
    let mut biggest = 0;

    for l in input.lines() {
        let id = parse_boarding_pass(l).2;
        if id > biggest {
            biggest = id;
        }
    }

    biggest
}

fn solve_part_b(input: &str) -> usize {
    let mut ids: BTreeSet<usize> = BTreeSet::new();

    for l in input.lines() {
        ids.insert(parse_boarding_pass(l).2);
    }

    for id in ids.iter() {
        if ids.contains(&(id + 2)) && !ids.contains(&(id + 1)) {
            return id + 1;
        }
    }
    // due to input , this will not be reached
    panic!("should not reach here");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input1 = "BFFFBBFRRR";
        assert_eq!(parse_boarding_pass(input1), (70, 7, 567));
        let input2 = "FFFBBBFRRR";
        assert_eq!(parse_boarding_pass(input2), (14, 7, 119));
        let input3 = "BBFFBBFRLL";
        assert_eq!(parse_boarding_pass(input3), (102, 4, 820));
    }

    #[test]
    fn test_part_b() {
        let input = "test input";
        assert_eq!(solve_part_b(input), 0);
    }
}
