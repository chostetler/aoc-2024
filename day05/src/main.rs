use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct OrderRule {
    page_x: u32,
    page_y: u32
}

#[derive(Debug)]
struct RuleBook {
    rules: Vec<OrderRule>
}

impl RuleBook {
    /// Get a rulebook with only the rules that fully apply to the given update
    fn get_sub_book(&self, update: &Update) -> RuleBook {
        let mut rules = vec![];
        for rule in self.rules.iter() {
            if update.pages.contains(&rule.page_x) && update.pages.contains(&rule.page_y) {
                rules.push(rule.clone());
            }
        }
        RuleBook {rules}
    }
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<u32>
}

impl Update {
    fn complies_with_rule(&self, rule: &OrderRule) -> bool {
        let x_index = self.pages.iter().position(|&p| p==rule.page_x);
        let y_index = self.pages.iter().position(|&p| p==rule.page_y);
        match (x_index, y_index) {
            // Make sure X and Y are both in this update
            (Some(xi), Some(yi)) => xi < yi,
            _ => true
        }
    }

    fn complies_with_rulebook(&self, rulebook: &RuleBook) -> bool{
        rulebook.rules.iter().all(|r| self.complies_with_rule(r))
    }

    fn get_part_1_value(&self, rulebook: &RuleBook) -> u32 {
        if self.complies_with_rulebook(rulebook) {
            return match self.pages.len() {
                0 => 0,
                n => self.pages[n / 2]
            }
        }
        0
    }

    fn get_correct_order(&self, rulebook: &RuleBook) -> Update {
        let mut unsorted_update = self.clone();
        let mut new_pages = vec![];
        while unsorted_update.pages.len() > 0 {
            let sub_book = rulebook.get_sub_book(&unsorted_update);
            // Find a page that isn't to the right of anything
            for &page in unsorted_update.pages.iter() {
                let mut could_be_leftmost = true;
                for &rule in sub_book.rules.iter() {
                    // If a rule appears as a y, it can't be leftmost
                    if rule.page_y == page {
                        could_be_leftmost = false;
                        break;
                    }
                }
                if could_be_leftmost {
                    new_pages.push(page);
                    let index = unsorted_update.pages.iter().position(|x| *x==page).unwrap();
                    unsorted_update.pages.remove(index);
                    break
                }
            }
        }
        Update {pages: new_pages}
    }

    fn get_part_2_value(&self, rulebook: &RuleBook) -> u32 {
        let fixed_update = self.get_correct_order(rulebook);
        return match fixed_update.pages.len() {
            0 => 0,
            n => fixed_update.pages[n / 2]
        }
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
    let rule_regex = Regex::new(r"^(?<x>\d+)\|(?<y>\d+)$").unwrap();
    let update_regex = Regex::new(r"\d+").unwrap();
    let mut rulebook = RuleBook {rules: vec![]};
    let mut updates: Vec<Update> = vec![];
    for row in lines {
        match rule_regex.captures(&row) {
            Some(cap) => rulebook.rules.push(OrderRule {
                                                page_x: cap[1].parse().unwrap(), 
                                                page_y: cap[2].parse().unwrap()}),
            None => updates.push({
                let pages = update_regex.find_iter(&row).map(|s| {
                    s.as_str().parse().unwrap()}).collect();
                Update {pages}
            })

        }
    }

    let mut pt1_total = 0;
    let mut pt2_total = 0;
    for update in updates {
        match update.get_part_1_value(&rulebook){
            0 => pt2_total += update.get_part_2_value(&rulebook),
            n => pt1_total += n
        }
    }
    println!("{pt1_total}");
    println!("{pt2_total}");
}
