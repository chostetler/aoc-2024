use std::fs::read_to_string;
use regex::Regex;

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
    // Totals for parts 1 and 2
    let mut total1: i32 = 0;
    let mut total2: i32 = 0;
    // Part 2 has a sense of "doing" - whether the calculations should be done
    let mut doing = true;
    for line in lines {
        // Regex finds all 'mul(x,y)' operations
        let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
        let muls = mul_re.find_iter(&line).map(|s| s.as_str());
        for mul in muls {
            let op_re = Regex::new(r"\d{1,3}").unwrap();
            let operands: Vec<i32> = op_re.find_iter(mul).map(|n| n.as_str().parse::<i32>().unwrap()).collect();
            total1 += operands[0] * operands[1];
        }

        let combined_re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
        let all = combined_re.find_iter(&line).map(|s| s.as_str());
        for operation in all {
            // mul(x,y)
            if operation.chars().nth(0).unwrap() == 'm' && doing {
                let op_re = Regex::new(r"\d{1,3}").unwrap();
                let operands: Vec<i32> = op_re.find_iter(operation).map(|n| n.as_str().parse::<i32>().unwrap()).collect();
                total2 += operands[0] * operands[1];

            // do()
            } else if operation.chars().nth(2).unwrap() == '(' {
                doing = true;
            // don't()
            } else {
                doing = false;
            }
        }
    }
    println!("{total1}");
    println!("{total2}");

}
