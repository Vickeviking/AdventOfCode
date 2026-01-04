use crate::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day03.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day03.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        3
    }
}

fn solve_part_a(input: &str) -> i32 {
    let map: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();

    // map meta data
    let width = map[0].len();
    let stepsize_x = 3;
    let stepsize_y = 1;

    //for varje line y, så går vi 3 höger

    let mut current_width = 0;
    let mut trees = 0;
    for (y, row) in map.iter().enumerate() {
        if y % stepsize_y != 0 {
            continue;
        }
        if map[y][current_width % width] == b'#' {
            trees += 1;
        }
        current_width += stepsize_x;
    }
    trees
}

fn solve_part_b(input: &str) -> u64 {
    let map: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();

    let check_slope = |stepsize_x: usize, stepsize_y: usize| {
        // map meta data
        let width = map[0].len();

        //for varje line y, så går vi 3 höger

        let mut current_width = 0;
        let mut trees = 0;
        for (y, row) in map.iter().enumerate() {
            if y % stepsize_y != 0 {
                continue;
            }
            if map[y][current_width % width] == b'#' {
                trees += 1;
            }
            current_width += stepsize_x;
        }
        trees
    };

    let a = check_slope(1, 1);
    let b = check_slope(3, 1);
    let c = check_slope(5, 1);
    let d = check_slope(7, 1);
    let e = check_slope(1, 2);

    a * b * c * d * e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(solve_part_a(input), 7);
    }

    #[test]
    fn test_part_b() {
        let input = "test input";
        assert_eq!(solve_part_b(input), 0);
    }
}
