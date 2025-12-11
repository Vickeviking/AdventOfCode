use std::{
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

// ERROR
#[derive(Debug, PartialEq)]
pub enum GameError {
    PlayerNotFound,
    PlayerOutOfBounds,
    Obstacle,
    StuckInLoop,
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
                GameError::StuckInLoop => "Player stuck in loop",
            }
        )
    }
}

impl std::error::Error for GameError {}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone)]
struct Map {
    player: Player,
    original_grid: Vec<Vec<char>>,
    grid: Vec<Vec<char>>,
    walkable_objects: Vec<char>,
    placed_obstacles: Vec<(u32, u32)>,
}

#[derive(Clone)]
struct Player {
    x: u32,
    y: u32,
    visited_tiles: u32,
    direction: Direction,
}

impl Map {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let p = Player {
            x: 0,
            y: 0,
            visited_tiles: 0,
            direction: Direction::Up,
        };

        let mut m = Map {
            player: p,
            original_grid: grid.clone(),
            grid,
            walkable_objects: vec!['.', 'h', 'j', 'k', 'l'],
            placed_obstacles: vec![],
        };
        let _ = m.find_player();
        m
    }

    pub fn reset_map(&mut self) {
        self.grid = self.original_grid.clone();
        let _ = self.find_player();
    }

    pub fn direction_to_char(&mut self) -> char {
        let c;
        match self.player.direction {
            Direction::Left => c = 'h',
            Direction::Right => c = 'l',
            Direction::Down => c = 'j',
            Direction::Up => c = 'k',
        }
        return c;
    }

    pub fn find_player(&mut self) -> Result<(), GameError> {
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, col) in row.iter().enumerate() {
                match col {
                    '^' => {
                        self.player.direction = Direction::Up;
                        self.player.x = col_index as u32;
                        self.player.y = row_index as u32;
                        self.grid[row_index][col_index] = 'k';
                        self.player.visited_tiles += 1;
                        return Ok(());
                    }
                    'v' => {
                        self.player.direction = Direction::Down;
                        self.player.x = col_index as u32;
                        self.player.y = row_index as u32;
                        self.grid[row_index][col_index] = 'j';
                        self.player.visited_tiles += 1;
                        return Ok(());
                    }
                    '<' => {
                        self.player.direction = Direction::Left;
                        self.player.x = col_index as u32;
                        self.player.y = row_index as u32;
                        self.grid[row_index][col_index] = 'h';
                        self.player.visited_tiles += 1;
                        return Ok(());
                    }
                    '>' => {
                        self.player.direction = Direction::Right;
                        self.player.x = col_index as u32;
                        self.player.y = row_index as u32;
                        self.grid[row_index][col_index] = 'l';
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
        //is it possible adding object in front of us?
        let res: Result<(u32, u32), GameError> = self.player_can_move_forward();
        if let Ok((x, y)) = res {
            if self.grid[y as usize][x as usize] == self.direction_to_char() {
                return Err(GameError::StuckInLoop);
            }
            self.grid[y as usize][x as usize] = self.direction_to_char();
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
            } else if next_tile == self.direction_to_char() {
                return Err(GameError::StuckInLoop);
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

fn run_sim(map: &mut Map) -> Result<(), GameError> {
    loop {
        let res = map.move_player();
        if let Ok(_) = res {
        } else if let Err(e) = res {
            match e {
                GameError::StuckInLoop => return Ok(()),
                _ => {
                    break;
                }
            }
        }
    }
    Err(GameError::PlayerNotFound)
}

fn main() -> Result<(), io::Error> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut loops: u32 = 0;
    let mut grid: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        match line {
            Ok(l) => {
                grid.push(l.chars().collect::<Vec<char>>());
            }
            Err(e) => eprintln!("error reading in file {}", e),
        }
    }

    let total = grid.len() * grid[0].len();
    let mut iter = 0;

    let map = Map::new(grid.clone());
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            iter += 1;
            if map.walkable_objects.contains(col) {
                //possible to place object
                let mut map_clone = map.clone();
                map_clone.grid[row_index][col_index] = '#';
                if let Ok(_) = run_sim(&mut map_clone) {
                    loops += 1;
                }
            }
            print!("\r{}/{} found:{}", iter, total, loops);
            std::io::stdout().flush().unwrap();
        }
    }
    println!("posible to loop in {} ways ", loops);
    Ok(())
}
