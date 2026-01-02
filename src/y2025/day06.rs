use crate::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2025/day06.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2025/day06.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        6
    }
}

fn solve_part_a(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut data_set: Vec<Vec<u32>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    
    for l in lines {
        let is_numeric = l.trim().chars().next().unwrap().is_numeric();
        if is_numeric {
            data_set.push(
                l.split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
            );
        } else {
            operators = l
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().next().unwrap())
                .collect();
        }
    }

    let mut line_sum: Vec<u64> = vec![0u64; operators.len()];
    
    for row in data_set {
        for (x, digit) in row.iter().enumerate() {
            match operators[x] {
                '*' => {
                    if line_sum[x] == 0 {
                        line_sum[x] = *digit as u64;
                    } else {
                        line_sum[x] *= *digit as u64
                    }
                }
                '+' => line_sum[x] += *digit as u64,
                _ => panic!("Invalid operator"),
            }
        }
    }

    line_sum.iter().sum::<u64>()
}

fn solve_part_b(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut data_set: Vec<Vec<char>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    
    for l in lines {
        let first_char = l.chars().next().unwrap();
        if first_char.is_numeric() || first_char == ' ' {
            data_set.push(l.chars().collect());
        } else {
            operators = l
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().next().unwrap())
                .collect();
        }
    }

    let width = data_set.iter().map(|r| r.len()).max().unwrap();
    let height = data_set.len();
    let mut cols = vec![vec![' '; height]; width];

    for i in 0..height {
        for j in 0..data_set[i].len() {
            cols[j][i] = data_set[i][j]
        }
    }

    let mut total_sum = 0;
    let mut operator_idx = operators.len() as isize - 1;
    let mut line_sum = 0;
    
    for col in cols.iter().rev() {
        let digits: String = col.iter().filter(|&&c| c != ' ').collect();
        
        if digits.is_empty() {
            total_sum += line_sum;
            operator_idx -= 1;
            line_sum = 0;
        } else {
            let val: u64 = digits.parse().unwrap_or(0);
            
            if operator_idx >= 0 {
                match operators[operator_idx as usize] {
                    '*' => {
                        if line_sum == 0 {
                            line_sum = val;
                        } else {
                            line_sum *= val;
                        }
                    }
                    '+' => line_sum += val,
                    _ => panic!("Invalid operator"),
                }
            }
        }
    }
    
    total_sum += line_sum;
    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_b() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   + ";
        assert_eq!(solve_part_b(input), 3263827);
    }
}
