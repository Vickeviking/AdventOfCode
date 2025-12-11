use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

struct ListTouple {
    lst1: Vec<i32>,
    lst2: Vec<i32>,
}

impl ListTouple {
    pub fn new() -> Self {
        ListTouple {
            lst1: Vec::new(),
            lst2: Vec::new(),
        }
    }
}

fn process_line(str: String, list_touple: &mut ListTouple) -> Result<(), io::Error> {
    let vec: Vec<String> = str.split_whitespace().map(String::from).collect();

    if vec.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "line had unexpected format",
        ));
    }

    list_touple.lst1.push(vec[0].parse::<i32>().unwrap());
    list_touple.lst2.push(vec[1].parse::<i32>().unwrap());
    Ok(())
}
fn main() -> Result<(), io::Error> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut list_touple = ListTouple::new();

    for line in reader.lines() {
        match line {
            Ok(l) => process_line(l, &mut list_touple)?,
            Err(e) => eprintln!("error reading line {}", e),
        }
    }

    list_touple.lst1.sort();
    list_touple.lst2.sort();

    let result: i32 = list_touple
        .lst1
        .iter()
        .zip(list_touple.lst2.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    println!("Hello, the total difference is {}", result);

    Ok(())
}
