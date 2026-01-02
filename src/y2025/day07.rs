use crate::Solution;
use std::collections::HashSet;

pub struct Day07;

impl Solution for Day07 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2025/day07.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2025/day07.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        7
    }
}

fn solve_part_a(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut map: HashSet<(usize, usize)> = HashSet::new();
    let mut start_pos: usize = 0;
    
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '^' => {
                    map.insert((x, y));
                }
                'S' => start_pos = x,
                _ => {}
            };
        }
    }
    
    let mut total_splits = 0;
    let height = lines.len();
    let width = lines[0].len();
    let mut next_row: Vec<bool> = vec![false; width];
    next_row[start_pos] = true;

    for y in 0..height {
        for x in 0..width {
            if next_row[x] && map.contains(&(x, y)) {
                next_row[x - 1] = true;
                next_row[x] = false;
                next_row[x + 1] = true;
                total_splits += 1;
            }
        }
    }

    total_splits
}

fn solve_part_b(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut map: HashSet<(usize, usize)> = HashSet::new();
    let mut start_pos: usize = 0;
    
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '^' => {
                    map.insert((x, y));
                }
                'S' => start_pos = x,
                _ => {}
            };
        }
    }
    
    let height = lines.len();
    let width = lines[0].len();
    let mut next_row: Vec<u64> = vec![0u64; width];
    next_row[start_pos] = 1;

    for y in 0..height {
        for x in 0..width {
            if next_row[x] > 0 && map.contains(&(x, y)) {
                next_row[x - 1] += next_row[x];
                next_row[x + 1] += next_row[x];
                next_row[x] = 0;
            }
        }
    }

    next_row.iter().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
        assert_eq!(solve_part_a(input), 21);
    }

    #[test]
    fn test_part_b() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
        assert_eq!(solve_part_b(input), 40);
    }
}
