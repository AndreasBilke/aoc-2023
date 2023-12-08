use std::collections::HashMap;
use std::env;
use std::fs;
use num::integer::lcm;
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
    let start_nodes: Vec<String> = map.paths.keys().filter(|key|
        key.ends_with("A")
    ).map(|node|
        String::from(node)
    ).collect();

    println!("Number of turns {}", map.find_solution(&start_nodes, &instructions));
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
        let path_re = Regex::new(r"([A-Z1-9]+) = \(([A-Z1-9]+), ([A-Z1-9]+)\)").unwrap();
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

    fn find_solution(&self, start_nodes: &Vec<String>, path: &Vec<Direction>) -> u64 {
        // assumption: Beginning from every node, there is a loop in the path
        // find the loop length for every start node and calculate kgV for all numbers
        // this might be the solution

        let mut loop_lengths: Vec<u64> = Vec::new();
        for start_node in start_nodes {
            let loop_length = self.find_loop_length(start_node, path);
            loop_lengths.push(loop_length);
        }

        let mut c_lcm = lcm(path.len() as u64,loop_lengths[0]);
        for i in 1 .. loop_lengths.len() {
            c_lcm = lcm(c_lcm, loop_lengths[i]);
        }

        c_lcm
    }

    fn find_loop_length(&self, start_node: &String, path: &Vec<Direction>) -> u64 {
        let mut expanded_path: Vec<&String> = Vec::new();
        let mut current_node = start_node;

        loop {
            for direction in path {
                let node_directions = self.paths.get(current_node).unwrap();
                let next_node = match direction {
                    Direction::Left => &node_directions.0,
                    Direction::Right => &node_directions.1
                };
                match expanded_path.iter().position(|n| *n == next_node) {
                    Some(i) => {
                        // next_node is already in list
                        return (expanded_path.len() - i) as u64;
                    },
                    None => {
                        expanded_path.push(next_node);
                    }
                }

                current_node = next_node;
            }
        }
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