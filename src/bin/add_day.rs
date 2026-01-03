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
    
    // Check if this is a new year
    let is_new_year = !Path::new(&src_dir).exists();
    
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
    
    // If new year, update lib.rs to add the module declaration
    if is_new_year {
        update_lib_rs(year)?;
    }
    
    // Update main.rs to register the solution
    update_main_rs(year, day)?;
    
    Ok(())
}

fn update_lib_rs(year: &str) -> Result<(), String> {
    let lib_path = "src/lib.rs";
    let content = fs::read_to_string(lib_path)
        .map_err(|e| format!("Failed to read lib.rs: {}", e))?;
    
    let year_mod = format!("pub mod y{};", year);
    
    // Check if already declared
    if content.contains(&year_mod) {
        return Ok(());
    }
    
    // Find where to insert (after other pub mod declarations)
    let lines: Vec<&str> = content.lines().collect();
    let mut new_lines = Vec::new();
    let mut inserted = false;
    
    for line in lines {
        new_lines.push(line.to_string());
        // Insert after the last pub mod line
        if line.starts_with("pub mod y") && !inserted {
            // Keep collecting pub mod lines
            continue;
        } else if !line.starts_with("pub mod") && !inserted && new_lines.iter().any(|l| l.starts_with("pub mod")) {
            // We've passed all pub mod lines, insert before this line
            new_lines.insert(new_lines.len() - 1, year_mod.clone());
            inserted = true;
        }
    }
    
    // If we didn't insert yet, append at the end
    if !inserted {
        new_lines.push(year_mod);
    }
    
    // Sort the pub mod lines
    let mut mod_lines: Vec<String> = new_lines.iter()
        .filter(|l| l.starts_with("pub mod y"))
        .map(|s| s.to_string())
        .collect();
    mod_lines.sort();
    
    // Rebuild content
    let other_lines: Vec<String> = new_lines.iter()
        .filter(|l| !l.starts_with("pub mod y"))
        .map(|s| s.to_string())
        .collect();
    
    let final_content = other_lines.join("\n") + "\n" + &mod_lines.join("\n") + "\n";
    
    fs::write(lib_path, final_content)
        .map_err(|e| format!("Failed to update lib.rs: {}", e))?;
    
    Ok(())
}

fn update_main_rs(year: &str, day: u8) -> Result<(), String> {
    let main_path = "src/main.rs";
    let content = fs::read_to_string(main_path)
        .map_err(|e| format!("Failed to read main.rs: {}", e))?;
    
    // Check if the year case exists
    let year_case = format!("\"{}\" => match day {{", year);
    
    if !content.contains(&year_case) {
        // Need to add the entire year case
        return add_year_to_main(year, day);
    }
    
    // Year exists, check if day is already registered
    let day_case = format!("{} => Box::new(advent_of_code::y{}::day{:02}::Day{:02})", day, year, day, day);
    if content.contains(&day_case) {
        return Ok(()); // Already registered
    }
    
    // Find the year's match block and add the day
    let year_start = content.find(&year_case).unwrap();
    let after_year = &content[year_start..];
    
    // Find the year's panic line - it uses format parameters, not the literal year
    let panic_pattern = "_ => panic!(\"Day {} not registered for year {}\", day, year)";
    if let Some(panic_pos) = after_year.find(panic_pattern) {
        let insert_pos = year_start + panic_pos;
        let new_day = format!("            {} => Box::new(advent_of_code::y{}::day{:02}::Day{:02}),\n            ", day, year, day, day);
        let new_content = format!("{}{}{}", &content[..insert_pos], new_day, &content[insert_pos..]);
        
        fs::write(main_path, new_content)
            .map_err(|e| format!("Failed to update main.rs: {}", e))?;
    }
    
    Ok(())
}

fn add_year_to_main(year: &str, day: u8) -> Result<(), String> {
    let main_path = "src/main.rs";
    let content = fs::read_to_string(main_path)
        .map_err(|e| format!("Failed to read main.rs: {}", e))?;
    
    // Find where to insert the new year (before the final _ => panic!)
    let final_panic = "_ => panic!(\"Year {} not registered\", year)";
    
    if let Some(insert_pos) = content.find(final_panic) {
        let new_year_case = format!(
r#"        "{}" => match day {{
            {} => Box::new(advent_of_code::y{}::day{:02}::Day{:02}),
            _ => panic!("Day {{}} not registered for year {{}}", day, year),
        }},
        "#,
            year, day, year, day, day
        );
        
        let new_content = format!("{}{}{}", &content[..insert_pos], new_year_case, &content[insert_pos..]);
        
        fs::write(main_path, new_content)
            .map_err(|e| format!("Failed to update main.rs: {}", e))?;
    }
    
    Ok(())
}
