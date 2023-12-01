use std::collections::HashMap;
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

fn extract_digit_from_start(data: &str, reverse_replace: bool) -> Result<char, &str> {
    let data = String::from(data);

    // first try single digits
    let first_char = data.chars().nth(0).expect("Empty length");
    if first_char.is_numeric() {
        return Ok(first_char);
    }

    // try words
    let replace_map = match reverse_replace {
        false => HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]),
        true => HashMap::from([
            ("eno", '1'),
            ("owt", '2'),
            ("eerht", '3'),
            ("ruof", '4'),
            ("evif", '5'),
            ("xis", '6'),
            ("neves", '7'),
            ("thgie", '8'),
            ("enin", '9'),
        ])
    };

    for (word, digit) in replace_map {
        if data.starts_with(word) {
            return Ok(digit);
        }
    }

    Err("Slice did not start with a extended digit")
}

fn extract_number(line: &str) -> u32 {
    let mut chars: Vec<char> = vec![];

    for start_index in 0..line.len() {
        let sub_string = &line[start_index..];
        match extract_digit_from_start(sub_string, false) {
            Ok(v) => {
                chars.push(v);

                break
            },
            _ => {} // keep going
        }
    }

    let rev_line = line.chars().rev().collect::<String>();
    for start_index in 0..rev_line.len() {
        let sub_string = &rev_line[start_index..];
        match extract_digit_from_start(sub_string, true) {
            Ok(v) => {
                chars.push(v);

                break
            },
            _ => {} // keep going
        }
    }

    match String::from_iter(chars.iter()).parse::<u32>() {
        Ok(r) => r,
        _ => panic!("Could not parse map input")
    }
}
