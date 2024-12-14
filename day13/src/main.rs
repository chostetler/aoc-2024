use std::fs::read_to_string;
use regex::Regex;

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    prize_x: i64,
    prize_y: i64
}

impl Machine {
    fn new() -> Self {
        Self {ax: 0, ay: 0, bx: 0, by: 0, prize_x: 0, prize_y: 0}
    }

    fn system_determinant(&self) -> i64 {
        (self.ax * self.by) - (self.bx * self.ay)
    }

    // Returns (a, b), the respective number of needed button presses
    fn get_linear_coefficients(&self) -> Option<(i64, i64)> {
        let determinant = self.system_determinant();
        let a_numerator = self.by * self.prize_x - self.bx * self.prize_y;
        let b_numerator = self.ax * self.prize_y - self.ay * self.prize_x;
        match (a_numerator % determinant, b_numerator % determinant) {
            (0, 0) => Some((a_numerator/determinant, b_numerator/determinant)),
            (_, _) => None
        }
    }

    fn get_cost(&self) -> Option<i64> {
        match self.get_linear_coefficients() {
            None => None,
            Some((a, b)) => Some(3*a + b)
        }
    }
}

enum MachinePart {
    PartA,
    PartB,
    PartPrize
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
    let mut machines: Vec<Machine> = vec![];
    let mut current_machine = Machine::new(); 
    let mut current_machine_part = MachinePart::PartA;
    let int_re = Regex::new(r"\d+").unwrap();

    for line in lines {
        let integers: Vec<i64> = int_re.find_iter(&line).map(|m| m.as_str().parse().unwrap()).collect();
        if integers.len() < 2 {
            continue;
        }
        match current_machine_part {
            MachinePart::PartA => {
                current_machine.ax = integers[0];
                current_machine.ay = integers[1];
                current_machine_part = MachinePart::PartB;
            },
            MachinePart::PartB => {
                current_machine.bx = integers[0];
                current_machine.by = integers[1];
                current_machine_part = MachinePart::PartPrize;
            },
            MachinePart::PartPrize => {
                current_machine.prize_x = integers[0];
                current_machine.prize_y = integers[1];
                machines.push(current_machine);
                current_machine = Machine::new();
                current_machine_part = MachinePart::PartA;
            },

        }
    }

    let mut part1total = 0;
    for machine in machines.iter() {
        part1total += machine.get_cost().unwrap_or(0);
    }
    println!("{part1total}");

    let mut part2total = 0;
    // For part 2, just increment the positions of the things
    for mut machine in machines.into_iter() {
        machine.prize_x += 10000000000000;
        machine.prize_y += 10000000000000;
        part2total += machine.get_cost().unwrap_or(0);
    }

    println!("{part2total}");
}
