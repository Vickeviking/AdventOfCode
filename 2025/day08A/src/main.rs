/// Not my solution, https://github.com/myleshyson/advent-of-code/tree/main/2025
use std::collections::{BTreeMap, HashMap};

use disjoint::DisjointSetVec;
use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd)]
struct Box(u64, u64, u64);

fn part1(input: &str) -> u64 {
    // Parsing into boxes
    let boxes: Vec<Box> = input
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line
                .split(',')
                .map(|num| num.parse().expect("invalid num"))
                .collect();
            Box(nums[0], nums[1], nums[2])
        })
        .collect();

    //disjoint set, using indexes to identifiying the "represented" box,
    //like having a tree storing keys, then used to index vec to retrieve
    let mut verticies: DisjointSetVec<Box> = DisjointSetVec::from(boxes.clone());

    let distance = |box1: &Box, box2: &Box| {
        box1.0.abs_diff(box2.0).pow(2)
            + box1.1.abs_diff(box2.1).pow(2)
            + box1.2.abs_diff(box2.2).pow(2)
    };

    let mut distances: BTreeMap<u64, (usize, usize)> = BTreeMap::new();

    for (i, box1) in boxes.iter().enumerate() {
        for (j, box2) in boxes.iter().enumerate() {
            if i == j {
                continue;
            }
            let box_distance = distance(box1, box2);
            distances.entry(box_distance).or_insert((i, j));
        }
    }

    let mut count = 0;
    for (_, (i, j)) in distances.iter() {
        verticies.join(*i, *j);
        count += 1;

        if count == 1000 {
            break;
        }
    }

    let mut set_sizes: HashMap<usize, u64> = HashMap::new();

    for i in 0..verticies.len() {
        let root = verticies.root_of(i);
        *set_sizes.entry(root).or_insert(0) += 1;
    }

    set_sizes
        .iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .take(3)
        .map(|(_, count)| count)
        .product()
}

fn part2(input: &str) -> u64 {
    let boxes: Vec<Box> = input
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line
                .split(',')
                .map(|num| num.parse().expect("invalid num"))
                .collect();
            Box(nums[0], nums[1], nums[2])
        })
        .collect();

    let mut verticies: DisjointSetVec<Box> = DisjointSetVec::from(boxes.clone());

    let distance = |box1: &Box, box2: &Box| {
        box1.0.abs_diff(box2.0).pow(2)
            + box1.1.abs_diff(box2.1).pow(2)
            + box1.2.abs_diff(box2.2).pow(2)
    };

    let mut distances: BTreeMap<u64, (usize, usize)> = BTreeMap::new();

    for (i, box1) in boxes.iter().enumerate() {
        for (j, box2) in boxes.iter().enumerate() {
            if i == j {
                continue;
            }
            let box_distance = distance(box1, box2);
            distances.entry(box_distance).or_insert((i, j));
        }
    }

    let mut joined: [(usize, usize); 1] = [(0, 1)];

    for (_, (i, j)) in distances.iter() {
        if verticies.join(*i, *j) {
            joined[0] = (*i, *j);
        }
    }

    boxes[joined[0].0].0 * boxes[joined[0].1].0
}

fn main() -> io::Result<()> {
    let file = File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = io::BufReader::new(file);

    let input: String = reader
        .lines()
        .map(|x| x.expect("Valid strings due to proj desc"))
        .collect::<Vec<String>>()
        .join("\n");

    let result_part1 = part1(&input);
    println!("Part 1: {:?}", result_part1);

    let result_part2 = part2(&input);
    println!("Part 2: {:?}", result_part2);

    Ok(())
}
