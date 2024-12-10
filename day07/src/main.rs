use std::fs::read_to_string;
use regex::Regex;

#[derive(Clone, Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
    running_total: u64,
}

impl Equation {
    fn result_possible(&self) -> bool {
        if self.operands.is_empty() {
            return self.running_total == self.result;
        }
        // If we are greater than our desired result, adding or multiplying
        // will not ever get us to our result.
        if self.running_total > self.result {
            return false;
        }
        self.after_add().result_possible() || self.after_multiply().result_possible() || (self.operands.len() >= 1 && self.after_concat().result_possible())
    }

    fn after_add(&self) -> Equation {
        if self.operands.len() == 0 {
            return self.clone();
        }
        let mut new_equation = self.clone();
        new_equation.running_total = new_equation.running_total.saturating_add(new_equation.operands[0]);
        new_equation.operands.remove(0);
        new_equation
    }

    fn after_multiply(&self) -> Equation {
        if self.operands.len() == 0 {
            return self.clone();
        }
        let mut new_equation = self.clone();
        new_equation.running_total = new_equation.running_total.saturating_mul(new_equation.operands[0]);
        new_equation.operands.remove(0);
        new_equation
    }

    fn after_concat(&self) -> Equation {
        if self.operands.len() == 0 {
            return self.clone();
        }
        let mut new_equation = self.clone();
        let first_number = new_equation.operands.remove(0);
        new_equation.running_total = new_equation.running_total.saturating_mul(10u64.saturating_pow(first_number.ilog10()+1));
        new_equation.running_total = new_equation.running_total.saturating_add(first_number);
        new_equation
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
    let mut total = 0;
    for row in lines {
        // Extract all numbers from each line using regex
        let parts_regex = Regex::new(r"\d+").unwrap();
        // Parse found numbers into a vector of u64
        let equation_numbers: Vec<u64> = parts_regex.find_iter(&row).map(|s| s.as_str().parse().unwrap()).collect();
        // Create an Equation struct with parsed values
        let equation = Equation { result: equation_numbers[0], operands: equation_numbers[2..].to_vec(), running_total: equation_numbers[1]};
        // Check if the equation result is possible and add to total if true
        if equation.result_possible() {
            total += equation.result;
        }
    }
    // Print final sum
    println!("{total}");
}
