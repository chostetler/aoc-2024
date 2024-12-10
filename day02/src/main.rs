use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut rising = false;

        for i in 0..self.levels.len()-1 {
            let difference = self.levels[i+1] - self.levels[i];
            if i==0 {
                rising = difference > 0;
            }
            if difference.abs() == 0 || difference.abs() > 3 {
                return false;
            }
            if (rising && difference < 0) || (!rising && difference > 0) {
                return false;
            }
        }
        true
    }

    fn is_safe_dampened(&self) -> bool {
        for i in 0..self.levels.len() {
            let mut new_rep = self.clone();
            new_rep.levels.remove(i);
            if new_rep.is_safe() {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct ParseReportErr;
impl FromStr for Report {
    type Err = ParseReportErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numlevels:Vec<i32> = s.trim().split(' ').filter_map(|n| n.parse::<i32>().ok()).collect();
        Ok(Report { levels: numlevels })
    }
    

}

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
    let mut reports = Vec::new();
    for line in lines {
        let report = Report::from_str(&line).unwrap();
        if !report.levels.is_empty() {
            reports.push(report);
        }
    }

    // Part 1: iterate through all of the reports
    let mut total1 = 0;
    let mut total2 = 0;
    for report in reports {
        // println!("{report:?}");
        // println!("{}", is_safe(&report));
        if report.is_safe() {
            total1 += 1;
            total2 += 1;
        } else if report.is_safe_dampened() {
            total2 += 1;
        }
    }
    println!("{total1}");
    println!("{total2}");

}
