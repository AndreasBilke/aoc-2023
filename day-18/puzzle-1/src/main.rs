use std::env;
use std::fs;

use regex;
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
    let actions: Vec<DigAction> = lines.iter().map(|line| DigAction::from(line)).collect();

    let mut hole = Hole::new();
    hole.process(&actions);

    let interior = hole.compute_interior();
    println!("Size of hole: {}", interior);
}

struct Hole {
    points: Vec<(i32, i32)>
}

impl Hole {
    fn new() -> Self {
        let points: Vec<(i32, i32)> = Vec::new();

        Hole { points }
    }

    fn process(&mut self, actions: &Vec<DigAction>) {
        let mut current_hole: (i32, i32) = (0, 0);
        self.points.push(current_hole.clone());

        for action in actions.iter() {
            let next_hole = action.dig(&current_hole);

            self.points.push(next_hole.clone());
            current_hole = next_hole;
        }
    }

    fn compute_interior(&self) -> u32 {
        let mut interior: i32 = 0;
        let mut border_count: i32 = 0;
        // formulae taken from https://www.mathopenref.com/coordpolygonarea.html

        for point_pair in self.points.windows(2) {
            let vec = (point_pair[1].0 - point_pair[0].0, point_pair[1].1 - point_pair[0].1);
            let border_length = (vec.0 + vec.1).abs();
            border_count = border_count + border_length;

            interior = interior +
                point_pair[0].0 * point_pair[1].1 - point_pair[0].1 * point_pair[1].0
        }
        interior = interior + border_count;
        interior = interior / 2;
        interior = interior + 1;

        interior.abs() as u32
    }
}

struct DigAction {
    direction: Direction,
    amount: i32
}

impl DigAction {
    fn from(line: &str) -> Self {
        let r = Regex::new(r"(\w) (\d+) \(#[a-z0-9]+\)").unwrap();
        let m = r.captures(line).unwrap();

        let direction = Direction::from(m.get(1).unwrap().as_str());
        let amount = m.get(2).unwrap().as_str().parse::<i32>().unwrap();

        DigAction { direction, amount }
    }

    fn dig(&self, start: &(i32, i32)) -> (i32, i32) {
        let direction_vector = self.direction.as_vector();
        let new_hole = (
            start.0 + direction_vector.0 * self.amount,
            start.1 + direction_vector.1 * self.amount
        );

        new_hole
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn from(d: &str) -> Self {
        match d {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Unknown direction: {}", d)
        }
    }

    fn as_vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        }
    }
}
