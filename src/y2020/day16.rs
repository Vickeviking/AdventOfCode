use std::collections::HashMap;

use crate::Solution;

pub struct Day16;

impl Solution for Day16 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day16.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day16.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        16
    }
}

fn solve_part_a(input: &str) -> u16 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let field_section = sections[0];
    let nearby_tickets_block = sections[2];

    // start -> end, not normalized
    let mut ranges: Vec<(u16, u16)> = Vec::new();

    //parsing field_section for raw ranges
    for l in field_section.lines() {
        let range_text = l.split(":").nth(1).unwrap().trim();

        // go through all the ranges by the hashmap
        for r in range_text.split("or") {
            let range: Vec<u16> = r
                .trim()
                .split('-')
                .map(|n| n.parse::<u16>().unwrap())
                .collect();
            let start = range[0];
            let end = range[1];

            ranges.push((start, end));
        }
    }

    // normalize, we need to sort and merge ranges
    // defines what the "key" is by the return of a closure given obj stored
    ranges.sort_by_key(|(start, _)| *start);
    let mut merged: Vec<(u16, u16)> = Vec::new();
    for (start, end) in ranges {
        if let Some((_, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
                continue; // we dont add start, and conditionaly update the last end
            }
        }
        // start is not contained in previous range
        merged.push((start, end));
    }

    // parse nearby tickets
    let nearby: Vec<u16> = nearby_tickets_block
        .lines()
        .skip(1)
        .flat_map(|line| line.split(',').map(|n| n.parse::<u16>().unwrap()))
        .collect();

    // how many of nearby is not in merged?
    let mut sum = 0;
    for n in nearby {
        // find start before n, if connected end is after n its valid else not
        let mut valid = false;

        for (start, end) in &merged {
            // om start är efter n så är vi utanför intervall
            if *start > n {
                break;
            }

            // start <= n OM end är mer eller lika med n , så är vi valid
            if *end >= n {
                valid = true;
                break;
            }
        }

        if !valid {
            sum += n;
        }
    }

    sum
}

fn solve_part_b(input: &str) -> i64 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let field_section = sections[0];
    let my_ticket_block = sections[1];
    let nearby_tickets_block = sections[2];

    // start -> end, entire raneg of valid tickets
    let mut ranges: Vec<(u16, u16)> = Vec::new();
    let mut field_ranges: HashMap<&str, Vec<(u16, u16)>> = HashMap::new();

    //parsing field sections
    for l in field_section.lines() {
        let parts: Vec<&str> = l.split(":").collect();
        let field = parts[0].trim();
        let range_text = parts[1];

        // go through all the ranges by the hashmap
        for r in range_text.split("or") {
            let range: Vec<u16> = r
                .trim()
                .split('-')
                .map(|n| n.parse::<u16>().unwrap())
                .collect();
            let start = range[0];
            let end = range[1];

            ranges.push((start, end));

            field_ranges
                .entry(field)
                .and_modify(|v| v.push((start, end)))
                .or_insert(vec![(start, end)]);
        }
    }

    // normalize, we need to sort and merge ranges
    ranges.sort_by_key(|(start, _)| *start);
    let mut normalised_ranges: Vec<(u16, u16)> = Vec::new();
    for (start, end) in ranges {
        if let Some((_, last_end)) = normalised_ranges.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
                continue; // we dont add start, and conditionaly update the last end
            }
        }
        // start is not contained in previous range
        normalised_ranges.push((start, end));
    }

    let my_tickets: Vec<u16> = my_ticket_block
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u16>().unwrap())
        .collect();

    let mut nearby_tickets: Vec<Vec<u16>> = nearby_tickets_block
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u16>().unwrap())
                .collect::<Vec<u16>>()
        })
        .filter(|line| {
            // Filters out invalid ones, where a ticket, has a number not in one of the sub ranges
            // in normalised_ranges
            // find start before n, if connected end is after n its valid else not

            for n in line {
                let mut valid = false;
                for (start, end) in &normalised_ranges {
                    // om start är efter n så är vi utanför intervall
                    if start > n {
                        break;
                    }

                    // start <= n OM end är mer eller lika med n , så är vi valid
                    if end >= n {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    return false;
                }
            }

            // if here the line was true
            true
        })
        .collect();

    // with move semantics, no new alloc should happen
    let mut all_tickets = nearby_tickets;
    all_tickets.push(my_tickets.clone());

    // transpose the ticket , so we have column first
    let n_cols = all_tickets[0].len(); // assuming all tickets have same length

    let columns: Vec<Vec<u16>> = (0..n_cols)
        .map(|col_idx| all_tickets.iter().map(|row| row[col_idx]).collect())
        .collect();

    // binding ticket_id referencing an index in `all_tickets` to potential `field_id` that in turn
    // references its valid ranges

    // initial candidates: column idx -> possible fields
    let mut candidates: HashMap<usize, Vec<&str>> = (0..n_cols)
        .map(|i| (i, field_ranges.keys().copied().collect()))
        .collect();

    // resolved mapping: field -> column
    let mut field_to_col: HashMap<&str, usize> = HashMap::new();

    loop {
        let mut made_change = false;

        for (col_idx, col) in columns.iter().enumerate() {
            let candidates_vec = candidates.get_mut(&col_idx).unwrap();

            // Remove candidates that are invalid for this column
            candidates_vec.retain(|&cand| {
                let ranges = &field_ranges[cand];
                col.iter().all(|&n| {
                    (ranges[0].0..=ranges[0].1).contains(&n)
                        || (ranges[1].0..=ranges[1].1).contains(&n)
                })
            });

            // If this column is now resolved, record mapping and remove from other columns
            if candidates_vec.len() == 1 {
                let resolved_field = candidates_vec[0];
                if !field_to_col.contains_key(resolved_field) {
                    field_to_col.insert(resolved_field, col_idx);

                    for other_idx in 0..n_cols {
                        if other_idx == col_idx {
                            continue;
                        }
                        candidates
                            .get_mut(&other_idx)
                            .unwrap()
                            .retain(|&f| f != resolved_field);
                    }

                    made_change = true;
                }
            }
        }

        if !made_change {
            break;
        }
    }

    field_to_col
        .iter()
        .filter(|(field, _)| field.starts_with("departure"))
        .map(|(_, &col_idx)| my_tickets[col_idx] as i64)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(solve_part_a(input), 71);
    }

    #[test]
    fn test_part_b() {}
}
