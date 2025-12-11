use std::{
    fmt,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

// ERROR
#[derive(Debug)]
pub enum GameError {
    PlayerNotFound,
    PlayerOutOfBounds,
    Obstacle,
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameError::PlayerNotFound => "Player not found in the grid",
                GameError::PlayerOutOfBounds => "Player out of grid",
                GameError::Obstacle => "Obstacle infront plaeyr",
            }
        )
    }
}

impl std::error::Error for GameError {}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Map {
    player: Player,
    grid: Vec<Vec<char>>,
    walkable_objects: Vec<char>,
}

struct Player {
    x: u32,
    y: u32,
    visited_tiles: u32,
    on_board: bool,
    direction: Direction,
}

impl Map {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let p = Player {
            x: 0,
            y: 0,
            visited_tiles: 0,
            on_board: true,
            direction: Direction::Up,
        };

        let mut m = Map {
            player: p,
            grid,
            walkable_objects: vec!['.', 'X'],
        };
        let _ = m.find_player();
        m
    }

    pub fn find_player(&mut self) -> Result<(), GameError> {
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, col) in row.iter().enumerate() {
                match col {
                    '^' => {
                        self.player.direction = Direction::Up;
                        self.player.x = col_index as u32;
                        self.player.y = row_index as u32;
                        self.grid[row_index][col_index] = 'X';
                        self.player.visited_tiles += 1;
                        return Ok(());
                    }
                    'v' => {
                        self.player.direction = Direction::Down;
                        self.player.x = col_index as u32;
                        self.player.y = row_index as u32;
                        self.grid[row_index][col_index] = 'X';
                        self.player.visited_tiles += 1;
                        return Ok(());
                    }
                    '<' => {
                        self.player.direction = Direction::Left;
                        self.player.x = col_index as u32;
                        self.player.y = row_index as u32;
                        self.grid[row_index][col_index] = 'X';
                        self.player.visited_tiles += 1;
                        return Ok(());
                    }
                    '>' => {
                        self.player.direction = Direction::Right;
                        self.player.x = col_index as u32;
                        self.player.y = row_index as u32;
                        self.grid[row_index][col_index] = 'X';
                        self.player.visited_tiles += 1;
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }

        Err(GameError::PlayerNotFound)
    }

    pub fn move_player(&mut self) -> Result<(), GameError> {
        let res: Result<(u32, u32), GameError> = self.player_can_move_forward();
        if let Ok((x, y)) = res {
            if self.grid[y as usize][x as usize] != 'X' {
                self.player.visited_tiles += 1;
                self.grid[y as usize][x as usize] = 'X';
            }
            self.player.x = x;
            self.player.y = y;
            return Ok(());
        } else if let Err(e) = res {
            match e {
                GameError::Obstacle => {
                    self.player.turn_right();
                    return Ok(());
                }
                GameError::PlayerOutOfBounds => {
                    return Err(GameError::PlayerOutOfBounds);
                }
                _ => return Err(e),
            }
        }
        Ok(())
    }

    pub fn player_can_move_forward(&mut self) -> Result<(u32, u32), GameError> {
        let res: Result<(u32, u32), GameError> = self.player_next_step();
        if let Ok((x, y)) = res {
            let next_tile = self.grid[y as usize][x as usize];
            if self.walkable_objects.contains(&next_tile) {
                return Ok((x, y));
            } else {
                return Err(GameError::Obstacle);
            }
        }
        return res;
    }

    pub fn player_next_step(&mut self) -> Result<(u32, u32), GameError> {
        let mut x: i32 = self.player.x as i32;
        let mut y: i32 = self.player.y as i32;

        match self.player.direction {
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
            Direction::Down => y += 1,
            Direction::Up => y -= 1,
        }

        if x < 0 || y < 0 || x >= self.grid[0].len() as i32 || y >= self.grid.len() as i32 {
            return Err(GameError::PlayerOutOfBounds);
        }

        Ok((x as u32, y as u32))
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = (self.player.x as usize, self.player.y as usize);

        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, &col) in row.iter().enumerate() {
                if row_index == y && col_index == x {
                    let player_char = match self.player.direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Right => '>',
                        Direction::Left => '<',
                    };
                    write!(f, "{}", player_char)?;
                } else {
                    write!(f, "{}", col)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Player {
    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => {
                self.direction = Direction::Right;
            }
            Direction::Down => {
                self.direction = Direction::Left;
            }
            Direction::Left => {
                self.direction = Direction::Up;
            }
            Direction::Right => {
                self.direction = Direction::Down;
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        match line {
            Ok(l) => {
                grid.push(l.chars().collect::<Vec<char>>());
            }
            Err(e) => eprintln!("error reading in file {}", e),
        }
    }

    let mut map = Map::new(grid);
    // move until out of bounds
    while let Ok(_) = map.move_player() {}

    println!("player touched {} unique tiles", map.player.visited_tiles);
    Ok(())
}
