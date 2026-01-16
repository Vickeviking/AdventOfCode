use advent_of_code::Solution;
use std::env;
use std::path::Path;
use std::time::Instant;

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
    let part = if args.len() > 3 {
        Some(args[3].as_str())
    } else {
        None
    };

    // Check if year directory exists
    let year_dir = format!("src/y{}", year);
    if !Path::new(&year_dir).exists() {
        eprintln!("❌ Year {} not found. Run: cargo run --bin add_day {} {}", year, year, day);
        std::process::exit(1);
    }

    // Check if source file exists
    let source_path = format!("src/y{}/day{:02}.rs", year, day);
    if !Path::new(&source_path).exists() {
        eprintln!("❌ Solution not found for {} day {}", year, day);
        eprintln!("Run: cargo run --bin add_day {} {}", year, day);
        std::process::exit(1);
    }

    // Check if input file exists
    let input_path = format!("inputs/{}/day{:02}.txt", year, day);
    if !Path::new(&input_path).exists() {
        eprintln!("❌ Input file not found: {}", input_path);
        eprintln!("Create it or run: cargo run --bin add_day {} {}", year, day);
        std::process::exit(1);
    }

    let start = Instant::now();
    let solution = get_solution(year, day);

    run_solution(solution, part);
    let duration = start.elapsed();
    println!("\n⏱️  Completed in {:?}", duration);
}

fn get_solution(year: &str, day: u8) -> Box<dyn Solution> {
    // Dynamically construct the solution based on year and day
    match year {
        "2015" => match day {
            1 => Box::new(advent_of_code::y2015::day01::Day01),
            _ => panic!("Day {} not registered for year {}", day, year),
        },
        "2016" => match day {
            1 => Box::new(advent_of_code::y2016::day01::Day01),
            2 => Box::new(advent_of_code::y2016::day02::Day02),
            _ => panic!("Day {} not registered for year {}", day, year),
        },
        "2017" => match day {
            1 => Box::new(advent_of_code::y2017::day01::Day01),
            _ => panic!("Day {} not registered for year {}", day, year),
        },
        "2025" => match day {
            1 => Box::new(advent_of_code::y2025::day01::Day01),
            2 => Box::new(advent_of_code::y2025::day02::Day02),
            3 => Box::new(advent_of_code::y2025::day03::Day03),
            4 => Box::new(advent_of_code::y2025::day04::Day04),
            5 => Box::new(advent_of_code::y2025::day05::Day05),
            6 => Box::new(advent_of_code::y2025::day06::Day06),
            7 => Box::new(advent_of_code::y2025::day07::Day07),
            8 => Box::new(advent_of_code::y2025::day08::Day08),
            9 => Box::new(advent_of_code::y2025::day09::Day09),
            10 => Box::new(advent_of_code::y2025::day10::Day10),
            _ => panic!("Day {} not registered for year {}", day, year),
        },
        "2018" => match day {
            1 => Box::new(advent_of_code::y2018::day01::Day01),
                        2 => Box::new(advent_of_code::y2018::day02::Day02),
            _ => panic!("Day {} not registered for year {}", day, year),
        },
                "2019" => match day {
            1 => Box::new(advent_of_code::y2019::day01::Day01),
                        2 => Box::new(advent_of_code::y2019::day02::Day02),
            _ => panic!("Day {} not registered for year {}", day, year),
        },
                "2020" => match day {
            1 => Box::new(advent_of_code::y2020::day01::Day01),
                        2 => Box::new(advent_of_code::y2020::day02::Day02),
                        3 => Box::new(advent_of_code::y2020::day03::Day03),
                        4 => Box::new(advent_of_code::y2020::day04::Day04),
                        5 => Box::new(advent_of_code::y2020::day05::Day05),
                        6 => Box::new(advent_of_code::y2020::day06::Day06),
                        7 => Box::new(advent_of_code::y2020::day07::Day07),
                        8 => Box::new(advent_of_code::y2020::day08::Day08),
                        9 => Box::new(advent_of_code::y2020::day09::Day09),
                        10 => Box::new(advent_of_code::y2020::day10::Day10),
                        11 => Box::new(advent_of_code::y2020::day11::Day11),
                        12 => Box::new(advent_of_code::y2020::day12::Day12),
                        13 => Box::new(advent_of_code::y2020::day13::Day13),
                        14 => Box::new(advent_of_code::y2020::day14::Day14),
                        15 => Box::new(advent_of_code::y2020::day15::Day15),
                        16 => Box::new(advent_of_code::y2020::day16::Day16),
                        17 => Box::new(advent_of_code::y2020::day17::Day17),
                        18 => Box::new(advent_of_code::y2020::day18::Day18),
                        19 => Box::new(advent_of_code::y2020::day19::Day19),
                        20 => Box::new(advent_of_code::y2020::day20::Day20),
            _ => panic!("Day {} not registered for year {}", day, year),
        },
        _ => panic!("Year {} not registered", year),
    }
}

fn run_solution(solution: Box<dyn Solution>, part: Option<&str>) {
    match part {
        Some("a") | Some("1") => {
            println!("Part A: {}", solution.part_a());
        }
        Some("b") | Some("2") => {
            println!("Part B: {}", solution.part_b());
        }
        _ => {
            println!("Part A: {}", solution.part_a());
            println!("Part B: {}", solution.part_b());
        }
    }
}
