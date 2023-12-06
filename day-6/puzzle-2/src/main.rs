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

    let time: u64 = lines.get(0).unwrap().split(": ")
        .nth(1).unwrap().trim()
        .replace(" ", "")
        .parse::<u64>().unwrap();
    let distance: u64 = lines.get(1).unwrap().split(": ")
        .nth(1).unwrap().trim()
        .replace(" ", "")
        .parse::<u64>().unwrap();

    let race = Race { available_time: time, winning_distance: distance };

    println!("Game result: {}", race.winning_possibilities());
}

struct Race {
    available_time: u64,
    winning_distance: u64
}

impl Race {
    fn winning_possibilities(&self) -> u64 {
        let range = self.winning_range();

        range.1 - range.0 + 1
    }

    fn winning_range(&self) -> (u64, u64) {
        // we need to solve 0 = p^2 - pT + D
        // where p is the button push duration, T is the available time for the race and D is the distance to beat

        let t = self.available_time as f64;
        let d = self.winning_distance as f64;

        let mut x1 = (t - (t*t - 4f64 * d).sqrt()) / 2f64;
        let mut x2 = (t + (t*t - 4f64 * d).sqrt()) / 2f64;

        if x1 == x1.ceil() {
            x1 = x1 + 1f64;
        } else {
            x1 = x1.ceil();
        }

        if x2 == x2.floor() {
            x2 = x2 - 1f64;
        } else {
            x2 = x2.floor();
        }

        (x1 as u64, x2 as u64)
    }
}
