use std::{collections::HashMap, u64};

use crate::Solution;

pub struct Day14;

impl Solution for Day14 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day14.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day14.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        14
    }
}

fn solve_part_a(input: &str) -> u64 {
    let mut mem: HashMap<u32, u64> = HashMap::new();
    let mut mask: Vec<(usize, bool)> = Vec::new();
    for l in input.lines() {
        let mut iter = l.split("=");
        let command = iter.next().unwrap().trim();
        let arg = iter.next().unwrap().trim();

        if command.starts_with("mask") {
            //parse mask and update old mask

            //clean old mask
            mask.clear();

            // update new mask
            for (idx, c) in arg.chars().enumerate() {
                match c {
                    '0' => {
                        mask.push((idx, false));
                    }
                    '1' => {
                        mask.push((idx, true));
                    }
                    _ => {}
                }
            }
        } else {
            // mem assignment
            let memslot: u32 = command
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .parse()
                .unwrap();

            let mut mem_val: u64 = arg.parse().unwrap();

            //now overwrite the arg with each bit
            for (idx, on) in &mask {
                let pos = 35 - idx;
                let tmp_mask = 1 << pos;
                if *on {
                    // slå på bit vid idx i mem_val
                    mem_val |= tmp_mask;
                } else {
                    // stäng av bit vid idx i mem_val
                    //invert bit mask
                    mem_val &= !tmp_mask;
                }
            }

            //now insert new value
            mem.insert(memslot, mem_val);
        }
    }

    mem.values().sum()
}

fn solve_part_b(input: &str) -> u64 {
    use std::collections::HashMap;

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: Vec<(usize, bool)> = Vec::new();

    for l in input.lines() {
        let mut iter = l.split('=');
        let command = iter.next().unwrap().trim();
        let arg = iter.next().unwrap().trim();

        if command.starts_with("mask") {
            mask.clear();
            for (idx, c) in arg.chars().enumerate() {
                match c {
                    'X' => mask.push((idx, false)), // floating
                    '1' => mask.push((idx, true)),  // force 1
                    _ => {}
                }
            }
        } else {
            let mut mem_slot: u64 = command
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .parse()
                .unwrap();

            let mem_val: u64 = arg.parse().unwrap();
            let mut floating_mask: u64 = 0;

            // Apply mask: set 1s, collect floating bits
            for (idx, on) in &mask {
                let pos = 35 - idx;
                let bit = 1 << pos;
                if *on {
                    mem_slot |= bit;
                } else {
                    floating_mask |= bit;
                }
            }

            let n = floating_mask.count_ones() as u64;

            // Iterate all 2**n combinations of floating bits
            for combo in 0..(1 << n) {
                let mut addr = mem_slot;
                let mut bit_idx = 0;

                for bit_pos in 0..36 {
                    if (floating_mask >> bit_pos) & 1 != 0 {
                        if (combo >> bit_idx) & 1 != 0 {
                            addr |= 1 << bit_pos;
                        } else {
                            addr &= !(1 << bit_pos);
                        }
                        bit_idx += 1;
                    }
                }

                mem.insert(addr, mem_val);
            }
        }
    }

    mem.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(solve_part_a(input), 165);
    }

    #[test]
    fn test_part_b() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(solve_part_b(input), 208);
    }
}
