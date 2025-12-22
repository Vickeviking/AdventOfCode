use std::{
    collections::{BTreeMap, HashSet},
    fs, io,
};

#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd, Hash)]
struct Point(usize, usize); // x, y

#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq, PartialOrd, Hash)]
struct Square(Point, Point); //normalized, p1 < p2

impl Square {
    pub fn area(self) -> u64 {
        let dx = (self.1 .0 as i64 - self.0 .0 as i64).abs() + 1;
        let dy = (self.1 .1 as i64 - self.0 .1 as i64).abs() + 1;
        (dx as u64) * (dy as u64)
    }
}

fn part1(input: &str) -> u64 {
    let mut points: Vec<Point> = Vec::new();

    let normalize = |p1: Point, p2: Point| -> Square {
        if p1 < p2 {
            return Square(p1, p2);
        }
        Square(p2, p1)
    };

    for l in input.lines() {
        let p: Vec<usize> = l.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        points.push(Point(p[0], p[1]));
    }

    println!("points {:?}", points);

    let mut squares: BTreeMap<u64, HashSet<Square>> = BTreeMap::new();

    //add all set of normalized point tuples and their area
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            let s = normalize(*p1, *p2);
            let area = s.area();
            squares.entry(area).or_default().insert(s);
        }
    }

    squares.pop_last().unwrap().0
}

fn part2(input: &str) -> u64 {
    12
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?;

    let result_part1 = part1(&input);
    println!("Part 1: {:?}", result_part1);

    let result_part2 = part2(&input);
    println!("Part 2: {:?}", result_part2);

    Ok(())
}
