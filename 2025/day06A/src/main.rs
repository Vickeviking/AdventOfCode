use std::{
    char,
    fs::File,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let file = File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = io::BufReader::new(file);

    let lines = reader
        .lines()
        .map(|x| x.expect("Valid strings due to proj desc"))
        .collect::<Vec<String>>();

    let mut data_set: Vec<Vec<u32>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    // cleaning up the dataset from white spaces
    for l in lines {
        //check first digit
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
    println!("vec has lenght {:?}", line_sum.len());
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
                _ => panic!(),
            }
        }
    }

    let total_sum = line_sum.iter().sum::<u64>();

    println!("{:?}", total_sum);
    Ok(())
}
