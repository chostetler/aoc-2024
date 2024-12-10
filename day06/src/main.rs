use std::{collections::HashMap, fs::read_to_string};

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    North, 
    East,
    South, 
    West
}

#[derive(Copy, Clone, PartialEq)]
enum TileType {
    Empty,
    Visited,
    Blocked, 
    Outside
}

#[derive(Clone, PartialEq)]
struct Map {
    tiles: HashMap<(i32, i32), TileType>,
    guard_position: (i32, i32),
    guard_direction: Direction,
    rows: usize,
    cols: usize
}

impl Map {
    fn get_tile(&self, (x, y): (i32, i32)) -> &TileType {
        if x < 0 || y < 0 || x >= (self.rows as i32) || y >= (self.cols as i32) {
            return &TileType::Outside;
        }
        self.tiles.get(&(x, y)).unwrap_or(&TileType::Empty)
    }

    fn set_tile(&mut self, (x, y): (i32, i32), tile_type: TileType) {
        self.tiles.insert((x, y), tile_type);
    }

    fn offset_position((x, y): (i32, i32), direction: Direction) -> (i32, i32){
        match direction {
            Direction::North => (x, y-1),
            Direction::East => (x+1, y),
            Direction::South => (x, y+1),
            Direction::West => (x-1, y)
        }
    }

    fn step_guard(&mut self) {
        let facing_tile = self.get_tile(Map::offset_position(self.guard_position, self.guard_direction));
        match facing_tile {
            TileType::Empty | TileType::Visited | TileType::Outside => {
                self.set_tile(self.guard_position, TileType::Visited);
                self.guard_position = Map::offset_position(self.guard_position, self.guard_direction);
            }
            TileType::Blocked => {
                self.guard_direction = match self.guard_direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North
                }
            }
        }
    }

    fn get_map_with_obstruction(&self, (x, y): (i32, i32)) -> Map{
        let mut new_map = self.clone();
        new_map.set_tile((x, y), TileType::Blocked);
        new_map
    }
}

/// Generalized function to get a Vec of strings in the file. 
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn main() {
    let lines = read_lines("input.txt");
    let mut map = Map { tiles: HashMap::new(), guard_position: (0, 0), guard_direction: Direction::North, rows: lines.len(), cols: lines[0].len() };

    // Part 1
    for (y, row) in lines.iter().enumerate() {
        for (x, tile) in row.chars().enumerate() {
            match tile {
                '^' => map.guard_position = (x as i32, y as i32), 
                '#' => map.set_tile((x as i32, y as i32), TileType::Blocked),
                _ => ()
            }
        }
    }
    let map2 = map.clone();

    while !(map.get_tile(map.guard_position) == &TileType::Outside) {
        map.step_guard();
    }

    let mut visited_count = 0;
    for tile in map.tiles.values() {
        if tile == &TileType::Visited {
            visited_count += 1;
        }
    }
    println!("{visited_count}");

    // Part 2
    let mut looping_spots = 0;
    for y in 0..map2.rows {
        println!("Row {y}");
        for x in 0..map2.cols {
            // Get an obstruction map for each possible position
            if (x as i32, y as i32) == map2.guard_position || map2.get_tile((x as i32, y as i32)) == &TileType::Blocked {
                continue;
            }
            let mut new_map = map2.get_map_with_obstruction((x as i32, y as i32));
            let starting_position = new_map.guard_position;
            new_map.step_guard();
            let mut is_looping = true;
            let mut steps = 1;
            let step_threshold = new_map.rows * new_map.cols * 4;
            while is_looping && (steps < step_threshold){
                new_map.step_guard();
                steps += 1;
                if new_map.get_tile(new_map.guard_position) == &TileType::Outside {
                    is_looping = false;
                    break;
                }
            }
            if is_looping {
                // println!("Found a spot! ({x}, {y})");
                looping_spots += 1;
            }
        }
    }
    println!("{looping_spots}");
}
