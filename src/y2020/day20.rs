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
    edges: Vec<String>, // top, right, bottom, left
}

impl Tile {
    fn get_all_edge_variants(&self) -> Vec<String> {
        let mut variants = Vec::new();
        for edge in &self.edges {
            variants.push(edge.clone());
            variants.push(edge.chars().rev().collect());
        }
        variants
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

        let grid: Vec<&str> = lines[1..].to_vec();
        let height = grid.len();

        // Extract edges: top, right, bottom, left
        let top = grid[0].to_string();
        let bottom = grid[height - 1].to_string();

        let mut left = String::new();
        let mut right = String::new();
        for row in &grid {
            left.push(row.chars().next().unwrap());
            right.push(row.chars().last().unwrap());
        }

        Ok(Tile {
            id,
            edges: vec![top, right, bottom, left],
        })
    }
}

fn solve_part_a(input: &str) -> u64 {
    let tiles: Vec<Tile> = input
        .split("\n\n")
        .map(|s| s.parse::<Tile>().unwrap())
        .collect();

    // Build a map of all edge strings (and their reverses) to tile IDs
    let mut edge_map: HashMap<String, Vec<u32>> = HashMap::new();

    for tile in &tiles {
        for edge in tile.get_all_edge_variants() {
            edge_map.entry(edge).or_insert_with(Vec::new).push(tile.id);
        }
    }

    // Count how many edges of each tile match with other tiles
    let mut tile_match_counts: HashMap<u32, usize> = HashMap::new();

    for tile in &tiles {
        let mut matching_edges = 0;

        for edge in &tile.edges {
            // Check if this edge (or its reverse) matches another tile
            let edge_rev: String = edge.chars().rev().collect();

            let matches_forward = edge_map.get(edge).map(|v| v.len()).unwrap_or(0);
            let matches_reverse = edge_map.get(&edge_rev).map(|v| v.len()).unwrap_or(0);

            // If more than one tile has this edge (counting both orientations),
            // it means this edge matches with another tile
            // Subtract 1 because the tile matches with itself
            if matches_forward > 1 || matches_reverse > 1 {
                matching_edges += 1;
            }
        }

        tile_match_counts.insert(tile.id, matching_edges);
    }

    // Corner tiles have exactly 2 matching edges (and 2 unmatched edges)
    let corner_tiles: Vec<u32> = tile_match_counts
        .iter()
        .filter(|(_, &count)| count == 2)
        .map(|(&id, _)| id)
        .collect();

    corner_tiles.iter().map(|&id| id as u64).product()
}

fn solve_part_b(_input: &str) -> i32 {
    // TODO: Implement part B
    0
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
        assert_eq!(tile.edges[0], "..##.#..#."); // top
        assert_eq!(tile.edges[1], "...#.##..#"); // right
        assert_eq!(tile.edges[2], "..###..###"); // bottom
        assert_eq!(tile.edges[3], ".#####..#."); // left
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
        let input = "test input";
        assert_eq!(solve_part_b(input), 0);
    }
}
