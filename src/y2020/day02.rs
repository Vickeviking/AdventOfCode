use core::{error, fmt};
use std::str::FromStr;

use crate::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day02.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day02.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        2
    }
}

struct PasswordAndRuleset<'a> {
    range: (u8, u8),
    char: char,
    password: &'a str,
}

#[derive(Debug)]
struct ParsePassordAndRulesetError;

impl fmt::Display for ParsePassordAndRulesetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing input")
    }
}

impl error::Error for ParsePassordAndRulesetError {}

impl<'a> PasswordAndRuleset<'a> {
    pub fn is_valid_part_a(&self) -> bool {
        let char_repetitions = self.password.chars().filter(|c| *c == self.char).count();
        (self.range.0..=self.range.1).contains(&(char_repetitions as u8))
    }

    pub fn is_valid_part_b(&self) -> bool {
        let a = self
            .password
            .chars()
            .nth(self.range.0 as usize - 1)
            .expect("valid input")
            == self.char;
        let b = self
            .password
            .chars()
            .nth(self.range.1 as usize - 1)
            .expect("valid input")
            == self.char;

        a ^ b
    }
}

fn parse_password<'a>(s: &'a str) -> Result<PasswordAndRuleset<'a>, ParsePassordAndRulesetError> {
    let mut s_iter = s.split_whitespace();
    let mut range_parts = s_iter.next().ok_or(ParsePassordAndRulesetError)?.split('-');
    let start = range_parts
        .next()
        .ok_or(ParsePassordAndRulesetError)?
        .parse::<u8>()
        .map_err(|_| ParsePassordAndRulesetError)?;
    let end = range_parts
        .next()
        .ok_or(ParsePassordAndRulesetError)?
        .parse::<u8>()
        .map_err(|_| ParsePassordAndRulesetError)?;

    let c = s_iter
        .next()
        .ok_or(ParsePassordAndRulesetError)?
        .chars()
        .next()
        .ok_or(ParsePassordAndRulesetError)?;

    let password = s_iter.next().ok_or(ParsePassordAndRulesetError)?;

    Ok(PasswordAndRuleset {
        range: (start, end),
        char: c,
        password,
    })
}

fn solve_part_a(input: &str) -> i32 {
    let mut valid_passw = 0;
    for l in input.lines() {
        if parse_password(l).expect("input is clean").is_valid_part_a() {
            valid_passw += 1;
        }
    }
    valid_passw
}

fn solve_part_b(input: &str) -> i32 {
    let mut valid_passw = 0;
    for l in input.lines() {
        if parse_password(l).expect("input is clean").is_valid_part_b() {
            valid_passw += 1;
        }
    }
    valid_passw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(solve_part_a(input), 2);
    }

    #[test]
    fn test_part_b() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(solve_part_b(input), 1);
    }
}
