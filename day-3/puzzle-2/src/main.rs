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
    let gears_values = plan.find_gear_values();
    let gear_ration_sum: u32 = gears_values.iter().map(|(part1, part2)|
        part1 * part2
    ).sum();

    println!("Gear ratio sum is {}", gear_ration_sum);
}

#[derive(Debug)]
struct Plan {
    symbols: Vec<Symbol>, // all symbols in the plan
    numbers: Vec<Number> // all numbers in the plan
}

impl Plan {
    fn from(map: Vec<&str>) -> Self {
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut numbers: Vec<Number> = Vec::new();

        for (index, &line) in map.iter().enumerate() {
            Plan::parse_line(index as u32, line, &mut symbols, &mut numbers);
        }

        Plan { symbols, numbers }
    }

    fn parse_line(line_no: u32, content: &str, symbols: &mut Vec<Symbol>, numbers: &mut Vec<Number>) {
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
            let symbol = symbol_match.as_str().chars().nth(0).unwrap();
            symbols.push(
                Symbol { value: symbol, position: (symbol_match.start() as u32, line_no) }
            )
        }
    }

    fn find_gear_values(&self) -> Vec<(u32, u32)> {
        let mut gear_values: Vec<(u32, u32)> = Vec::new();

        for symbol in self.symbols.iter() {
            if !symbol.is_gear_value() {
                continue;
            }

            let adjacent_numbers = self.find_adjacent_numbers(symbol);
            if adjacent_numbers.len() != 2 {
                continue;
            }
            let n1 = adjacent_numbers.get(0).unwrap().value;
            let n2 = adjacent_numbers.get(1).unwrap().value;

            gear_values.push((n1, n2));
        }

        gear_values
    }

    fn find_adjacent_numbers(&self, symbol: &Symbol) -> Vec<&Number> {
        let mut adjacent_numbers: Vec<&Number> = Vec::new();

        for number in self.numbers.iter() {
            if number.intersect_symbol(symbol) {
                adjacent_numbers.push(number);
            }
        }

        adjacent_numbers
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    line: u32,
    position: (u32, u32)
}

impl Number {
    fn intersect_symbol(&self, symbol: &Symbol) -> bool {
        // check if y can match
        let min_y = if self.line > 0 { self.line - 1 } else { 0 };
        if !(min_y <= symbol.position.1 && symbol.position.1 <= self.line + 1) {
            return false
        }

        // check if x can match
        let min_x = if self.position.0 > 0 { self.position.0 - 1 } else { 0 };
        if !(min_x <= symbol.position.0 && symbol.position.0 <= self.position.1 + 1) {
            return false
        }

        true
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    position: (u32, u32)
}

impl Symbol {
    fn is_gear_value(&self) -> bool {
        self.value == '*'
    }
}