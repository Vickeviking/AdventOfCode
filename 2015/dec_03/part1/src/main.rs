use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Cords {
    x: i32,
    y: i32,
}

pub struct SantaMap {
    visited_tiles: HashSet<Cords>,
    current_tile: Cords,
}

impl SantaMap {
    pub fn new() -> Self {
        let mut sm = SantaMap {
            visited_tiles: HashSet::new(),
            current_tile: Cords { x: 0, y: 0 },
        };
        sm.visited_tiles.insert(sm.current_tile.clone());
        sm
    }
}

pub fn visit_tiles_in_line(str: String, santa_map: &mut SantaMap) -> io::Result<()> {
    let mut rev_str: String = str.chars().rev().collect();
    while let Some(c) = rev_str.pop() {
        match c {
            '>' => santa_map.current_tile.x += 1,
            '<' => santa_map.current_tile.x -= 1,
            '^' => santa_map.current_tile.y += 1,
            'v' => santa_map.current_tile.y -= 1,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid direction",
                ))
            }
        };
        let _ = santa_map
            .visited_tiles
            .insert(santa_map.current_tile.clone());
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut santa_map = SantaMap::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                visit_tiles_in_line(l, &mut santa_map)?;
            }
            Err(e) => eprintln!("error reading line {}", e),
        }
    }
    println!("santa visited {} houses", santa_map.visited_tiles.len());
    Ok(())
}
