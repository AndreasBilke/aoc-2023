use std::env;
use std::fs;

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

    let plan = Plan::from(lines);
    let part_numbers = plan.find_part_numbers();
    let sum: u32 = part_numbers.iter().sum();

    println!("Sum of parts is {}", sum);
}

#[derive(Debug)]
struct Plan {
    symbols: Vec<(u32, u32)>, // coordinates of a symbol
    numbers: Vec<Number> // all numbers in the plan
}

impl Plan {
    fn from(map: Vec<&str>) -> Self {
        let mut symbols: Vec<(u32, u32)> = Vec::new();
        let mut numbers: Vec<Number> = Vec::new();

        for (index, &line) in map.iter().enumerate() {
            Plan::parse_line(index as u32, line, &mut symbols, &mut numbers);
        }

        Plan { symbols, numbers }
    }

    fn parse_line(line_no: u32, content: &str, symbols: &mut Vec<(u32, u32)>, numbers: &mut Vec<Number>) {
        let number_re = Regex::new(r"\d+").unwrap();
        for number_match in number_re.find_iter(content) {
            let start = number_match.start() as u32;
            let end = number_match.end() as u32 - 1;
            let number = number_match.as_str().parse::<u32>().expect("Expected a number");

            numbers.push(
                Number { value: number, line: line_no, position: (start, end) }
            )
        }

        let symbol_re = Regex::new(r"[^\d.]");
        for symbol_match in symbol_re.unwrap().find_iter(content) {
            symbols.push(
                (symbol_match.start() as u32, line_no)
            )
        }
    }

    fn find_part_numbers(&self) -> Vec<u32> {
        let mut part_numbers: Vec<u32> = Vec::new();

        for number in self.numbers.iter() {
            if self.is_part_number(number) {
                part_numbers.push(number.value);
            }
        }

        part_numbers
    }

    fn is_part_number(&self, number: &Number) -> bool {
        let x_start = if number.position.0 > 0 { number.position.0 - 1 } else { 0 };
        let y_start = if number.line > 0 { number.line - 1 } else { 0 };

        for x in x_start ..= (number.position.1 + 1u32) {
            for y in y_start ..= (number.line + 1u32) {
                if self.match_symbol((x, y)) {
                    return true
                }
            }
        }

        false
    }

    fn match_symbol(&self, position: (u32, u32)) -> bool {
        for symbol in self.symbols.iter() {
            if symbol.0 == position.0 && symbol.1 == position.1 {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    line: u32,
    position: (u32, u32)
}