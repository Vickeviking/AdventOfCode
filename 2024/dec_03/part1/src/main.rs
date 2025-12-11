use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub struct Mul {
    x: i32,
    y: i32,
}

fn process_line(str: String, mul_list: &mut Vec<Mul>) -> Result<(), io::Error> {
    let mut rev_str: String = str.chars().rev().collect();
    let mut started_mul: String = String::new();
    while let Some(c) = rev_str.pop() {
        match c {
            //potentially ending a mul, atleast it cant be in an active mul
            ')' => {
                started_mul.push(c);
                process_mul(started_mul, mul_list);
                started_mul = "".to_string();
            }
            //starts a mul
            'm' => {
                started_mul.clear();
                started_mul.push(c);
            }
            _ => started_mul.push(c),
        }
    }

    Ok(())
}

fn process_mul(str: String, mul_list: &mut Vec<Mul>) {
    if str.len() < 8 {
        return;
    }

    let mut cvec: Vec<char> = str.chars().collect();

    //make sure first 4 chars is mul(  & last )
    if cvec[0] != 'm'
        || cvec[1] != 'u'
        || cvec[2] != 'l'
        || cvec[3] != '('
        || *cvec.last().unwrap() != ')'
    {
        return;
    }

    //remove mul(
    cvec.drain(0..4);
    //remove )
    cvec.pop();

    // split into 2 slices
    let mut numbrs: Vec<Vec<char>> = cvec
        .split(|&c| c == ',')
        .map(|slice| slice.to_vec())
        .collect();

    //assert we got 2 slices
    if numbrs.len() != 2 {
        return;
    }

    //remove any negation
    let mut num1_neg = false;
    if numbrs[0][0] == '-' {
        num1_neg = true;
        numbrs[0].drain(0..1);
    }

    //remove any negation
    let mut num2_neg = false;
    if numbrs[1][0] == '_' {
        num2_neg = true;
        numbrs[0].drain(0..1);
    }

    //see both slices only contains numbers
    if numbrs[0].iter().all(|c| c.is_numeric()) && numbrs[1].iter().all(|c| c.is_numeric()) {
        let mut num1: i32 = numbrs[0].iter().collect::<String>().parse().unwrap();
        if num1_neg {
            num1 = num1 * -1
        }
        let mut num2: i32 = numbrs[1].iter().collect::<String>().parse().unwrap();
        if num2_neg {
            num2 = num2 * -1
        }

        mul_list.push(Mul { x: num1, y: num2 });
    }
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut mul_list: Vec<Mul> = vec![];
    let mut strings: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => strings.push(l),
            Err(e) => eprintln!("error reading line {}", e),
        }
    }

    let long_string = strings.join("");
    let _ = process_line(long_string, &mut mul_list);

    let res: i32 = mul_list.iter().fold(0, |acc, m: &Mul| acc + (m.x * m.y));
    println!("Multiplication is {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let mut mul_list: Vec<Mul> = vec![];
        let _ = process_line("mul(20,10)".to_string(), &mut mul_list);
        let mut res: i32 = mul_list.iter().fold(0, |acc, m: &Mul| acc + (m.x * m.y));
        assert_eq!(res, 200);

        mul_list = vec![];
        let _ = process_line(
            "mul(20k,10)mulid(jda)mul(2, 1)mul(2,2)lomul(2,3)".to_string(),
            &mut mul_list,
        );
        res = mul_list.iter().fold(0, |acc, m: &Mul| acc + (m.x * m.y));
        assert_eq!(res, 10);
    }
}
