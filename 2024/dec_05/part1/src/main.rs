use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn add_rules(rules_table: &mut HashMap<u32, Vec<u32>>, before: u32, after: u32) {
    if let Some(vec) = rules_table.get_mut(&before) {
        vec.push(after);
    } else {
        rules_table.insert(before, vec![after]);
    }
}

fn process_line(rules_table: &HashMap<u32, Vec<u32>>, line: String) -> u32 {
    let numbers: Vec<u32> = line.split(',').map(|n| n.parse().unwrap()).collect();
    let mut set: HashSet<u32> = HashSet::new();
    let mut broke_rules = false;
    for n in &numbers {
        if broke_rules {
            break;
        }
        let rules = rules_table.get(n);
        if let Some(unwraped_rules) = rules {
            for r in unwraped_rules {
                if set.contains(r) {
                    broke_rules = true;
                    break;
                }
            }
        }
        set.insert(*n);
    }

    if broke_rules {
        return 0;
    } else {
        let middle_number = numbers[numbers.len() / 2];
        return middle_number;
    }
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut reading_in_rules = true;
    let mut page_number_total: u32 = 0;

    let mut rules = 0;
    let mut updates = 0;

    let mut rules_table: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    reading_in_rules = false;
                } else if reading_in_rules {
                    let tuple: Vec<u32> = l
                        .split('|')
                        .map(|s| s.trim().parse::<u32>().unwrap())
                        .collect();
                    add_rules(&mut rules_table, tuple[0], tuple[1]);
                    rules += 1;
                } else {
                    page_number_total += process_line(&rules_table, l) as u32;
                    updates += 1;
                }
            }
            Err(e) => eprint!("error readingn in line {}", e),
        }
    }
    println!(
        "total pages: {}, updates:{} , rules:{}",
        page_number_total, updates, rules
    );

    Ok(())
}
