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

// AST impl
enum Expr {
    // leaf node
    Num(u64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn solve_part_b(input: &str) -> u64 {
    fn flush_num(buf: &mut String, tokens: &mut VecDeque<Token>) {
        if !buf.is_empty() {
            tokens.push_back(Token::Num(buf.parse().unwrap()));
            buf.clear();
        }
    }

    // inner helper: tokenize
    fn tokenize(s: &str) -> VecDeque<Token> {
        /* same as part A */
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

    // inner parser functions
    fn parse_expr(ts: &mut VecDeque<Token>) -> Expr {
        parse_mul(ts)
    }

    fn parse_mul(ts: &mut VecDeque<Token>) -> Expr {
        let mut lhs = parse_add(ts); // addition binds tighter

        while matches!(ts.front(), Some(Token::Mul)) {
            ts.pop_front();
            let rhs = parse_add(ts);
            lhs = Expr::Mul(Box::new(lhs), Box::new(rhs));
        }

        lhs
    }

    fn parse_add(ts: &mut VecDeque<Token>) -> Expr {
        let mut lhs = parse_atom(ts);

        while matches!(ts.front(), Some(Token::Plus)) {
            ts.pop_front();
            let rhs = parse_atom(ts);
            lhs = Expr::Add(Box::new(lhs), Box::new(rhs));
        }

        lhs
    }

    fn parse_atom(ts: &mut VecDeque<Token>) -> Expr {
        match ts.pop_front().unwrap() {
            Token::Num(n) => Expr::Num(n),
            Token::LParen => {
                let e = parse_expr(ts);
                assert!(matches!(ts.pop_front().unwrap(), Token::RParen));
                e
            }
            _ => unreachable!(),
        }
    }

    // evaluation
    fn eval(e: &Expr) -> u64 {
        match e {
            Expr::Num(n) => *n,
            Expr::Add(l, r) => eval(l) + eval(r),
            Expr::Mul(l, r) => eval(l) * eval(r),
        }
    }
    // loop over lines
    input
        .lines()
        .map(|l| {
            let mut ts = tokenize(l);
            let ast = parse_expr(&mut ts);
            eval(&ast)
        })
        .sum()
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
            let result = solve_part_a(input);
            assert_eq!(result, expected, "Failed on input: {}", input);
        }
    }

    #[test]
    fn test_part_b() {
        let examples = vec![
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];

        for (input, expected) in examples {
            let result = solve_part_b(input);
            assert_eq!(result, expected, "Failed on input: {}", input);
        }
    }
}
