use std::collections::HashMap;
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
    let instructions: Vec<Direction> = lines.get(0).unwrap().chars().map(|direction|
        Direction::from(direction)
    ).collect();

    let map = Map::from(&lines[2..lines.len()]);
    println!("Number of turns {}", map.follow(instructions, "AAA", "ZZZ"));
}

#[derive(Debug)]
struct Map {
    paths: HashMap<String, (String, String)>
}

impl Map {
    fn from(lines: &[&str]) -> Self {
        let mut paths: HashMap<String, (String, String)> = HashMap::new();
        // example line
        // AAA = (BBB, CCC)
        let path_re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
        for line in lines {
            let re_match = path_re.captures(line).unwrap();
            let from = re_match.get(1).unwrap().as_str();
            let to_left = re_match.get(2).unwrap().as_str();
            let to_right = re_match.get(3).unwrap().as_str();

            paths.insert(
                String::from(from),
                (String::from(to_left), String::from(to_right))
            );
        }

        Map { paths }
    }

    fn follow(&self, instructions: Vec<Direction>, start: &str, end: &str) -> u32 {
        let mut counter = 0u32;

        let mut current_pos = start;
        loop {
            for instruction in instructions.iter() {
                current_pos = self.next_path(current_pos, instruction);
            }
            counter = counter + instructions.len() as u32;

            if current_pos == end {
                break;
            }
        }

        counter
    }
    fn next_path(&self, current_pos: &str, direction: &Direction) -> &str {
        let possible_paths = self.paths.get(current_pos).unwrap();

        let next_pos = match direction {
            Direction::Left => &possible_paths.0,
            Direction::Right => &possible_paths.1
        };

        next_pos
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

impl Direction {
    fn from(direction_char: char) -> Self {
        match direction_char {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Unknown direction character {}", direction_char)
        }
    }
}