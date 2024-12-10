use std::{collections::HashMap, fs::read_to_string, cmp};
use itertools::Itertools;


#[derive(Debug)]
struct Frequencies {
    locations: HashMap<char, Vec<(i32, i32)>>,
    antinodes: HashMap<char, Vec<(i32, i32)>>,
    max_x: i32,
    max_y: i32,
}

impl Frequencies {
    fn get_locations(&self, frequency: char) -> Option<&Vec<(i32, i32)>> {
        self.locations.get(&frequency)
    }

    fn add_antenna(&mut self, frequency: char, (x, y): (i32, i32)) {
        self.locations
            .entry(frequency)
            .or_insert(vec![])
            .push((x, y));
        self.max_x = cmp::max(x, self.max_x);
        self.max_y = cmp::max(y, self.max_y);
        self.update_antinodes(frequency);
    }

    fn within_bounds(&self, (x, y): (i32, i32)) -> bool {
        x <= self.max_x && y <= self.max_y && x >= 0 && y >= 0
    }

    fn get_antinodes(&self, (x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> Vec<(i32, i32)> {
        // Let p1=(x1, y1) and p2=(x2, y2) be two points in 2d space
        // The antinodes a1=(ax1, ay1), a2=(ax2, ay2) are the two points such that:
        //     - d(p1, a1) == 2 * d(p2, a1)
        //     - d(p2, a2) == 2 * d(p1, a2)
        //     - a1 and a2 do not lie on the line segment between p1 and p2
        let dy = y2 - y1;
        let dx = x2 - x1;
        let a1 = (x2 + dx, y2 + dy);
        let a2 = (x1 - dx, y1 - dy);

        vec![a1, a2]

    }

    fn get_resonant_antinodes(&self, (x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> Vec<(i32, i32)> {
        // Let p1=(x1, y1) and p2=(x2, y2) be two points in 2d space
        // The antinodes a1=(ax1, ay1), a2=(ax2, ay2) are the two points such that:
        //     - d(p1, a1) == n * d(p2, a1) (n>2)
        //     - d(p2, a2) == n * d(p1, a2) (n>2)
        //     - a1 and a2 do not lie on the line segment between p1 and p2
        let dy = y2 - y1;
        let dx = x2 - x1;
        let mut antinodes = vec![];
        let mut x = *x2;
        let mut y = *y2;
        while self.within_bounds((x, y)) {
            antinodes.push((x, y));
            x += dx;
            y += dy;
        }
        let mut x = *x1;
        let mut y = *y1;
        while self.within_bounds((x, y)) {
            antinodes.push((x, y));
            x -= dx;
            y -= dy;
        }
        antinodes
    }

    fn update_antinodes(&mut self, frequency: char) {
        let pairs: Vec<_> = self.get_locations(frequency).unwrap().iter().combinations(2).collect();
        /*
        let mut new_antinodes: Vec<_> = pairs.iter().flat_map(|pair| {
            let antinodes = self.get_antinodes(pair[0], pair[1]);
            antinodes.into_iter().filter(|an| self.within_bounds(*an))
        }).collect();
        */
        let mut new_antinodes: Vec<_> = pairs.iter().flat_map(|pair| {
            let antinodes = self.get_resonant_antinodes(pair[0], pair[1]);
            antinodes.into_iter().filter(|an| self.within_bounds(*an))
        }).collect();
        let this_antinode_list = self.antinodes.entry(frequency).or_insert(vec![]);
        new_antinodes.retain(|an| !this_antinode_list.contains(an));
        this_antinode_list.extend(new_antinodes);
    }

    fn count_antinodes(&self, frequency: char) -> usize{
        self.antinodes.get(&frequency).unwrap_or(&vec![]).len()
    }

    fn count_all_unique_antinodes(&self) -> usize {
        let mut antinode_positions:Vec<&(i32, i32)> = vec![];
        for positions in self.antinodes.values() {
            for position in positions {
                if !antinode_positions.contains(&position) {
                    antinode_positions.push(position);
                }
            }
        }
        antinode_positions.len()
    }
}

/// Generalized function to get a Vec of strings in the file. 
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    // Read each line from input.txt
    let lines = read_lines("input.txt");
    let mut frequencies = Frequencies {locations: HashMap::new(), antinodes: HashMap::new(), max_x: lines[0].len() as i32 -1, max_y: lines.len() as i32 -1 };
    for (y, row) in lines.iter().enumerate() {
        for (x, item) in row.chars().enumerate() {
            if item != '.' {
                frequencies.add_antenna(item, (x as i32, y as i32));
            }
        }
    }
    println!("{frequencies:?}");

    let unique_count = frequencies.count_all_unique_antinodes();
    println!("{unique_count:?}");
}
