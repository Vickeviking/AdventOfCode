use crate::Solution;
use std::collections::BTreeMap;

pub struct Day05;

impl Solution for Day05 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2025/day05.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2025/day05.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        5
    }
}

fn solve_part_a(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut iter = lines.split(|l| l.is_empty());
    let fresh_ingredients_ranges = iter.next().unwrap();
    let available_ingredients: Vec<u64> = iter
        .next()
        .unwrap()
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    let mut fresh_ingredients = BTreeMap::<u64, u64>::new();

    for range in fresh_ingredients_ranges {
        let (a, b) = range.split_once("-").unwrap();
        let (a, b) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
        let entry = fresh_ingredients.entry(a).or_insert(b);
        *entry = (*entry).max(b);
    }

    let mut num_fresh = 0;

    for i in available_ingredients {
        num_fresh += fresh_ingredients.range(..=i).any(|(_, &end)| i <= end) as u64;
    }

    num_fresh
}

fn solve_part_b(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut iter = lines.split(|l| l.is_empty());
    let fresh_ingredients_ranges = iter.next().unwrap();

    let mut fresh_ingredients = BTreeMap::<u64, u64>::new();

    for range in fresh_ingredients_ranges {
        let (a, b) = range.split_once("-").unwrap();
        let (a, b) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
        let entry = fresh_ingredients.entry(a).or_insert(b);
        *entry = (*entry).max(b);
    }

    let mut ranges: Vec<(u64, u64)> = fresh_ingredients
        .iter()
        .map(|(&start, &end)| (start, end))
        .collect();

    ranges.sort_by_key(|&(start, _)| start);

    let mut sum = 0;
    let mut last_end = 0;

    for (start, end) in ranges {
        let start = start.max(last_end + 1);
        if start <= end {
            sum += end - start + 1;
            last_end = end;
        }
    }

    sum
}
