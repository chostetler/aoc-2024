use std::{collections::HashMap, fs::read_to_string, };

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn get_line_numbers(line: &str) -> Vec<Option<u32>> {
    line.splitn(2, "   ")
        .map(|x| x.parse::<u32>().ok())
        .collect()
}

fn main() {
    let lines = read_lines("input.txt");
    let mut list_a:Vec<u32> = vec![];
    let mut list_b:Vec<u32> = vec![];

    for line in &lines {
        let numbers = get_line_numbers(line);
        list_a.push(numbers[0].unwrap());
        list_b.push(numbers[1].unwrap());
    }

    // Part 1: Calculate the differences for each list
    list_a.sort();
    list_b.sort();

    let differences:Vec<u32> = list_a.iter()
        .zip(list_b.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .collect();
    let total: u32 = differences.iter().sum();
    println!("total sum {}", total);

    // Part 2: Calculating similarity scores
    let mut occurrences: HashMap<u32, u32> = HashMap::new();

    for &number in &list_b {
        occurrences.entry(number).and_modify(|x| {*x+=1}).or_insert(1);
    }
    // println!("{:?}", occurrences);

    let mut total: u32 = 0;
    for &number in &list_a {
        total += number * occurrences.get(&number).copied().unwrap_or(0);
    }
    println!("total sum {}", total);

}
