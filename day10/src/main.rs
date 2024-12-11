use std::{collections::HashSet, collections::HashMap, fs::read_to_string};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Map {
    topo: HashMap<Position, u32>,
}

impl Map {
    fn new(lines: Vec<String>) -> Self {
        let mut map = HashMap::new();
        for (row, line) in lines.into_iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                map.insert(Position { x: col as i32, y: row as i32 }, c.to_digit(10).unwrap());
            }
        }
        Self { topo: map }
    }
}

fn neighbor_positions(map: &Map, position: &Position) -> HashSet<Position> {
    let north = Position {x: position.x, y: position.y-1};
    let east = Position {x: position.x+1, y: position.y};
    let south = Position {x: position.x, y: position.y+1};
    let west = Position {x: position.x-1, y: position.y};
    HashSet::from_iter(vec![north, east, south, west].into_iter().filter(|p| map.topo.contains_key(p)))
}

fn peaks_reachable(map: &Map, position: &Position) -> HashSet<Position>{
   if *map.topo.get(position).unwrap()==9 {
        return HashSet::from([*position]);
   }
   let mut peaks = HashSet::new(); 
   let good_neighbors = neighbor_positions(map, position).into_iter().filter(|np| *map.topo.get(np).unwrap() == map.topo.get(position).unwrap()+1u32);
   // println!("Good neighbors of {position:?}: {good_neighbors:?}");
   peaks.extend(good_neighbors.flat_map(|np| peaks_reachable(map, &np)));
   // println!("Peaks: {peaks:?}");
   peaks
}

fn rating_from_position(map: &Map, position: &Position) -> usize {
    if *map.topo.get(position).unwrap() == 9 {
        return 1;
    }
    let mut rating = 0;
    let good_neighbors = neighbor_positions(map, position).into_iter().filter(|np| *map.topo.get(np).unwrap() == map.topo.get(position).unwrap()+1u32);
    for neighbor in good_neighbors {
        rating += rating_from_position(map, &neighbor);
    }
    rating
}

fn trailhead_score(map: &Map, trailhead: &Position) -> usize {
    // println!("Trailhead score for {trailhead:?}");
    peaks_reachable(map, trailhead).len()
}

fn trailhead_rating(map: &Map, trailhead: &Position) -> usize {
    rating_from_position(map, trailhead)
}

fn total_trail_score(map: &Map) -> usize {
    let mut total = 0;
    for (trailhead, _) in map.topo.iter().filter(|(_, &v)| v==0) {
        total += trailhead_score(map, trailhead);
    }
    total
}

fn total_trail_ratings(map: &Map) -> usize {
    let mut total = 0;
    for (trailhead, _) in map.topo.iter().filter(|(_, &v)| v==0) {
        total += trailhead_rating(map, trailhead);
    }
    total
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    // Read each line from input.txt
    let lines = read_lines("input.txt");
    let map = Map::new(lines);
    let part1 = total_trail_score(&map);
    println!("{part1}");
    let part2 = total_trail_ratings(&map);
    println!("{part2}");
}
