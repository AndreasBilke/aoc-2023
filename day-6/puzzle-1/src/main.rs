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

    let times: Vec<u32> = lines.get(0).unwrap().split(": ")
        .nth(1).unwrap().trim()
        .split_whitespace()
        .map(|e| e.parse::<u32>().unwrap()).collect();
    let distances: Vec<u32> = lines.get(1).unwrap().split(": ")
        .nth(1).unwrap().trim()
        .split_whitespace()
        .map(|e| e.parse::<u32>().unwrap()).collect();

    let mut races: Vec<Race> = Vec::new();
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];

        let race = Race { available_time: time, winning_distance: distance };
        races.push(race);
    }

    let result: u32 = races.iter().map(|race| race.winning_possibilities()).product();

    println!("Game result: {result}");
}

struct Race {
    available_time: u32,
    winning_distance: u32
}

impl Race {
    fn winning_possibilities(&self) -> u32 {
        let range = self.winning_range();

        range.1 - range.0 + 1
    }

    fn winning_range(&self) -> (u32, u32) {
        // we need to solve 0 = p^2 - pT + D
        // where p is the button push duration, T is the available time for the race and D is the distance to beat

        let t = self.available_time as f64;
        let d = self.winning_distance as f64;

        let mut p1 = (t - (t*t - 4f64 * d).sqrt()) / 2f64;
        let mut p2 = (t + (t*t - 4f64 * d).sqrt()) / 2f64;

        if p1 == p1.ceil() {
            p1 = p1 + 1f64;
        } else {
            p1 = p1.ceil();
        }

        if p2 == p2.floor() {
            p2 = p2 - 1f64;
        } else {
            p2 = p2.floor();
        }

        (p1 as u32, p2 as u32)
    }
}
