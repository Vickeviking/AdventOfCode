use std::collections::{HashMap, HashSet};

use crate::Solution;

pub struct Day17;

impl Solution for Day17 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day17.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day17.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        17
    }
}

fn solve_part_a(input: &str) -> i32 {
    let mut map: HashSet<(isize, isize, isize)> = HashSet::new();
    let cycles = 6;

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                map.insert((x as isize, y as isize, 0));
            }
        }
    }

    for _ in 0..cycles {
        let mut neighbor_count: HashMap<(isize, isize, isize), usize> = HashMap::new();

        // Count neighbors for each active cube
        for &(x, y, z) in &map {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }
                        let pos = (x + dx, y + dy, z + dz);
                        *neighbor_count.entry(pos).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut new_map = HashSet::new();

        // Apply rules
        for (pos, count) in neighbor_count {
            if map.contains(&pos) {
                if count == 2 || count == 3 {
                    new_map.insert(pos);
                }
            } else {
                if count == 3 {
                    new_map.insert(pos);
                }
            }
        }

        map = new_map;
    }

    map.len() as i32
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
        let input = ".#.
..#
###";
        assert_eq!(solve_part_a(input), 112);
    }

    #[test]
    fn test_part_b() {
        let input = "test input";
        assert_eq!(solve_part_b(input), 0);
    }
}
