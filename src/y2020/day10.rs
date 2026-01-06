use std::collections::HashSet;

use crate::Solution;

pub struct Day10;

impl Solution for Day10 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day10.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day10.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        10
    }
}

// device adapter 3 more than biggest in list
// each adapter in list can be seried with at most a 3 lower i.e 3->6->9
// charging outlet has 0 joltage,
//

fn solve_part_a(input: &str) -> usize {
    let mut one_joltage_dif = 0;
    let mut three_joltage_dif = 0;

    let mut joltages: Vec<u16> = input.lines().map(|l| l.parse::<u16>().unwrap()).collect();
    joltages.push(0);
    joltages.sort();

    for i in (1..joltages.len()) {
        match joltages[i] - joltages[i - 1] {
            1 => one_joltage_dif += 1,
            3 => three_joltage_dif += 1,
            _ => {}
        }
    }

    one_joltage_dif * (three_joltage_dif + 1)
}

fn solve_part_b(input: &str) -> usize {
    let mut joltages: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();
    joltages.push(0);
    joltages.sort();
    joltages.push(joltages[joltages.len() - 1] + 3);

    let mut ways: Vec<usize> = vec![1usize; joltages.len()];

    //antalet sätt att nå nod x , är summan av de bakom liggande noderna med distans 1-3

    for (index, value) in joltages.iter().enumerate() {
        if index == 0 {
            continue;
        }

        let mut sum = 0;
        for i in (1..=3) {
            if let Some(j) = index.checked_sub(i) {
                // Safety, subtractions only ever access earlier elements, we never exceed right bound

                let joltage_rating = joltages.get(j).unwrap();
                let last_ways = ways.get(j).unwrap();
                if *value - *joltage_rating <= 3 {
                    sum += last_ways;
                } else {
                    break;
                }
            }
        }
        ways[index] = sum;
    }

    ways[joltages.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(solve_part_a(input), 220);
    }

    #[test]
    fn test_part_b() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(solve_part_b(input), 8);
    }
}
