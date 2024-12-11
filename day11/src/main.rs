use std::collections::HashMap;

#[derive(Debug)]
struct StoneSummary {
    count: HashMap<usize, usize>
}

impl StoneSummary {
    fn stone_count(&self) -> usize {
        self.count.values().sum()
    }

    fn blink(&mut self) {
        let mut new_count: HashMap<usize, usize> = HashMap::new();
        for (stone, quantity) in self.count.iter() {
            let blink_result = blink(*stone);
            for new_stone in blink_result {
                let new_quantity: usize = new_count.entry(new_stone).or_insert(0).clone() + *quantity;
                new_count.insert(new_stone, new_quantity);
            }
        }

        self.count = new_count;
    }
}

fn split_digits(stone: usize) -> Vec<usize> {
    let stone_string = stone.to_string();
    let len = stone_string.chars().count();
    if len % 2 != 0 {
        panic!();
    }
    let first_part: usize = stone_string[..len/2].parse().unwrap();
    let second_part: usize = stone_string[len/2..].parse().unwrap();
    vec![first_part, second_part]
}

fn blink(stone: usize) -> Vec<usize> {
    match stone {
        0 => vec![1],
        n => {
            match n.to_string().chars().count()%2 {
                1 => vec![n*2024],
                0 => split_digits(n),
                _ => panic!()
            }
        }
    }
}

fn main() {
    // Read each line from input.txt
    let stones: Vec<usize> = vec![5910927, 0, 1, 47, 261223, 94788, 545, 7771];
    // let stones: Vec<usize> = vec![125, 17];
    let mut summary: StoneSummary = StoneSummary { count: HashMap::new() };
    for stone in stones {
        summary.count.insert(stone, 1);
    }

    for _ in 0..25 {
        summary.blink();
    }
    let part1 = summary.stone_count();
    println!("{part1}");
    for _ in 0..50 {
        summary.blink();
    }
    let part2 = summary.stone_count();
    println!("{part2}");
}
