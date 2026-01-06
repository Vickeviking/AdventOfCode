use crate::Solution;

pub struct Day12;

impl Solution for Day12 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day12.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day12.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        12
    }
}

fn rotate_enum(dir: Direction, rotation: i16) -> Direction {
    let direction_to_num = |d: Direction| -> isize {
        match d {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    };

    let num_to_direction = |num: isize| -> Direction {
        match num.rem_euclid(4) {
            // wraps negatives correctly
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!(),
        }
    };

    let new_num = direction_to_num(dir) + (rotation as isize / 90);
    num_to_direction(new_num)
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn solve_part_a(input: &str) -> i16 {
    let mut facing = Direction::East;
    let mut x: i16 = 0;
    let mut y: i16 = 0;

    for i in input.lines() {
        let mut iter = i.chars();
        let action = iter.next().unwrap();
        let arg = iter.collect::<String>().parse::<i16>().unwrap();

        match action {
            'N' => y += arg,
            'S' => y -= arg,
            'E' => x += arg,
            'W' => x -= arg,
            'L' => facing = rotate_enum(facing, -arg),
            'R' => facing = rotate_enum(facing, arg),
            'F' => match facing {
                Direction::North => y += arg,
                Direction::East => x += arg,
                Direction::South => y -= arg,
                Direction::West => x -= arg,
            },
            _ => {}
        }
    }

    x.abs() + y.abs()
}

fn rotate(wx: i16, wy: i16, deg: i16) -> (i16, i16) {
    match deg.rem_euclid(360) {
        0 => (wx, wy),
        90 => (wy, -wx),
        180 => (-wx, -wy),
        270 => (-wy, wx),
        _ => unreachable!(),
    }
}

fn solve_part_b(input: &str) -> i16 {
    let mut wx: i16 = 10;
    let mut wy: i16 = 1;
    let mut ship_x: i16 = 0;
    let mut ship_y: i16 = 0;

    for i in input.lines() {
        let mut iter = i.chars();
        let action = iter.next().unwrap();
        let arg = iter.collect::<String>().parse::<i16>().unwrap();

        match action {
            'N' => wy += arg,
            'S' => wy -= arg,
            'E' => wx += arg,
            'W' => wx -= arg,
            'L' => {
                let (nx, ny) = rotate(wx, wy, -arg);
                wx = nx;
                wy = ny;
            }
            'R' => {
                let (nx, ny) = rotate(wx, wy, arg);
                wx = nx;
                wy = ny;
            }
            'F' => {
                // points from ship towards target
                ship_x += wx * arg;
                ship_y += wy * arg;
            }
            _ => {}
        }
    }
    ship_x.abs() + ship_y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "F10
N3
F7
R90
F11";
        assert_eq!(solve_part_a(input), 25);
    }

    #[test]
    fn test_part_b() {
        let input = "F10
N3
F7
R90
F11";
        assert_eq!(solve_part_b(input), 286);
    }
}
