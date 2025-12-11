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

fn process_line(rules_table: &HashMap<u32, Vec<u32>>, line: String) -> Vec<u32> {
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
        numbers
    } else {
        vec![]
    }
}

fn fixLine(rules_table: &HashMap<u32, Vec<u32>>, line: Vec<u32>) -> u32 {
    //adjacency list
    let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut new_line: Vec<u32> = vec![];

    //for each number in line , insert n, vec![]
    for n in &line {
        adj_list.insert(*n, vec![]);
    }

    for n in &line {
        let rules: Option<&Vec<u32>> = rules_table.get(&n);
        if let Some(u_rules) = rules {
            for r in u_rules {
                if let Some(vec) = adj_list.get_mut(r) {
                    vec.push(*n)
                }
            }
        }
    }

    while !adj_list.is_empty() {
        let nodes_with_no_incoming: Vec<u32> = adj_list
            .iter()
            .filter_map(|(&node, neighbors)| {
                if neighbors.is_empty() {
                    Some(node)
                } else {
                    None
                }
            })
            .collect();

        for node in nodes_with_no_incoming {
            new_line.push(node);

            adj_list.remove(&node);

            for neighbors in adj_list.values_mut() {
                neighbors.retain(|&n| n != node);
            }
        }
    }

    return new_line[new_line.len() / 2];
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut reading_in_rules = true;
    let mut rules_table: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut error_lines: Vec<Vec<u32>> = vec![];

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
                } else {
                    let number_line: Vec<u32> = process_line(&rules_table, l);
                    if !number_line.is_empty() {
                        error_lines.push(number_line);
                    }
                }
            }
            Err(e) => eprint!("error readingn in line {}", e),
        }
    }

    let mut total_lines: u32 = 0;
    for l in error_lines {
        total_lines += fixLine(&rules_table, l);
    }

    println!("total number {}", total_lines);

    Ok(())
}
