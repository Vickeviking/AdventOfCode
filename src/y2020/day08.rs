use std::collections::HashSet;

use crate::Solution;

pub struct Day08;

impl Solution for Day08 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day08.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day08.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        8
    }
}

fn solve_part_a(input: &str) -> i32 {
    // load in instructions
    let mut instructions: Vec<(&str, i32)> = Vec::new();
    for l in input.lines() {
        let mut iter = l.split_whitespace();
        let op = iter.next().unwrap();
        let arg = iter.next().unwrap().parse::<i32>().unwrap();
        instructions.push((op, arg));
    }

    let mut executed: HashSet<usize> = HashSet::new();
    let mut pc = 0;
    let mut acc = 0;

    loop {
        // has instruction been executed? otherwise insert
        if !executed.insert(pc) {
            break;
        }

        let (op, arg) = instructions[pc];
        match op {
            "acc" => acc += arg,
            "jmp" => {
                pc = (arg + pc as i32) as usize;
                continue;
            }
            _ => {} //here nop is handled
        }

        pc += 1;
    }

    acc
}

fn solve_part_b(input: &str) -> i32 {
    // load in instructions
    let mut instructions: Vec<(&str, i32)> = Vec::new();
    for l in input.lines() {
        let mut iter = l.split_whitespace();
        let op = iter.next().unwrap();
        let arg = iter.next().unwrap().parse::<i32>().unwrap();
        instructions.push((op, arg));
    }

    let mut executed: HashSet<usize> = HashSet::new();
    let mut acc: i32 = 0;
    let mut possible_corrupted: Vec<(usize, i32)> = Vec::new(); // pc and acc
    let mut pc = 0;

    // find first infinite loop!
    loop {
        // has instruction been executed? otherwise insert
        if !executed.insert(pc) {
            break;
        }

        let (op, arg) = instructions[pc];
        match op {
            "jmp" => {
                possible_corrupted.push((pc, acc));
                pc = (arg + pc as i32) as usize;
                continue;
            }
            "nop" => {
                possible_corrupted.push((pc, acc));
            }
            "acc" => {
                acc += arg;
            }
            _ => {}
        }

        pc += 1;
    }

    // when pc = instructions.len() terminate
    // changing one of the instruction should make pc that, without running into an already visited
    // function,

    let end = instructions.len();
    for (c_pc, c_acc) in possible_corrupted {
        //flip instruction
        pc = c_pc;
        acc = c_acc;
        let (op, arg) = instructions[pc];
        match op {
            "nop" => {
                pc = (arg + pc as i32) as usize;
            }
            _ => pc += 1,
        }

        // now we are on a new instruction, until pc == end,
        let mut at_end = false;
        loop {
            if executed.contains(&pc) {
                break;
            } else if pc == end {
                at_end = true;
                break;
            }

            let (op, arg) = instructions[pc];
            match op {
                "jmp" => {
                    pc = (arg + pc as i32) as usize;
                    continue;
                }
                "acc" => {
                    acc += arg;
                }
                "nop" => {}
                _ => {}
            }
            pc += 1;
        }

        if at_end {
            break;
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(solve_part_a(input), 5);
    }

    #[test]
    fn test_part_b() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(solve_part_b(input), 8);
    }
}
