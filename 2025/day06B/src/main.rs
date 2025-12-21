use std::{
    char,
    fs::File,
    io::{self, BufRead},
};

fn calculate_sum(lines: Vec<String>) -> u64 {
    let mut data_set: Vec<Vec<char>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    
    for l in lines {
        //check first digit
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

    //we need max width of rows
    let width = data_set.iter().map(|r| r.len()).max().unwrap();

    //transpose
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
        // Form a number from this column by reading down (ignoring spaces)
        let digits: String = col.iter().filter(|&&c| c != ' ').collect();
        
        if digits.is_empty() {
            // Empty column (all spaces) = separator
            total_sum += line_sum;
            operator_idx -= 1;
            line_sum = 0;
        } else {
            // Parse the number from concatenated digits
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
                    _ => panic!(),
                }
            }
        }
    }
    //since no ending seperator, we push linesum so we dont forget last col
    total_sum += line_sum;

    total_sum
}

fn main() -> io::Result<()> {
    let file = File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = io::BufReader::new(file);

    let lines = reader
        .lines()
        .map(|x| x.expect("Valid strings due to proj desc"))
        .collect::<Vec<String>>();

    let total_sum = calculate_sum(lines);

    println!("{:?}", total_sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   + ".to_string(),
        ];

        assert_eq!(calculate_sum(input), 3263827);
    }
}
