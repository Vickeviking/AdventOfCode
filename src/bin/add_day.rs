use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: cargo run --bin add_day <year> <day>");
        eprintln!("Example: cargo run --bin add_day 2025 10");
        process::exit(1);
    }
    
    let year = &args[1];
    let day: u8 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid day number");
        process::exit(1);
    });
    
    create_day_files(year, day).unwrap_or_else(|e| {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    });
    
    println!("✅ Created files for {} day {}", year, day);
    println!("   - src/y{}/day{:02}.rs", year, day);
    println!("   - inputs/{}/day{:02}.txt", year, day);
    println!("\nNext steps:");
    println!("   1. Add your input to inputs/{}/day{:02}.txt", year, day);
    println!("   2. Implement solve_part_a() and solve_part_b() in src/y{}/day{:02}.rs", year, day);
    println!("   3. Run: cargo run {} {} a", year, day);
}

fn create_day_files(year: &str, day: u8) -> Result<(), String> {
    // Create source file
    let src_dir = format!("src/y{}", year);
    let src_path = format!("{}/day{:02}.rs", src_dir, day);
    
    if Path::new(&src_path).exists() {
        return Err(format!("Source file already exists: {}", src_path));
    }
    
    fs::create_dir_all(&src_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    
    let template = format!(
r#"use crate::Solution;

pub struct Day{day:02};

impl Solution for Day{day:02} {{
    fn part_a(&self) -> String {{
        let input = include_str!("../../inputs/{year}/day{day:02}.txt");
        solve_part_a(input).to_string()
    }}

    fn part_b(&self) -> String {{
        let input = include_str!("../../inputs/{year}/day{day:02}.txt");
        solve_part_b(input).to_string()
    }}

    fn day(&self) -> u8 {{
        {day}
    }}
}}

fn solve_part_a(_input: &str) -> i32 {{
    // TODO: Implement part A
    0
}}

fn solve_part_b(_input: &str) -> i32 {{
    // TODO: Implement part B
    0
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_part_a() {{
        let input = "test input";
        assert_eq!(solve_part_a(input), 0);
    }}

    #[test]
    fn test_part_b() {{
        let input = "test input";
        assert_eq!(solve_part_b(input), 0);
    }}
}}
"#,
        year = year,
        day = day
    );
    
    fs::write(&src_path, template).map_err(|e| format!("Failed to write source file: {}", e))?;
    
    // Update mod.rs to include the new day
    let mod_path = format!("{}/mod.rs", src_dir);
    let mod_content = if Path::new(&mod_path).exists() {
        fs::read_to_string(&mod_path).map_err(|e| format!("Failed to read mod.rs: {}", e))?
    } else {
        String::new()
    };
    
    let day_mod = format!("pub mod day{:02};", day);
    if !mod_content.contains(&day_mod) {
        let mut lines: Vec<String> = mod_content.lines().map(String::from).collect();
        lines.push(day_mod);
        lines.sort();
        let new_content = lines.join("\n") + "\n";
        fs::write(&mod_path, new_content).map_err(|e| format!("Failed to update mod.rs: {}", e))?;
    }
    
    // Create input file
    let input_dir = format!("inputs/{}", year);
    let input_path = format!("{}/day{:02}.txt", input_dir, day);
    
    fs::create_dir_all(&input_dir).map_err(|e| format!("Failed to create input directory: {}", e))?;
    
    if !Path::new(&input_path).exists() {
        fs::write(&input_path, "").map_err(|e| format!("Failed to create input file: {}", e))?;
    }
    
    // Update main.rs get_YEAR_solution function
    update_main_rs(year, day)?;
    
    Ok(())
}

fn update_main_rs(year: &str, day: u8) -> Result<(), String> {
    let main_path = "src/main.rs";
    let content = fs::read_to_string(main_path)
        .map_err(|e| format!("Failed to read main.rs: {}", e))?;
    
    let func_name = format!("fn get_{}_solution(day: u8)", year);
    
    // Find the function and check if day is already added
    if let Some(func_start) = content.find(&func_name) {
        let func_section = &content[func_start..];
        let case_str = format!("{} => Some(Box::new(y{}::day{:02}::Day{:02}))", day, year, day, day);
        
        if func_section.contains(&case_str) {
            return Ok(()); // Already added
        }
        
        // Find the end of the match statement (before the last _ => None)
        if let Some(default_case_pos) = func_section.find("_ => None,") {
            let insert_pos = func_start + default_case_pos;
            let new_case = format!("        {} => Some(Box::new(y{}::day{:02}::Day{:02})),\n        ", day, year, day, day);
            let new_content = format!("{}{}{}", &content[..insert_pos], new_case, &content[insert_pos..]);
            
            fs::write(main_path, new_content)
                .map_err(|e| format!("Failed to update main.rs: {}", e))?;
        }
    }
    
    Ok(())
}
