use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

struct Expression {
    sum: u64,
    operands: Vec<u64>,
}

fn check_expression_recursive(sum: u64, operands: &[u64], carry: u64) -> bool {
    // base case 1, we got to deep
    if carry > sum {
        return false;
    // we found it
    } else if carry == sum {
        return true;
    } else if operands.is_empty() {
        return false;
    } else {
        // lets see if adding or multiplying returns correct!
        let op = operands[0];
        let remaining_operands = &operands[1..];
        let add = carry + op;
        let mul = carry * op;
        let concatenated = format!("{}{}", carry, op);
        let result = concatenated.parse::<u64>().unwrap_or(u64::MAX);
        return check_expression_recursive(sum, remaining_operands, mul)
            || check_expression_recursive(sum, remaining_operands, add)
            || check_expression_recursive(sum, remaining_operands, result);
    }
}

fn check_expression(exp: &mut Expression) -> bool {
    if exp.operands.is_empty() {
        return false;
    }
    let first_op = exp.operands[0];
    let remaining_operands = &exp.operands[1..];
    check_expression_recursive(exp.sum, remaining_operands, first_op)
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut expressions: Vec<Expression> = vec![];
    let mut total = 0;

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let tuple: Vec<String> = l.split(':').map(|s| s.to_string()).collect();
                let sum = tuple[0].parse().unwrap();
                let operands: Vec<u64> = tuple[1]
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
                let expr = Expression { sum, operands };
                expressions.push(expr);
            }
            Err(e) => eprintln!("Error reading in line {}", e),
        }
    }

    for exp in &mut expressions {
        print!(
            "Expression, sum({}), operands({:?}) - ",
            exp.sum, exp.operands
        );
        let sum = exp.sum.clone();
        if check_expression(exp) {
            total += sum;
            println!("Pass");
        } else {
            println!("Fail");
        }
    }

    println!("total sum: {}", total);

    Ok(())
}
