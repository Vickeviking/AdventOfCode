use std::{
    char,
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn calculate_sum(lines: Vec<String>) -> u64 {
    let mut map: HashSet<(usize, usize)> = HashSet::new();
    let mut start_pos: usize = 0;
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '^' => {
                    map.insert((x, y));
                }
                'S' => start_pos = x,
                _ => {}
            };
        }
    }
    let heigth = lines.len();
    let width = lines[0].len();
    let mut next_row: Vec<u64> = vec![0u64; width];
    next_row[start_pos] = 1;

    //go row for row and flip switches in next
    for y in 0..(heigth) {
        for x in 0..width {
            if next_row[x] > 0 && map.contains(&(x, y)) {
                println!("Happened");
                // splitting
                next_row[x - 1] += next_row[x];
                next_row[x + 1] += next_row[x];
                next_row[x] = 0; //no beam here since its a splitter here
                                 //counts as one split
            }
        }
    }

    println!("{:?}", next_row);
    // sum up next_row
    next_row.iter().sum::<u64>()
}

fn main() -> io::Result<()> {
    let file = File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = io::BufReader::new(file);

    let lines = reader
        .lines()
        .map(|x| x.expect("Valid strings due to proj desc"))
        .collect::<Vec<String>>();

    let total_sum = calculate_sum(lines);

    println!("{:?}", total_sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string(),
        ];

        assert_eq!(calculate_sum(input), 40);
    }
}
