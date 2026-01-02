use advent_of_code::Solution;
use std::env;
use std::path::Path;
use std::time::Instant;

mod y2015 {
    pub use advent_of_code::y2015::*;
}

mod y2025 {
    pub use advent_of_code::y2025::*;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("Usage: cargo run <year> <day> [part]");
        println!("Example: cargo run 2025 1");
        println!("Example: cargo run 2025 1 a");
        return;
    }
    
    let year = &args[1];
    let day: u8 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid day number");
        std::process::exit(1);
    });
    let part = if args.len() > 3 { Some(args[3].as_str()) } else { None };
    
    // Check if input file exists
    let input_path = format!("inputs/{}/day{:02}.txt", year, day);
    if !Path::new(&input_path).exists() {
        eprintln!("❌ Input file not found: {}", input_path);
        eprintln!("Create it or run: cargo run --bin add_day {} {}", year, day);
        std::process::exit(1);
    }
    
    let start = Instant::now();
    
    let solution = match year.as_str() {
        "2015" => get_solution(year, day, get_2015_solution),
        "2025" => get_solution(year, day, get_2025_solution),
        _ => {
            eprintln!("❌ Year {} not supported", year);
            std::process::exit(1);
        }
    };
    
    if let Some(sol) = solution {
        run_solution(Some(sol), part);
        let duration = start.elapsed();
        println!("\n⏱️  Completed in {:?}", duration);
    } else {
        eprintln!("❌ Solution not found for {} day {}", year, day);
        eprintln!("Run: cargo run --bin add_day {} {}", year, day);
        std::process::exit(1);
    }
}

fn get_solution<F>(year: &str, day: u8, getter: F) -> Option<Box<dyn Solution>>
where
    F: Fn(u8) -> Option<Box<dyn Solution>>,
{
    // Check if source file exists
    let source_path = format!("src/y{}/day{:02}.rs", year, day);
    if !Path::new(&source_path).exists() {
        return None;
    }
    getter(day)
}

fn run_solution(solution: Option<Box<dyn Solution>>, part: Option<&str>) {
    match solution {
        Some(sol) => {
            match part {
                Some("a") | Some("1") => {
                    println!("Part A: {}", sol.part_a());
                }
                Some("b") | Some("2") => {
                    println!("Part B: {}", sol.part_b());
                }
                _ => {
                    println!("Part A: {}", sol.part_a());
                    println!("Part B: {}", sol.part_b());
                }
            }
        }
        None => println!("Solution not implemented"),
    }
}

fn get_2015_solution(day: u8) -> Option<Box<dyn Solution>> {
    match day {
                4 => Some(Box::new(y2015::day04::Day04)),
                1 => Some(Box::new(y2015::day01::Day01)),
                2 => Some(Box::new(y2015::day02::Day02)),
                3 => Some(Box::new(y2015::day03::Day03)),
        _ => None,
    }
}

fn get_2025_solution(day: u8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(y2025::day01::Day01)),
        2 => Some(Box::new(y2025::day02::Day02)),
        3 => Some(Box::new(y2025::day03::Day03)),
        4 => Some(Box::new(y2025::day04::Day04)),
        5 => Some(Box::new(y2025::day05::Day05)),
        6 => Some(Box::new(y2025::day06::Day06)),
        7 => Some(Box::new(y2025::day07::Day07)),
        8 => Some(Box::new(y2025::day08::Day08)),
        9 => Some(Box::new(y2025::day09::Day09)),
                10 => Some(Box::new(y2025::day10::Day10)),
        _ => None,
    }
}
