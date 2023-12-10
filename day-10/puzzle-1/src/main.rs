use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use queues::{IsQueue, Queue, queue};

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
    println!("Longest distance is {}", m.find_loop_length());
}

// assumption: The starting node has only 2 pipes connecting to it

struct Map {
    pos_to_type: HashMap<(i32, i32), Type>,
    start_node: (i32, i32)
}

impl Map {
    fn from(lines: &Vec<&str>) -> Self {
        let mut types: HashMap<(i32, i32), Type> = HashMap::new();

        let mut start_position: Option<(i32, i32)> = None;

        // we found all items with position
        for (row, line) in lines.iter().enumerate() {
            for (column, item) in line.chars().enumerate() {
                if let Some(t) = Type::from(item) {
                    types.insert((column as i32, row as i32), t);

                    if t == Type::Start {
                        start_position = Some((column as i32, row as i32));
                    }
                }
            }
        }

        let Some(start_node) = start_position else { panic!("We didn't found a starting node on the map") };

        Map { pos_to_type: types, start_node }
    }

    fn find_loop_length(&self) -> u32 {
        let mut next_nodes: Queue<(i32, i32)> = queue![self.start_node];
        let mut visited_nodes: HashSet<(i32, i32)> = HashSet::new();
        visited_nodes.insert(self.start_node);

        while next_nodes.size() > 0 {
            let next_node = next_nodes.remove().unwrap();
            visited_nodes.insert(next_node.clone());

            let neighbours = Self::possible_neighbours(next_node, &self.pos_to_type);

            for neighbour in neighbours.iter() {
                if !visited_nodes.contains(neighbour) {
                    let _ = next_nodes.add(neighbour.clone());
                }
            }
        }

        (visited_nodes.len() / 2) as u32
    }

    fn possible_neighbours(n: (i32, i32), all_neighbours: &HashMap<(i32, i32), Type>) -> Vec<(i32, i32)> {
        let mut possible_neighbours: Vec<(i32, i32)> = Vec::new();

        let neighbours: [(i32, i32); 4] = [
            (n.0 - 1, n.1), (n.0 + 1, n.1),
            (n.0, n.1 - 1), (n.0, n.1 + 1)
        ];

        for to_n in neighbours {
            if !all_neighbours.contains_key(&to_n) { // bound check in the map
                continue;
            }

            let f_type = all_neighbours.get(&n).unwrap();
            let t_type = all_neighbours.get(&to_n).unwrap();
            let direction = Direction::from(n, to_n);

            if f_type.compatible(t_type.clone(), direction) {
                possible_neighbours.push(to_n);
            }
        }

        possible_neighbours
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Type {
    Start, // S
    NorthSouth, // |
    EastWest, // -
    NorthEast, // L
    NorthWest, // J
    SouthWest, // 7
    SouthEast // F
}

impl Type {
    fn from(n: char) -> Option<Self> {
        match n {
            'S' => Some(Self::Start),
            '|' => Some(Self::NorthSouth),
            '-' => Some(Self::EastWest),
            'L' => Some(Self::NorthEast),
            'J' => Some(Self::NorthWest),
            '7' => Some(Self::SouthWest),
            'F' => Some(Self::SouthEast),
            _ => None
        }
    }

    fn compatible(&self, other: Type, direction: Direction) -> bool {
        match (self, direction) {
            (Self::NorthSouth, Direction::ToLeft | Direction::ToRight) => false,
            (Self::NorthSouth, Direction::ToTop) => other == Self::NorthSouth || other == Self::SouthEast || other == Self::SouthWest,
            (Self::NorthSouth, Direction::ToBottom) => other == Self::NorthSouth || other == Self::NorthEast || other == Self::NorthWest,

            (Self::EastWest, Direction::ToLeft) => other == Self::EastWest || other == Self::NorthEast || other == Self::SouthEast,
            (Self::EastWest, Direction::ToRight) => other == Self::EastWest || other == Self::NorthWest || other == Self::SouthWest,
            (Self::EastWest, Direction::ToTop | Direction::ToBottom) => false,

            (Self::NorthEast, Direction::ToLeft | Direction::ToBottom) => false,
            (Self::NorthEast, Direction::ToRight) => other == Self::EastWest || other == Self::NorthWest || other == Self::SouthWest,
            (Self::NorthEast, Direction::ToTop) => other ==  Self::NorthSouth || other == Self::SouthWest || other == Self::SouthEast,

            (Self::NorthWest, Direction::ToRight | Direction::ToBottom) => false,
            (Self::NorthWest, Direction::ToLeft) => other == Self::EastWest || other == Self::NorthEast || other == Self::SouthEast,
            (Self::NorthWest, Direction::ToTop) => other == Self::NorthSouth || other == Self::SouthWest || other == Self::SouthEast,

            (Self::SouthWest, Direction::ToRight | Direction::ToTop) => false,
            (Self::SouthWest, Direction::ToLeft) => other == Self::EastWest || other == Self::NorthEast || other == Self::SouthEast,
            (Self::SouthWest, Direction::ToBottom) => other == Self::NorthSouth || other == Self::NorthEast || other == Self::NorthWest,

            (Self::SouthEast, Direction::ToLeft | Direction::ToTop) => false,
            (Self::SouthEast, Direction::ToRight) => other == Self::EastWest || other == Self::NorthWest || other == Self::SouthWest,
            (Self::SouthEast, Direction::ToBottom) => other == Self::NorthSouth || other == Self::NorthEast || other == Self::NorthWest,

            (Self::Start, Direction::ToLeft) => !(other == Self::NorthSouth || other == Self::NorthWest || other == Self::SouthWest),
            (Self::Start, Direction::ToRight) => !(other == Self::NorthSouth || other == Self::NorthEast || other == Self::SouthEast),
            (Self::Start, Direction::ToTop) => !(other == Self::EastWest || other == Self::NorthEast || other == Self::NorthWest),
            (Self::Start, Direction::ToBottom) => !(other == Self::EastWest || other == Self::SouthWest || other == Self::SouthEast)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    ToLeft,
    ToRight,
    ToTop,
    ToBottom
}

impl Direction {
    fn from(from: (i32, i32), to: (i32, i32)) -> Self {
        let diff = (to.0 - from.0, to.1 - from.1);

        match diff {
            (x, y) if x < 0 && y == 0 => Direction::ToLeft,
            (x, y) if x > 0 && y == 0 => Direction::ToRight,
            (x, y) if x == 0 && y < 0 => Direction::ToTop,
            (x, y) if x == 0 && y > 0 => Direction::ToBottom,
            (_, _) => panic!("Cannot decide what to do with from={:?} and to={:?}", from, to)
        }
    }
}
