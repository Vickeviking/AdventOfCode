use std::collections::VecDeque;

use crate::Solution;

pub struct Day18;

impl Solution for Day18 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day18.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day18.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        18
    }
}

enum Token {
    Num(u64),
    Plus,
    Mul,
    LParen,
    RParen,
}

fn solve_part_a(input: &str) -> u64 {
    enum Token {
        Num(u64),
        Plus,
        Mul,
        LParen,
        RParen,
    }

    // helper: flush number buffer into token deque
    fn flush_num(buf: &mut String, tokens: &mut VecDeque<Token>) {
        if !buf.is_empty() {
            tokens.push_back(Token::Num(buf.parse().unwrap()));
            buf.clear();
        }
    }

    // tokenize a line
    fn tokenize(s: &str) -> VecDeque<Token> {
        use Token::*;
        let mut tokens = VecDeque::new();
        let mut buf = String::new();

        for c in s.chars() {
            match c {
                '+' => {
                    flush_num(&mut buf, &mut tokens);
                    tokens.push_back(Plus);
                }
                '*' => {
                    flush_num(&mut buf, &mut tokens);
                    tokens.push_back(Mul);
                }
                '(' => {
                    flush_num(&mut buf, &mut tokens);
                    tokens.push_back(LParen);
                }
                ')' => {
                    flush_num(&mut buf, &mut tokens);
                    tokens.push_back(RParen);
                }
                ' ' => {}
                _ => buf.push(c),
            }
        }

        flush_num(&mut buf, &mut tokens);
        tokens
    }

    // evaluate a number or parenthesis
    fn evaluate_atom(ts: &mut VecDeque<Token>) -> u64 {
        match ts.pop_front().unwrap() {
            Token::Num(n) => n,
            Token::LParen => {
                let value = evaluate_expr(ts);
                ts.pop_front(); // pop matching RParen
                value
            }
            _ => unreachable!(),
        }
    }

    // apply operator
    fn apply(lhs: u64, op: Token, rhs: u64) -> u64 {
        match op {
            Token::Plus => lhs + rhs,
            Token::Mul => lhs * rhs,
            _ => unreachable!(),
        }
    }

    // main left-to-right evaluation
    fn evaluate_expr(ts: &mut VecDeque<Token>) -> u64 {
        let mut value = evaluate_atom(ts);

        while let Some(op) = ts.front() {
            match op {
                Token::Plus | Token::Mul => {
                    let op = ts.pop_front().unwrap();
                    let rhs = evaluate_atom(ts);
                    value = apply(value, op, rhs);
                }
                Token::RParen => break,
                _ => unreachable!(),
            }
        }

        value
    }

    // loop over lines
    input
        .lines()
        .map(|l| {
            let mut ts = tokenize(l);
            evaluate_expr(&mut ts)
        })
        .sum()
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
        let examples = vec![
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];

        for (input, expected) in examples {
            let mut tokens = tokenize(input);
            let result = evaluate_expr(&mut tokens);
            assert_eq!(result, expected, "Failed on input: {}", input);
        }
    }

    #[test]
    fn test_part_b() {
        let input = "test input";
        assert_eq!(solve_part_b(input), 0);
    }
}
