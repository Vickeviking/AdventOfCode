use std::collections::hash_map::HashMap;
use std::{
    char,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), io::Error> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut numbr_of_matches = 0;

    let mut board: Vec<String> = Vec::new();

    //read in all lines;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                board.push(l.chars().collect());
            }
            Err(e) => eprint!("Error reading in line {}", e),
        }
    }

    let num_cols = board[0].len();
    let columns: Vec<String> = (0..num_cols)
        .map(|col_index| {
            board
                .iter()
                .map(|row| row.chars().nth(col_index).unwrap())
                .collect::<String>()
        })
        .collect();

    let mut main_diagonals: HashMap<isize, String> = HashMap::new();
    let mut anti_diagonals: HashMap<usize, String> = HashMap::new();

    // Traverse the board
    for (row_index, row) in board.iter().enumerate() {
        for (col_index, char) in row.chars().enumerate() {
            let main_diag_index = row_index as isize - col_index as isize;
            let anti_diag_index = row_index + col_index;

            main_diagonals
                .entry(main_diag_index)
                .or_default()
                .push(char);
            anti_diagonals
                .entry(anti_diag_index)
                .or_default()
                .push(char);
        }
    }

    let main_diagonals_vec: Vec<String> = main_diagonals.into_iter().map(|(_, v)| v).collect();

    let anti_diagonals_vec: Vec<String> = anti_diagonals.into_iter().map(|(_, v)| v).collect();

    for row in board {
        numbr_of_matches += slide_over_string(&row);
    }
    for col in columns {
        numbr_of_matches += slide_over_string(&col);
    }
    for diagonal in main_diagonals_vec {
        numbr_of_matches += slide_over_string(&diagonal);
    }
    for diagonal in anti_diagonals_vec {
        numbr_of_matches += slide_over_string(&diagonal);
    }

    println!("in total {} matches!", numbr_of_matches);
    Ok(())
}

fn is_xmas(str: String) -> bool {
    return str == "XMAS" || str.chars().rev().collect::<String>() == "XMAS";
}

fn slide_over_string(board: &String) -> u16 {
    let str_len = board.len();
    if str_len < 4 {
        return 0;
    }
    let mut curr_index = 3;
    let mut total: u16 = 0;
    let mut window: String = board[..].chars().take(4).collect::<String>();

    if is_xmas(window.clone()) {
        total += 1;
    }

    while curr_index < str_len - 1 {
        curr_index += 1;
        window.push(
            *board
                .chars()
                .collect::<Vec<char>>()
                .get(curr_index)
                .unwrap(),
        );
        window.drain(..1);
        total += is_xmas(window.clone()) as u16;
    }

    return total;
}
