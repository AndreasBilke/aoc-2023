mod mirror;

use std::env;

use mirror::map::Map;
use crate::mirror::map::{parse_input, summarize_all};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Not enough command line arguments");
    }
    
    let input = &args[1];

    let maps: Vec<Map> = parse_input(input);
    let sum: u32 = summarize_all(&maps);

    println!("Result {sum}");
}
