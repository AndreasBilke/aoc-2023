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

    let elements: Vec<&str> = lines[0].split(",").collect();

    let sum: u128 = elements.iter().map(|element|
        to_hash(element)
    ).sum();

    println!("Sum is {}", sum);
}

fn to_hash(element: &str) -> u128 {
    let mut h = 0u128;

    element.chars().for_each(|c| {
        let ascii_code = c as u128;
        h = h + ascii_code;
        h = h * 17;
        h = h % 256;

    });

    h
}
