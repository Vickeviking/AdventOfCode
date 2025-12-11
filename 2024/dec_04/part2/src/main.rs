use std::collections::hash_map::HashMap;
use std::{
    char,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn matches_pattern(board: &Vec<Vec<char>>, row: i16, col: i16) -> bool {
    let row = row as usize;
    let col = col as usize;

    // 'a' in the middle
    if board[row + 1][col + 1] != 'A' {
        return false;
    }

    if !(board[row][col] == 'S' || board[row][col] == 'M') {
        return false;
    }
    if !(board[row + 2][col] == 'S' || board[row + 2][col] == 'M') {
        return false;
    }
    if !(board[row][col + 2] == 'S' || board[row][col + 2] == 'M') {
        return false;
    }
    if !(board[row + 2][col + 2] == 'S' || board[row + 2][col + 2] == 'M') {
        return false;
    }

    if board[row][col] == board[row + 2][col + 2] {
        return false;
    }
    if board[row + 2][col] == board[row][col + 2] {
        return false;
    }

    return true;
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut numbr_of_matches: u32 = 0;

    let mut board: Vec<Vec<char>> = Vec::new();

    //read in all lines;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                board.push(l.chars().collect::<Vec<char>>());
            }
            Err(e) => eprint!("Error reading in line {}", e),
        }
    }

    for row_index in 0..(board.len() - 2) {
        for col_index in 0..(board[0].len() - 2) {
            numbr_of_matches += matches_pattern(&board, row_index as i16, col_index as i16) as u32;
        }
    }

    println!("you have {} matches", numbr_of_matches);
    Ok(())
}
