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

    let games: Vec<Game> = lines.iter().map(|&line|
        Game::from(line)
    ).collect();

    let game_sum: u32 = games.iter().map(|game|
        if game.is_valid() {
            game.id
        } else {
            0
        }
    ).sum();

    println!("Sum of valid games is {}", game_sum);
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>
}

impl Game {
    fn from(game_line: &str) -> Self {
        // example line: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

        let line_components: Vec<&str> = game_line.split(": ").collect();
        if line_components.len() != 2 {
            panic!("Game line needs : as a split char");
        }
        let game_id = line_components.get(0).unwrap()
            .split(" ").collect::<Vec<&str>>().get(1).unwrap().parse::<u32>()
            .expect("Game id seems not a number");

        let pull_lines: Vec<&str> = line_components.get(1).unwrap()
            .split("; ").collect::<Vec<&str>>();

        let pulls: Vec<Pull> = pull_lines.iter().map(|&line|
            Pull::from(line)
        ).collect();

        Game { id: game_id, pulls }
    }

    fn is_valid(&self) -> bool {
        for pull in self.pulls.iter() {
            if !pull.is_valid() {
                return false
            }
        }

        true
    }
}

#[derive(Debug)]
struct Pull {
    num_red: u32,
    num_green: u32,
    num_blue: u32
}

impl Pull {
    fn from(game_data: &str) -> Self {
        let cubes: Vec<&str> = game_data.split(", ").collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cube in cubes {
            let cube_pair: Vec<&str> = cube.split(" ").collect();
            if cube_pair.len() != 2 {
                panic!("Cube pair does not have two components");
            }
            let cube_count = cube_pair.get(0).unwrap()
                .parse::<u32>().expect("First cube component should be a number");

            match cube_pair.get(1).unwrap() {
                &"red" => red = cube_count,
                &"green" => green = cube_count,
                &"blue" => blue = cube_count,
                _ => panic!("Unexpected colour")
            }
        }


        Pull { num_red: red, num_green: green, num_blue: blue }
    }

    fn is_valid(&self) -> bool {
        self.num_red <= 12 && self.num_green <= 13 && self.num_blue <= 14
    }
}
