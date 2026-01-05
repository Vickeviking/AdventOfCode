use std::collections::VecDeque;

use crate::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day09.txt");
        solve_part_a(input, 25).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day09.txt");
        const INVALID_NUM: usize = 1124361034;
        solve_part_b(input, INVALID_NUM).to_string()
    }

    fn day(&self) -> u8 {
        9
    }
}

fn find_first_invalid(num_list: Vec<usize>, mut preamble: VecDeque<usize>) -> usize {
    for target_num in num_list {
        let mut valid = false;
        // is valid? if any of the history num creates it its not
        for i in preamble.iter() {
            for j in preamble.iter() {
                if i == j {
                    continue;
                }

                if i + j == target_num {
                    valid = true;
                    break;
                }
            }
            if valid {
                break;
            }
        }

        // if valid still false, means no combination satisfied
        if !valid {
            return target_num;
        }

        // now we push the num back, and pop front
        preamble.pop_front();
        preamble.push_back(target_num);
    }

    panic!("There will be an invalid num")
}

fn solve_part_a(input: &str, preamble_length: usize) -> usize {
    let mut num_list: Vec<usize> = input.lines().map(|s| s.parse::<usize>().unwrap()).collect();
    let preamble: VecDeque<usize> = num_list.drain(..preamble_length).collect();

    find_first_invalid(num_list, preamble)
}

fn solve_part_b(input: &str, invalid_num: usize) -> usize {
    let num_list: Vec<usize> = input.lines().map(|s| s.parse::<usize>().unwrap()).collect();
    let mut head = 0;
    let mut tail = 0;
    let mut sum = num_list[tail];

    //SAFETY: There is a window summing up to invalid_sum
    loop {
        if sum == invalid_num {
            break;
        } else if sum < invalid_num {
            head += 1;
            sum += num_list[head];
        } else {
            sum -= num_list[tail];
            tail += 1;
        }
    }
    let window = &num_list[tail..=head];
    window.iter().max().unwrap() + window.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(solve_part_a(input, 5), 127);
    }

    #[test]
    fn test_part_b() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(solve_part_b(input, 127), 62);
    }
}
