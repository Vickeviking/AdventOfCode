use crate::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2025/day03.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2025/day03.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        3
    }
}

fn highest_digit_in_string(s: &str, start_idx: usize, end_idx: usize) -> (usize, u8) {
    let digit_arr: Vec<u8> = s
        .chars()
        .skip(start_idx)
        .take(end_idx - start_idx + 1)
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();

    let mut biggest_digit: u8 = 0;
    let mut string_index: usize = 0;
    for (index, value) in digit_arr.iter().enumerate() {
        if *value > biggest_digit {
            biggest_digit = *value;
            string_index = index;
            if biggest_digit == 9 {
                break;
            }
        }
    }
    string_index += start_idx;
    (string_index, biggest_digit)
}

fn bank_max_joltage_a(bank: &str) -> u8 {
    let num_batteries = bank.len();
    if num_batteries <= 2 {
        return bank.parse::<u8>().unwrap();
    }

    let a = highest_digit_in_string(bank, 0, num_batteries - 2);
    let b = highest_digit_in_string(bank, a.0 + 1, num_batteries - 1);

    a.1 * 10 + b.1
}

fn bank_max_joltage_b(bank: &str) -> u64 {
    let num_batteries = bank.len();
    if num_batteries <= 12 {
        return bank.parse::<u64>().unwrap();
    }

    let mut batteries: Vec<u8> = vec![0u8; 12];
    let mut last_idx: usize = 0;
    
    for i in (0..12).rev() {
        let digit = highest_digit_in_string(bank, last_idx, num_batteries - i - 1);
        last_idx = digit.0 + 1;
        batteries[11 - i] = digit.1;
    }

    let mut sum: u64 = 0;
    for b in batteries {
        sum += b as u64;
        sum *= 10;
    }
    sum / 10
}

fn solve_part_a(input: &str) -> u16 {
    input.lines().map(|bank| bank_max_joltage_a(bank) as u16).sum()
}

fn solve_part_b(input: &str) -> u64 {
    input.lines().map(|bank| bank_max_joltage_b(bank)).sum()
}
