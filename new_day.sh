#!/bin/sh

if [ $# -ne 1 ]; then
    echo "Usage: $0 <day>"
    exit 1
fi

DAY=$1

mkdir "day-$DAY"
cd "day-$DAY"
cargo new "puzzle-1"
cd puzzle-1

cat <<EOF > src/main.rs
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
}
EOF

