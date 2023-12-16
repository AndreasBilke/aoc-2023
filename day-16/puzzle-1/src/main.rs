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

    let mut r = Room::from(&lines);
    r.simulate_beam((-1, 0));

    let spots = r.num_energized_spots();
    println!("Number of spots: {spots}");
}

struct Room {
    seen_beams: HashSet<Beam>,
    mirrors: HashMap<(i32, i32), Mirror>,
    max_x: i32,
    max_y: i32
}

impl Room {
    fn from(lines: &Vec<&str>) -> Self {
        let seen_beams: HashSet<Beam> = HashSet::new();
        let mut mirrors: HashMap<(i32, i32), Mirror> = HashMap::new();

        for (row, line) in lines.iter().enumerate() {
            for (column, item) in line.chars().enumerate() {
                if let Ok(mirror) = Mirror::try_from(item) {
                    mirrors.insert((column as i32, row as i32), mirror);
                }
            }
        }

        let max_x = mirrors.iter().map(|item| item.0.0 ).max().unwrap();
        let max_y = mirrors.iter().map(|item| item.0.1 ).max().unwrap();

        Room { seen_beams, mirrors, max_x, max_y }
    }

    fn simulate_beam(&mut self, start: (i32, i32)) {
        let initial_beam = Beam { position: start, direction: Direction::ToRight };
        let mut beams = vec![initial_beam];

        loop {
            let next_beams= self.simulate_beams(&beams);
            if next_beams.len() == 0 {
                break;
            }
            beams = next_beams;
        }
    }

    fn simulate_beams(&mut self, beams: &Vec<Beam>) -> Vec<Beam> {
        let mut next_beams: Vec<Beam> = Vec::new();
        for beam in beams {
            let next_pos = &beam.next_position();
            if let Some(hit_mirror) = self.mirrors.get(next_pos) {
                // there is a mirror at the next position, let them reflect
                let reflected_beams = hit_mirror.reflect(beam, next_pos.clone());
                next_beams.extend(reflected_beams);
            } else {
                // there is no mirror, just move the beam forward
                next_beams.push(Beam { position: next_pos.clone(), direction: beam.direction.clone() });
            }
        }

        let mut filtered_beams: Vec<Beam> = Vec::new();
        for beam in next_beams {
            if !self.seen_beams.contains(&beam) && beam.position.0 >= 0 && beam.position.0 <= self.max_x && beam.position.1 >= 0 && beam.position.1 <= self.max_y {
                self.seen_beams.insert(beam.clone());
                filtered_beams.push(beam);
            }
        }

        filtered_beams
    }

    fn num_energized_spots(&self) -> u32 {
        let mut spots: HashSet<(i32, i32)> = HashSet::new();
        for beam in self.seen_beams.iter() {
            spots.insert(beam.position.clone());
        }

        spots.len() as u32
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Beam {
    position: (i32, i32),
    direction: Direction
}

impl Beam {
    fn next_position(&self) -> (i32, i32) {
        match self.direction {
            Direction::ToRight => (self.position.0 + 1, self.position.1),
            Direction::ToLeft => (self.position.0 - 1, self.position.1),
            Direction::ToTop => (self.position.0, self.position.1 - 1),
            Direction::ToBottom => (self.position.0, self.position.1 + 1)
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    ToTop,
    ToBottom,
    ToRight,
    ToLeft
}

#[derive(Debug)]
enum Mirror {
    Vertical,
    Horizontal,
    BottomToTop,
    TopToBottom
}

impl Mirror {
    fn reflect(&self, beam: &Beam, next_pos: (i32, i32)) -> Vec<Beam> {
        let mut new_beams: Vec<Beam> = Vec::new();

        match (beam.direction, self) {
            (Direction::ToRight | Direction::ToLeft, Mirror::Horizontal) => {
                new_beams.push(Beam { direction: beam.direction.clone(), position: next_pos })
            },
            (Direction::ToTop | Direction::ToBottom, Mirror::Vertical) => {
                new_beams.push(Beam { direction: beam.direction.clone(), position: next_pos })
            },
            (Direction::ToRight | Direction::ToLeft, Mirror::Vertical) => {
                new_beams.push(Beam { position: next_pos.clone(), direction: Direction::ToTop });
                new_beams.push(Beam { position: next_pos.clone(), direction: Direction::ToBottom });
            },
            (Direction::ToTop | Direction::ToBottom, Mirror::Horizontal) => {
                new_beams.push(Beam { position: next_pos.clone(), direction: Direction::ToRight });
                new_beams.push(Beam { position: next_pos.clone(), direction: Direction::ToLeft });
            },
            (Direction::ToRight, Mirror::BottomToTop) => {
                new_beams.push(Beam { position: next_pos, direction: Direction::ToTop});
            },
            (Direction::ToLeft, Mirror::BottomToTop) => {
                new_beams.push(Beam { position: next_pos, direction: Direction::ToBottom});
            },
            (Direction::ToTop, Mirror::BottomToTop) => {
                new_beams.push(Beam { position: next_pos, direction: Direction::ToRight});
            },
            (Direction::ToBottom, Mirror::BottomToTop) => {
                new_beams.push(Beam { position: next_pos, direction: Direction::ToLeft});
            },
            (Direction::ToRight, Mirror::TopToBottom) => {
                new_beams.push(Beam { position: next_pos, direction: Direction::ToBottom});
            },
            (Direction::ToLeft, Mirror::TopToBottom) => {
                new_beams.push(Beam { position: next_pos, direction: Direction::ToTop});
            },
            (Direction::ToTop, Mirror::TopToBottom) => {
                new_beams.push(Beam { position: next_pos, direction: Direction::ToLeft});
            },
            (Direction::ToBottom, Mirror::TopToBottom) => {
                new_beams.push(Beam { position: next_pos, direction: Direction::ToRight});
            }
        }

        new_beams
    }
}

impl TryFrom<char> for Mirror {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            '/' => Ok(Self::BottomToTop),
            '\\' => Ok(Self::TopToBottom),
            _ => Err("Unknown value")
        }
    }
}