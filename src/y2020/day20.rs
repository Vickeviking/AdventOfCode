use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::Solution;

pub struct Day20;

impl Solution for Day20 {
    fn part_a(&self) -> String {
        let input = include_str!("../../inputs/2020/day20.txt");
        solve_part_a(input).to_string()
    }

    fn part_b(&self) -> String {
        let input = include_str!("../../inputs/2020/day20.txt");
        solve_part_b(input).to_string()
    }

    fn day(&self) -> u8 {
        20
    }
}

#[derive(Clone, Debug)]
struct Tile {
    id: u32,
    grid: Vec<Vec<char>>,
}

impl Tile {
    fn get_edge(&self, side: usize) -> String {
        match side {
            0 => self.grid[0].iter().collect(), // top
            1 => self.grid.iter().map(|row| row[row.len() - 1]).collect(), // right
            2 => self.grid[self.grid.len() - 1].iter().collect(), // bottom
            3 => self.grid.iter().map(|row| row[0]).collect(), // left
            _ => panic!("Invalid side"),
        }
    }

    fn get_edges(&self) -> Vec<String> {
        (0..4).map(|i| self.get_edge(i)).collect()
    }

    fn rotate_right(&mut self) {
        let n = self.grid.len();
        let mut new_grid = vec![vec!['.'; n]; n];
        for i in 0..n {
            for j in 0..n {
                new_grid[j][n - 1 - i] = self.grid[i][j];
            }
        }
        self.grid = new_grid;
    }

    fn flip_horizontal(&mut self) {
        for row in &mut self.grid {
            row.reverse();
        }
    }

    fn flip_vertical(&mut self) {
        self.grid.reverse();
    }

    fn remove_border(&self) -> Vec<Vec<char>> {
        let n = self.grid.len();
        self.grid[1..n - 1]
            .iter()
            .map(|row| row[1..n - 1].to_vec())
            .collect()
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let header = lines[0].trim();

        let id = header
            .strip_prefix("Tile ")
            .ok_or("missing Tile prefix")?
            .strip_suffix(":")
            .ok_or("missing : suffix")?
            .parse::<u32>()
            .map_err(|_| "invalid id")?;

        let grid: Vec<Vec<char>> = lines[1..]
            .iter()
            .map(|line| line.chars().collect())
            .collect();

        Ok(Tile { id, grid })
    }
}

fn solve_part_a(input: &str) -> u64 {
    let tiles: Vec<Tile> = input
        .split("\n\n")
        .map(|s| s.parse::<Tile>().unwrap())
        .collect();

    let mut tile_match_counts: HashMap<u32, usize> = HashMap::new();

    for tile in &tiles {
        let mut matching_edges = 0;
        let edges = tile.get_edges();

        for edge in &edges {
            let edge_rev: String = edge.chars().rev().collect();

            for other_tile in &tiles {
                if other_tile.id == tile.id {
                    continue;
                }

                let other_edges = other_tile.get_edges();
                for other_edge in &other_edges {
                    if edge == other_edge || edge_rev == *other_edge {
                        matching_edges += 1;
                        break;
                    }
                }
            }
        }

        tile_match_counts.insert(tile.id, matching_edges);
    }

    let corner_tiles: Vec<u32> = tile_match_counts
        .iter()
        .filter(|(_, &count)| count == 2)
        .map(|(&id, _)| id)
        .collect();

    corner_tiles.iter().map(|&id| id as u64).product()
}

fn solve_part_b(input: &str) -> usize {
    let mut tiles: Vec<Tile> = input
        .split("\n\n")
        .map(|s| s.parse::<Tile>().unwrap())
        .collect();

    // Find grid size (square root of number of tiles)
    let grid_size = (tiles.len() as f64).sqrt() as usize;

    // Assemble the puzzle
    let arranged = arrange_tiles(&mut tiles, grid_size);

    // Build the full image (without borders)
    let image = build_image(&arranged);

    // Find sea monsters in all orientations
    let roughness = find_sea_monsters(image);

    roughness
}

fn arrange_tiles(tiles: &mut Vec<Tile>, grid_size: usize) -> Vec<Vec<Tile>> {
    let mut arranged: Vec<Vec<Option<Tile>>> = vec![vec![None; grid_size]; grid_size];
    let mut used = HashSet::new();

    if backtrack(tiles, &mut arranged, &mut used, 0, 0, grid_size) {
        arranged
            .into_iter()
            .map(|row| row.into_iter().map(|t| t.unwrap()).collect())
            .collect()
    } else {
        panic!("Could not arrange tiles");
    }
}

fn backtrack(
    tiles: &[Tile],
    arranged: &mut Vec<Vec<Option<Tile>>>,
    used: &mut HashSet<u32>,
    row: usize,
    col: usize,
    grid_size: usize,
) -> bool {
    if row == grid_size {
        return true;
    }

    let (next_row, next_col) = if col + 1 == grid_size {
        (row + 1, 0)
    } else {
        (row, col + 1)
    };

    for tile in tiles {
        if used.contains(&tile.id) {
            continue;
        }

        // Try all 8 orientations
        for orientation in 0..8 {
            let mut test_tile = tile.clone();
            apply_orientation(&mut test_tile, orientation);

            // Check if this tile fits
            let mut fits = true;

            // Check top neighbor
            if row > 0 {
                if let Some(ref top_tile) = arranged[row - 1][col] {
                    if test_tile.get_edge(0) != top_tile.get_edge(2) {
                        fits = false;
                    }
                }
            }

            // Check left neighbor
            if col > 0 {
                if let Some(ref left_tile) = arranged[row][col - 1] {
                    if test_tile.get_edge(3) != left_tile.get_edge(1) {
                        fits = false;
                    }
                }
            }

            if fits {
                arranged[row][col] = Some(test_tile);
                used.insert(tile.id);

                if backtrack(tiles, arranged, used, next_row, next_col, grid_size) {
                    return true;
                }

                arranged[row][col] = None;
                used.remove(&tile.id);
            }
        }
    }

    false
}

fn apply_orientation(tile: &mut Tile, orientation: usize) {
    match orientation {
        0 => {}
        1 => tile.rotate_right(),
        2 => {
            tile.rotate_right();
            tile.rotate_right();
        }
        3 => {
            tile.rotate_right();
            tile.rotate_right();
            tile.rotate_right();
        }
        4 => tile.flip_horizontal(),
        5 => {
            tile.flip_horizontal();
            tile.rotate_right();
        }
        6 => {
            tile.flip_horizontal();
            tile.rotate_right();
            tile.rotate_right();
        }
        7 => {
            tile.flip_horizontal();
            tile.rotate_right();
            tile.rotate_right();
            tile.rotate_right();
        }
        _ => {}
    }
}

fn build_image(arranged: &Vec<Vec<Tile>>) -> Vec<Vec<char>> {
    let grid_size = arranged.len();
    let tile_size = arranged[0][0].remove_border().len();
    let image_size = grid_size * tile_size;

    let mut image = vec![vec!['.'; image_size]; image_size];

    for (tile_row, row_tiles) in arranged.iter().enumerate() {
        for (tile_col, tile) in row_tiles.iter().enumerate() {
            let borderless = tile.remove_border();
            for (i, row) in borderless.iter().enumerate() {
                for (j, &ch) in row.iter().enumerate() {
                    image[tile_row * tile_size + i][tile_col * tile_size + j] = ch;
                }
            }
        }
    }

    image
}

fn find_sea_monsters(mut image: Vec<Vec<char>>) -> usize {
    let sea_monster = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];

    let monster_coords: Vec<(usize, usize)> = sea_monster
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect();

    // Try all 8 orientations
    for orientation in 0..8 {
        let count = count_sea_monsters(&image, &monster_coords);
        if count > 0 {
            // Found sea monsters, calculate roughness
            let total_hash = image
                .iter()
                .flat_map(|row| row.iter())
                .filter(|&&ch| ch == '#')
                .count();

            let monster_hash = monster_coords.len() * count;
            return total_hash - monster_hash;
        }

        // Try next orientation
        if orientation < 7 {
            image = rotate_image(image, orientation);
        }
    }

    0
}

fn count_sea_monsters(image: &Vec<Vec<char>>, monster_coords: &[(usize, usize)]) -> usize {
    let height = image.len();
    let width = image[0].len();
    let mut count = 0;

    for i in 0..height {
        for j in 0..width {
            if check_monster_at(image, i, j, monster_coords) {
                count += 1;
            }
        }
    }

    count
}

fn check_monster_at(
    image: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    monster_coords: &[(usize, usize)],
) -> bool {
    let height = image.len();
    let width = image[0].len();

    for &(di, dj) in monster_coords {
        let ni = row + di;
        let nj = col + dj;

        if ni >= height || nj >= width || image[ni][nj] != '#' {
            return false;
        }
    }

    true
}

fn rotate_image(image: Vec<Vec<char>>, orientation: usize) -> Vec<Vec<char>> {
    let n = image.len();
    let mut result = image.clone();

    match orientation {
        0 => {
            // Rotate right
            let mut new_img = vec![vec!['.'; n]; n];
            for i in 0..n {
                for j in 0..n {
                    new_img[j][n - 1 - i] = result[i][j];
                }
            }
            new_img
        }
        3 => {
            // Flip horizontal
            for row in &mut result {
                row.reverse();
            }
            result
        }
        _ => {
            // Rotate right
            let mut new_img = vec![vec!['.'; n]; n];
            for i in 0..n {
                for j in 0..n {
                    new_img[j][n - 1 - i] = result[i][j];
                }
            }
            new_img
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tile_from_string() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

        let tile = input.parse::<Tile>().unwrap();
        assert_eq!(tile.id, 2311);
        assert_eq!(tile.get_edge(0), "..##.#..#.");
        assert_eq!(tile.get_edge(1), "...#.##..#");
        assert_eq!(tile.get_edge(2), "..###..###");
        assert_eq!(tile.get_edge(3), ".#####..#.");
    }

    #[test]
    fn test_part_a() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        assert_eq!(solve_part_a(input), 20899048083289);
    }

    #[test]
    fn test_part_b() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        assert_eq!(solve_part_b(input), 273);
    }
}
