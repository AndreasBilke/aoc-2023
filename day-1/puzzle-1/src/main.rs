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

    let numbers= lines.iter().map( |&line|
        extract_number(line)
    ).collect::<Vec<u32>>();
    let result: u32 = numbers.iter().sum();

    println!("Result is {}", result);
}

fn extract_number(line: &str) -> u32 {
    let mut chars: Vec<char> = vec![];

    for c in line.chars() {
        if c.is_numeric() {
            chars.push(c);

            break
        }
    }

    for c in line.chars().rev() {
        if c.is_numeric() {
            chars.push(c);

            break
        }
    }

    match String::from_iter(chars.iter()).parse::<u32>() {
        Ok(r) => r,
        _ => panic!("Could not parse map input")
    }
}
