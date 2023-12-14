mod mirror;

use std::env;
use crate::mirror::mirror::parse_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];

    let mut map = parse_input(input);
    map.tilt();

    let w = map.weight();
    println!("Result {w}");
}
