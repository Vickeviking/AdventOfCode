use std::collections::HashMap;

use crate::Solution;

pub struct Day16;

impl Solution for Day16 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day16.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day16.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        16
    }
}

fn solve_part_a(input: &str) -> i32 {
    // use BTreeMap perhaps
    let mut map: HashMap<&str, Vec<(u16, u16)>> = HashMap::new();

    let mut sections = input.split("\n\n");

    //parsing field sections
    let field_section = sections.next().unwrap();
    for l in field_section.lines() {
        let mut iter = l.split(":");
        let field = iter.next().unwrap();
        let range_text = iter.next().unwrap();

        // go through all the ranges by the hashmap
        for r in range_text.split("or") {
            let range: Vec<u16> = r
                .trim()
                .split('-')
                .map(|n| n.parse::<u16>().unwrap())
                .collect();
            let start = range[0];

            let end = range[1];

            //push range to field in hashmap
            map.entry(field)
                .and_modify(|v| v.push((start, end)))
                .or_insert(vec![(start, end)]);
        }
    }

    let my_tickets: Vec<u16> = sections
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u16>().unwrap())
        .collect();

    let nearby: Vec<Vec<u16>> = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|n| n.parse::<u16>().unwrap()).collect())
        .collect();

    // do rest lata

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
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(solve_part_a(input), 71);
    }

    #[test]
    fn test_part_b() {
        let input = "test input";
        assert_eq!(solve_part_b(input), 0);
    }
}
