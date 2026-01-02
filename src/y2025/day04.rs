use crate::Solution;

pub struct Day04;

impl Solution for Day04 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2025/day04.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2025/day04.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        4
    }
}

fn solve_part_a(input: &str) -> u32 {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let mut rolls = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == b'@' {
                let mut adjacent_rolls = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }

                        let is_roll = grid
                            .get((y as isize + dy) as usize)
                            .and_then(|row| row.get((x as isize + dx) as usize))
                            .map_or(false, |&cell| cell == b'@');

                        if is_roll {
                            adjacent_rolls += 1;
                        }
                    }
                }

                if adjacent_rolls < 4 {
                    rolls += 1;
                }
            }
        }
    }

    rolls
}

fn solve_part_b(input: &str) -> u32 {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let mut rolls = 0;
    let mut could_remove = true;

    while could_remove {
        let mut rm_y = 0;
        let mut rm_x = 0;
        could_remove = false;
        
        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == b'@' {
                    let mut adjacent_rolls = 0;

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }

                            let is_roll = grid
                                .get((y as isize + dy) as usize)
                                .and_then(|row| row.get((x as isize + dx) as usize))
                                .map_or(false, |&cell| cell == b'@');

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
