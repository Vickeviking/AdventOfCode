use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2025/day01.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2025/day01.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        1
    }
}

struct DialInstruction {
    left: bool,
    steps: u16,
}

impl DialInstruction {
    fn from_string(s: &str) -> Self {
        let mut char_iter = s.chars();
        let left = match char_iter.next().expect("at least one char") {
            'L' => true,
            'R' => false,
            _ => panic!("Invalid direction"),
        };
        let steps: u16 = char_iter.collect::<String>().parse().unwrap();
        DialInstruction { left, steps }
    }
}

fn solve_part_a(input: &str) -> u32 {
    let mut zeroes: u32 = 0;
    let mut dial: i16 = 50;

    for line in input.lines() {
        let ins = DialInstruction::from_string(line);
        dial = match ins.left {
            true => (dial - ins.steps as i16) % 100,
            false => (dial + ins.steps as i16) % 100,
        };

        if dial == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

fn solve_part_b(input: &str) -> u16 {
    let mut zeroes: u16 = 0;
    let mut dial: i16 = 50;

    for line in input.lines() {
        let ins = DialInstruction::from_string(line);

        let cycles = ins.steps / 100;
        let normalized_steps: i16 = (ins.steps % 100) as i16;
        zeroes += cycles;

        if ins.left {
            if dial == 0 {
                dial = 100 - normalized_steps;
                continue;
            }

            let new_dial: i16 = dial - normalized_steps;
            if new_dial <= 0 {
                zeroes += 1;
            }
            dial = new_dial.rem_euclid(100);
        } else {
            if dial == 0 {
                dial = normalized_steps;
                continue;
            }

            let new_dial = dial + normalized_steps;
            if new_dial >= 100 {
                zeroes += 1;
            }
            dial = new_dial.rem_euclid(100);
        }
    }

    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dial_instruction() {
        let d = DialInstruction::from_string("L12");
        assert!(d.left);
        assert_eq!(d.steps, 12);

        let d = DialInstruction::from_string("R5");
        assert!(!d.left);
        assert_eq!(d.steps, 5);
    }
}
