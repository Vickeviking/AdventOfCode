use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn process_report(report: String) -> bool {
    let mut vec: Vec<i32> = report
        .split_whitespace()
        .map(|a| a.parse::<i32>().unwrap())
        .collect();

    if process_report_helper(&mut vec) {
        return true;
    }

    let mut index = 0;
    for _ in &vec {
        let mut vec_clone = vec.clone();
        vec_clone.remove(index);
        if process_report_helper(&mut vec_clone) {
            return true;
        }

        index += 1;
    }
    return false;
}

fn process_report_helper(vec: &mut Vec<i32>) -> bool {
    if vec.len() == 1 {
        return true;
    }
    let mut safe = true;
    let decreasing = vec[0] > vec[1];
    let mut latest = vec[0];

    for i in vec.iter().skip(1) {
        if (decreasing != (latest > *i))
            || !(((latest - i).abs() >= 1) && ((latest - i).abs()) <= 3)
        {
            safe = false;
            break;
        }

        latest = *i;
    }
    return safe;
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut safe_reports: i32 = 0;

    for report in reader.lines() {
        match report {
            Ok(r) => safe_reports += process_report(r) as i32,
            Err(e) => eprintln!("Error reading line {}", e),
        }
    }

    println!("There was {} safe reports", safe_reports);

    Ok(())
}
