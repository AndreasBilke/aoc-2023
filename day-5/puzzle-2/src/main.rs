use std::collections::HashMap;
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

    let seeds_line = lines[0];
    let seeds_line_re = Regex::new(r"seeds: (.*)").unwrap();
    let seeds_numbers_line = seeds_line_re.captures(seeds_line).unwrap().get(1).unwrap().as_str();
    let seeds_numbers_as_string: Vec<&str> = seeds_numbers_line.split(" ").collect();

    let mut seed_pairs: Vec<(u128, u128)> = Vec::new();
    for pair in seeds_numbers_as_string.chunks(2) {
        let seed_start: u128 = pair[0].parse().unwrap();
        let seed_range: u128 = pair[1].parse().unwrap();

        seed_pairs.push((seed_start, seed_range));
    }

    let map_lines: Vec<&str> = lines[2..lines.len()].to_vec();
    let almanac = Almanac::from(map_lines);

    let mut min_dest: Option<u128> = None;
    seed_pairs.iter().for_each(|seed| {
        let range_min: u128 = (seed.0 .. seed.0 + seed.1).map(|seed|
            almanac.convert(seed)
        ).min().unwrap();
        if let Some(old_min) = min_dest {
            if old_min > range_min {
                min_dest = Some(range_min);
            }
        } else {
            min_dest = Some(range_min);
        }
    });

    println!("Result is {:?}", min_dest);
}

struct Almanac {
    maps: HashMap<Category, ConversionMap>
}

impl Almanac {
    fn from(lines: Vec<&str>) -> Self {
        let mut conversion_maps: HashMap<Category, ConversionMap> = HashMap::new();
        let mut map: Vec<&str> = Vec::new();

        for line in lines.iter() {
            // we have a map split
            if line.len() == 0 {
                Self::conversion_map_from_input(&mut conversion_maps, &mut map);
                map.clear();
            } else {
                map.push(*line);
            }
        }
        Self::conversion_map_from_input(&mut conversion_maps, &mut map);

        Almanac { maps: conversion_maps }
    }

    fn conversion_map_from_input(conversion_maps: &mut HashMap<Category, ConversionMap>, map: &mut Vec<&str>) {
        let conversion_map = ConversionMap::from(&map);
        let source_dest = conversion_map.source_category.clone();

        conversion_maps.insert(source_dest, conversion_map.into());
    }

    fn convert(&self, soil: u128) -> u128 {
        let mut map: &ConversionMap = self.maps.get(&Category::Seed).unwrap();
        let mut dest: u128 = soil;

        loop {
            dest = map.convert(dest);

            if map.dest_category == Category::Location {
                break;
            }

            map = self.maps.get(&map.dest_category).unwrap();
        }

        dest
    }
}

#[derive(Debug)]
struct ConversionMap {
    source_category: Category,
    dest_category: Category,
    conversion_rules: Vec<Rule>
}

impl ConversionMap {
    fn from(conversion_matrix: &Vec<&str>) -> Self {
        if conversion_matrix.len() < 2 {
            panic!("Impossible conversion matrix with {:?}", conversion_matrix);
        }

        let map_identifier_re = Regex::new(r"([a-z]+)-to-([a-z]+) map:").unwrap();
        let first_line = conversion_matrix.get(0).unwrap();
        let first_line_match = map_identifier_re.captures(first_line).unwrap();
        let source = first_line_match.get(1).unwrap().as_str();
        let dest = first_line_match.get(2).unwrap().as_str();

        let source_category = Category::from(source);
        let dest_category = Category::from(dest);
        let mut rules: Vec<Rule> = Vec::new();
        for conversion_line in conversion_matrix[1..conversion_matrix.len()].iter() {
            rules.push(Rule::from(conversion_line));
        }

        ConversionMap { source_category, dest_category, conversion_rules: rules }
    }

    fn convert(&self, source: u128) -> u128 {
        for rule in self.conversion_rules.iter() {
            if let Some(d) = rule.convert(source) {
                return d;
            }
        }

        source
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

impl Category {
    fn from(cat: &str) -> Self {
        match cat {
            "seed" => Category::Seed,
            "soil" => Category::Soil,
            "fertilizer" => Category::Fertilizer,
            "water" => Category::Water,
            "light" => Category::Light,
            "temperature" => Category::Temperature,
            "humidity" => Category::Humidity,
            "location" => Category::Location,
            _ => panic!("Unknown category: {}", cat)
        }
    }
}

#[derive(Debug)]
struct Rule {
    dest_start: u128,
    source_start: u128,
    range: u128
}

impl Rule {
    fn from(line: &str) -> Self {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() != 3 {
            panic!("Cant parse conversion map line: {}", line);
        }

        let ds: u128 = parts[0].parse().unwrap();
        let ss: u128 = parts[1].parse().unwrap();
        let r: u128 = parts[2].parse().unwrap();

        Rule { dest_start: ds, source_start: ss, range: r }
    }

    fn convert(&self, source: u128) -> Option<u128> {
        return if source >= self.source_start && source < self.source_start + self.range {
            let dest = self.dest_start + (source - self.source_start);

            Some(dest)
        } else {
            None
        }
    }
}
