use std::{
    fs::File,
    io::{self, BufRead},
};

// Roll can be accessed if less than 4 rolls of paper in the eight adjacent positions

fn accessable_rolls(lines: Vec<String>) -> u32 {
    let mut grid: Vec<Vec<u8>> = lines.iter().map(|l| l.as_bytes().to_vec()).collect();

    let mut rolls = 0;

    let mut could_remove = true;

    while could_remove {
        let mut rm_y = 0;
        let mut rm_x = 0;
        could_remove = false;
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == b'@' {
                    //cell is relevant for removal

                    let mut adjacent_rolls = 0;

                    //find number of adjacent_rolls
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }

                            let is_roll = grid
                                .get((y as isize + dy) as usize)
                                .and_then(|row| row.get((x as isize + dx) as usize))
                                .and_then(|&cell| if cell == b'@' { Some(true) } else { None })
                                .unwrap_or(false);

                            if is_roll {
                                adjacent_rolls += 1;
                            }
                        }
                    }

                    if adjacent_rolls < 4 {
                        rolls += 1;
                        could_remove = true;
                        rm_y = y;
                        rm_x = x;
                        break;
                    }
                }
                if could_remove {
                    break;
                }
            }
            if could_remove {
                break;
            }
        }

        if could_remove {
            grid[rm_y][rm_x] = b'.';
        }
    }

    rolls
}

fn main() -> io::Result<()> {
    let file = File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = io::BufReader::new(file);

    let lines = reader
        .lines()
        .map(|x| x.expect("Valid strings due to proj desc"))
        .collect::<Vec<String>>();

    println!("sum is: {}", accessable_rolls(lines));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
