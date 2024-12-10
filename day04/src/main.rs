use std::fs::read_to_string;

#[derive(Debug)]
struct Crossword {
    grid: Vec<Vec<char>>
}

impl Crossword {
    fn char_at(&self, row:usize, col:usize) -> Option<char> {
        if row < self.grid.len() && col < self.grid[0].len() {
            return Some(self.grid[row][col]);
        } else {
            return None;
        }
    }

    fn needle_here(&self, (row, col): (usize, usize), (dr, dc): (i32, i32), needle:&str) -> bool{
        let mut r = row;
        let mut c = col;
        for letter in needle.chars() {
            match self.char_at(r, c) {
                None => return false,
                Some(c) if c==letter => (),
                Some(_) => return false
            }
            r = (r as i32 + dr) as usize;
            c = (c as i32 + dc) as usize;
        }
        // If we didn't have any mismatches by here, it must exist
        true

    }

    fn xmas_here(&self, (row, col): (usize, usize)) -> bool {
        if self.char_at(row, col).unwrap() != 'A' {
            return false;
        }
        if col <= 0 || row <= 0 {
            return false;
        }
        self.char_at(row, col);
        let corners = vec![self.char_at(row+1, col+1).unwrap_or('.'),
                           self.char_at(row+1, col-1).unwrap_or('.'),
                           self.char_at(row-1, col-1).unwrap_or('.'),
                           self.char_at(row-1, col+1).unwrap_or('.')];
        if (corners[0] == corners[1]) {
            return (corners[0]=='S' && corners[2]=='M' && corners[3]=='M') ||
                   (corners[0]=='M' && corners[2]=='S' && corners[3]=='S')
        } else if (corners[0] == corners[3]) {
            return (corners[0]=='S' && corners[2]=='M' && corners[1]=='M') ||
                   (corners[0]=='M' && corners[2]=='S' && corners[1]=='S')
        }
        false
    }

    fn count_needles_starting_at(&self, (row, col): (usize, usize), needle:&str) -> usize {
        let mut count = 0;
        let all_directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0),
                                  (1, 1), (1, -1), (-1, 1), (-1, -1)];
        for direction in all_directions {
            if self.needle_here((row, col), direction, needle) {
                count += 1;
            }
        }
        count
    }

    fn count_all_needles(&self, needle:&str) -> usize {
        let mut count = 0;
        for r in 0..self.grid.len() {
            for c in 0..self.grid[0].len() {
                count += self.count_needles_starting_at((r, c), needle);
            }
        }
        count
    }

    fn count_all_xmas(&self) -> usize {
        let mut count = 0;
        for r in 0..self.grid.len() {
            for c in 0..self.grid[0].len() {
                if self.xmas_here((r, c)) {
                    count += 1;
                }
            }
        }
        count
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
    // First, gather together all of the reports
    let lines = read_lines("input.txt");
    let mut grid: Vec<Vec<char>> = vec![];
    for row in lines {
        let rowvec: Vec<char> = row.chars().collect();
        grid.push(rowvec);
    }

    let crossword = Crossword{grid};
    
    let count = crossword.count_all_needles("XMAS");
    println!("{count}");
    let xmas = crossword.count_all_xmas();
    println!("{xmas}");
}
