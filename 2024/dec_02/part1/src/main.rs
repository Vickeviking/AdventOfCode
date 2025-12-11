use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn process_report(report: String) -> bool {
    let vec: Vec<i32> = report
        .split_whitespace()
        .map(|a| a.parse::<i32>().unwrap())
        .collect();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reports() {
        //one el
        assert_eq!(process_report("3".to_string()), true);
        //two el
        assert_eq!(process_report("3 6".to_string()), true);
        assert_eq!(process_report("6 3".to_string()), true);
        // three el
        assert_eq!(process_report("3 6 7".to_string()), true);
        assert_eq!(process_report("3 6 10".to_string()), false);
        // decreasing
        assert_eq!(process_report("1 2 5 3 4".to_string()), true);
    }
}
