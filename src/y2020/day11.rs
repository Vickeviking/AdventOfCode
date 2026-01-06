use crate::Solution;

pub struct Day11;

impl Solution for Day11 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day11.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day11.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        11
    }
}

fn first_seat_occupied(
    map: &[Vec<u8>],
    y0: usize,
    x0: usize,
    dy: isize,
    dx: isize,
    distance: Option<usize>,
) -> bool {
    let mut iteration = 1;
    loop {
        let y = y0 as isize + dy * iteration as isize;
        let x = x0 as isize + dx * iteration as isize;

        if y < 0 || x < 0 {
            break;
        }

        let (y, x) = (y as usize, x as usize);

        if y >= map.len() || x >= map[0].len() {
            break;
        }

        match map[y][x] {
            b'#' => return true,
            b'L' => return false,
            b'.' => {}
            _ => {}
        }

        if let Some(max_dist) = distance {
            if iteration >= max_dist {
                break;
            }
        }

        iteration += 1;
    }
    false
}

fn solve_part_a(input: &str) -> usize {
    let mut map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect();
    let mut next_map = map.clone(); // only one clone at start

    let directions = [
        (1, 0),   // S
        (-1, 0),  // N
        (0, 1),   // E
        (0, -1),  // W
        (1, 1),   // SE
        (1, -1),  // SW
        (-1, 1),  // NE
        (-1, -1), // NW
    ];

    loop {
        let mut changed = false;

        for (y, row) in map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == b'.' {
                    continue;
                }

                let mut free_seats = 0;

                for &(dy, dx) in &directions {
                    if !first_seat_occupied(&map, y, x, dy, dx, Some(1)) {
                        free_seats += 1;
                    }
                }

                next_map[y][x] = match c {
                    b'L' if free_seats == 8 => {
                        changed = true;
                        b'#'
                    }
                    b'#' if free_seats <= 4 => {
                        changed = true;
                        b'L'
                    }
                    _ => c,
                };
            }
        }

        if !changed {
            break;
        }

        std::mem::swap(&mut map, &mut next_map); // next_map becomes old map, map becomes new
    }

    map.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == b'#')
        .count()
}

fn solve_part_b(input: &str) -> usize {
    let mut map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect();
    let mut next_map = map.clone(); // only one clone at start

    let directions = [
        (1, 0),   // S
        (-1, 0),  // N
        (0, 1),   // E
        (0, -1),  // W
        (1, 1),   // SE
        (1, -1),  // SW
        (-1, 1),  // NE
        (-1, -1), // NW
    ];

    loop {
        let mut changed = false;

        for (y, row) in map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == b'.' {
                    continue;
                }

                let mut free_seats = 0;

                for &(dy, dx) in &directions {
                    if !first_seat_occupied(&map, y, x, dy, dx, None) {
                        free_seats += 1;
                    }
                }

                next_map[y][x] = match c {
                    b'L' if free_seats == 8 => {
                        changed = true;
                        b'#'
                    }
                    b'#' if free_seats <= 3 => {
                        changed = true;
                        b'L'
                    }
                    _ => c,
                };
            }
        }

        if !changed {
            break;
        }

        std::mem::swap(&mut map, &mut next_map); // next_map becomes old map, map becomes new
    }

    map.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == b'#')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(solve_part_a(input), 37);
    }

    #[test]
    fn test_part_b() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(solve_part_b(input), 26);
    }
}
