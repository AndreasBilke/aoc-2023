use std::collections::{HashMap, HashSet};
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

    let m = Map::from(&lines);
    let steps = 64;
    println!("Number of distinct fields after {steps} steps: {}", m.inspect(steps));
}

type Node = (i64, i64);

struct Map {
    nodes: HashMap<Node, Vec<Node>>,
    start: Node
}

impl Map {
    fn from(lines: &Vec<&str>) -> Self {
        let mut nodes: HashMap<Node, Vec<Node>> = HashMap::new();
        let mut start: Node = (0, 0); // initial default value

        let mut empty_fields: HashSet<Node> = HashSet::new();

        for (row, line) in lines.iter().enumerate() {
            for (column, item) in line.chars().enumerate() {
                let pos = (column as i64, row as i64);
                match item {
                    'S' => {
                        start = pos.clone();
                        empty_fields.insert(pos);
                    },
                    '.' => {
                        empty_fields.insert(pos);
                    },
                    _ => { } // noop
                }
            }
        }

        empty_fields.iter().for_each(|item_type| {
            let possible_neighbours = vec![
                (item_type.0 + 1, item_type.1),
                (item_type.0 - 1, item_type.1),
                (item_type.0, item_type.1 + 1),
                (item_type.0, item_type.1 - 1),
            ];
            let mut neighbours: Vec<Node> = Vec::new();

            for neighbour in possible_neighbours.iter() {
                if empty_fields.contains(neighbour) {
                    neighbours.push(neighbour.clone());
                }
            }

            nodes.insert(item_type.clone(), neighbours);
        });

        Map { nodes, start }
    }

    fn inspect(&self, max_steps: u64) -> u64 {
        let mut current_nodes: HashSet<&Node> = HashSet::new();
        current_nodes.insert(&self.start);

        let mut round = 0;
        loop {
            if round == max_steps {
                break;
            }

            let mut next_nodes: HashSet<&Node> = HashSet::new();
            current_nodes.iter().for_each(|node| {
                let neighbours = self.nodes.get(node).unwrap();
                for n in neighbours {
                    next_nodes.insert(n);
                }
            });
            current_nodes = next_nodes;

            round = round + 1;
        }

        current_nodes.len() as u64
    }
}
