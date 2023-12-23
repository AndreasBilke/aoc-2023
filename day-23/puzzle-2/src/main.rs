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

    let hf = HikingFun::from(&lines);
    let path = hf.longest_path();

    println!("Longest path {}", path);
}

type Node = (i64, i64);

struct HikingFun {
    nodes_to_edges: HashMap<Node, Vec<Node>>,
    start_node: Node,
    end_node: Node
}

impl HikingFun {
    fn from(lines: &Vec<&str>) -> Self {
        let mut nodes_to_edges: HashMap<Node, Vec<Node>> = HashMap::new();

        // first add nodes. Edges comes later since we don't know the boundaries
        // of our graph for neighbour computation
        for (row, line) in lines.iter().enumerate() {
            for (column, _) in line.chars().enumerate() {
                nodes_to_edges.insert((column as i64, row as i64), vec![]);
            }
        }
        let max_x = nodes_to_edges.iter().map(|n| n.0.0).max().unwrap();
        let max_y = nodes_to_edges.iter().map(|n| n.0.1).max().unwrap();

        let mut all_nodes: HashMap<Node, char> = HashMap::new();
        for (row, line) in lines.iter().enumerate() {
            for (column, c) in line.chars().enumerate() {
                if c != '#' {
                    all_nodes.insert((column as i64, row as i64), c);
                }
            }
        }

        let start_node = all_nodes.iter()
            .find(|n| n.0.1 == 0) // row == 0
            .unwrap().0.clone();
        let end_node = all_nodes.iter()
            .find(|n| n.0.1 == max_x) // row == last one
            .unwrap().0.clone();

        // now search for neighbours
        for (coordinate, _) in all_nodes.iter() {
            let possible_neighbours = vec![
                (coordinate.0 - 1, coordinate.1),
                (coordinate.0 + 1, coordinate.1),
                (coordinate.0, coordinate.1 - 1),
                (coordinate.0, coordinate.1 + 1),
            ];

            let node_neighbours: &mut Vec<Node> = nodes_to_edges.get_mut(coordinate).unwrap();
            for neighbour in possible_neighbours {
                if neighbour.0 >= 0 && neighbour.0 <= max_x && neighbour.1 >= 0 && neighbour.1 <= max_y {
                    if !all_nodes.contains_key(&neighbour) {
                        continue;
                    }
                    node_neighbours.push(neighbour);
                }
            }
        }

        HikingFun { nodes_to_edges, start_node, end_node }
    }

    fn longest_path(&self) -> i64 {
        let mut stack: Vec<(&Node, HashSet<&Node>)> = Vec::new(); // nodes with the previous current path
        let mut end_distances: Vec<i64> = Vec::new(); // save distances to self.end_node

        // doing a DFS
        let mut start_path: HashSet<&Node> = HashSet::new();
        start_path.insert(&self.start_node);
        stack.push((&self.start_node, start_path.clone()));
        loop {
            if stack.len() == 0 {
                break;
            }

            let (current_node, current_path) = stack.pop().unwrap();
            if current_node == &self.end_node {
                end_distances.push(current_path.len() as i64);
            }

            let neighbours = self.nodes_to_edges.get(&current_node).unwrap();
            for neighbour in neighbours {
                if current_path.contains(neighbour) {
                    continue; // never visit a node twice
                }

                let mut new_path = current_path.clone();
                new_path.insert(neighbour);
                stack.push((&neighbour, new_path));
            }
        }

        end_distances.iter().max().unwrap().clone() - 1
    }
}