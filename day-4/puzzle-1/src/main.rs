use std::collections::HashSet;
use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];
    let lines = fs::read_to_string(input)
        .expect("Could not read file");
    let lines: Vec<&str> = lines.trim().split('\n').collect();

    let mut intersection_size: Vec<u32> = Vec::new();
    for line in lines {
        let g = Game::from(line);

        intersection_size.push(g.intersection_size());
    }

    let game_result: u32 = intersection_size.iter().map(|elem|
        return if *elem == 0u32 {
            0
        } else {
            2u32.pow(elem - 1)
        }
    ).sum();

    println!("Game result {}", game_result);
}

#[derive(Debug)]
struct Game {
    winning_numbers: HashSet<u32>,
    drawn_numbers: HashSet<u32>
}

impl Game {
    fn from(line: &str) -> Self {
        let full_line_re = Regex::new(r"Card +\d+: (.*)").unwrap();
        let Some(caps) = full_line_re.captures(line) else { panic!("Unexpected line format --{}--", line) };

        let numbers = caps[1].trim();
        let number_pairs: Vec<&str> = numbers.split(" | ").collect();

        if number_pairs.len() != 2 {
            panic!("Unexpected line format --{}--", line);
        }

        let winning_numbers = Game::extract_numbers(number_pairs.get(0).unwrap());
        let drawn_numbers = Game::extract_numbers(number_pairs.get(1).unwrap());

        Game { winning_numbers, drawn_numbers }
    }

    fn extract_numbers(numbers: &str) -> HashSet<u32> {
        let numbers_re = Regex::new(r"\d+").unwrap();

        let numbers: Vec<u32> = numbers_re.captures_iter(numbers).map(|cap| {
            let number = cap.get(0).unwrap().as_str();

            number.parse::<u32>().unwrap()
        }).collect();

        HashSet::from_iter(numbers.into_iter())
    }

    fn intersection_size(&self) -> u32 {
        let intersection: Vec<&u32> = self.winning_numbers.intersection(&self.drawn_numbers).collect();

        intersection.len() as u32
    }
}