use crate::Solution;
use std::collections::{BTreeMap, HashSet};

pub struct Day08;

impl Solution for Day08 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2025/day08.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2025/day08.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        8
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd, Hash)]
struct Point(usize, usize);

#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd, Hash)]
struct Square(Point, Point);

impl Square {
    fn area(self) -> u64 {
        let dx = (self.1.0 as i64 - self.0.0 as i64).abs() + 1;
        let dy = (self.1.1 as i64 - self.0.1 as i64).abs() + 1;
        (dx as u64) * (dy as u64)
    }
}

fn solve_part_a(input: &str) -> u64 {
    let mut points: Vec<Point> = Vec::new();

    let normalize = |p1: Point, p2: Point| -> Square {
        if p1 < p2 {
            Square(p1, p2)
        } else {
            Square(p2, p1)
        }
    };

    for line in input.lines() {
        let p: Vec<usize> = line
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        points.push(Point(p[0], p[1]));
    }

    let mut squares: BTreeMap<u64, HashSet<Square>> = BTreeMap::new();

    for (i, &p1) in points.iter().enumerate() {
        for (j, &p2) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            let s = normalize(p1, p2);
            let area = s.area();
            squares.entry(area).or_default().insert(s);
        }
    }

    squares.pop_last().unwrap().0
}

fn solve_part_b(_input: &str) -> u64 {
    12
}
