use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();

    let spring_rows: Vec<SpringRow> = lines.iter().map(|line|
        SpringRow::from(line)
    ).collect();

    let sum_combinations: u32 = spring_rows.iter().map(|spring_row| {
        spring_row.count_combinations()
    }).sum();

    println!("Total number of combinations {sum_combinations}");
}

struct SpringRow {
    original_line: String,
    corrupt_spring_pattern: Vec<u32>
}

impl SpringRow {
    fn from(line: &str) -> Self {
        let mut s = line.split(" ");

        let original_line = s.nth(0).unwrap().to_string();
        let corrupt_spring_pattern = s.nth(0).unwrap().split(",").map(|item|
            item.parse::<u32>().unwrap()
        ).collect();

        SpringRow {original_line, corrupt_spring_pattern }
    }

    fn count_combinations(&self) -> u32 {
        let starting_pattern = self.original_line.clone();

        let mut combinations = 0;
        self.explore_combinations(starting_pattern, &mut combinations);

        combinations
    }

    fn explore_combinations(&self, current: String, combinations: &mut u32) {
        // if there are no place holders me might find a valid combination
        if !current.contains("?") {
            if self.is_valid(&current) {
                *combinations = *combinations + 1u32;
            }

            return;
        }

        // if !self.can_be_valid_pattern(&current) {
        //     return;
        // }

        let new_corrupt_line = current.replacen("?", "#", 1);
        self.explore_combinations(new_corrupt_line, combinations);

        let new_working_line = current.replacen("?", ".", 1);
        self.explore_combinations(new_working_line, combinations);
    }

    fn is_valid(&self, current: &String) -> bool {
        // idea as in can_be_valid_pattern but much stronger conditions

        let current_pattern = Self::extract_pattern(&current);
        if current_pattern.len() != self.corrupt_spring_pattern.len() {
            return false;
        }

        for i in 0 .. self.corrupt_spring_pattern.len() {
            if current_pattern[i] != self.corrupt_spring_pattern[i] {
                return false;
            }
        }

        true
    }

    fn extract_pattern(current: &String) -> Vec<u32> {
        let pattern: Vec<u32> = current
            .replace("?", " ")
            .replace(".", " ")
            .split_whitespace()
            .map(|item| {
                item.len() as u32
            })
            .collect();

        pattern
    }

    fn can_be_valid_pattern(&self, current: &String) -> bool {
        // idea: replace all ? with ., split by . and count sub array length
        // compare corrupt_spring_pattern (starting with the first element) with the current pattern
        // if current pattern has MORE consecutive corrupted springs than needed, current cannot in any case be a
        // valid combination

        let current_pattern = Self::extract_pattern(current);

        // by the previous tactic we get more/equal consecutive pieces compared to the search pattern
        // if our current_pattern has LESS pieces, we cannot (ever) find a valid pattern here

        if current_pattern.len() < self.corrupt_spring_pattern.len() {
            return false;
        }

        for i in 0 .. self.corrupt_spring_pattern.len() {
            if current_pattern[i] > self.corrupt_spring_pattern[i] {
                return false;
            }
        }

        true
    }
}