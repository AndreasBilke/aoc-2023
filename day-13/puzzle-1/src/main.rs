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

    let maps: Vec<&[&str]> = lines.split(|&line|
        line.len() == 0
    ).collect();

    let maps: Vec<Map> = maps.iter().map(|&str_map|
        Map::from(str_map)
    ).collect();

    let sum: u32 = maps.iter().map(|m|
        m.summarize()
    ).sum();

    println!("Result {sum}");
}

#[derive(Debug)]
struct Map {
    // data[i] is the ith row
    data: Vec<Vec<Type>>
}

impl Map {
    fn from(lines: &[&str]) -> Self {
        let data = lines.iter().map(|&line| {
           line.chars().map(|c|
                Type::from(c)
           ).collect()
        }).collect();

        Map { data }
    }

    fn summarize(&self) -> u32 {
        // for now, we only try horizontal

        if let Some(s) = Self::find_reflection_line(&self.data) {
            return (s as u32 + 1) * 100 // horizontal line
        }

        // we have no horizontal line, try vertical. To keep our beautiful logic, we transpose
        // our data and then do the search again
        let data = Self::transpose(&self.data);
        if let Some(s) = Self::find_reflection_line(&data) {
            return s as u32 + 1 // vertical line
        }

        panic!("Whoops. I cannot find any reflection line");
    }

    fn transpose(data: &Vec<Vec<Type>>) -> Vec<Vec<Type>> {
        let mut transposed_data: Vec<Vec<Type>> = Vec::new();

        // for every column i, fetch every ith element from the sub vectors
        let sub_vector_length = data.first().unwrap().len();

        for i in 0 .. sub_vector_length {
            let ith_data: Vec<Type> = data.iter().map(|v|
                Type::clone(v.get(i).unwrap())
            ).collect();
            transposed_data.push(ith_data);
        }

        transposed_data
    }

    // Find a horizontal reflection line. We try all possible rows and
    // look for mirrored rows.
    //
    // We start in the middle of input and work +/-1 around it until we reach the border
    fn find_reflection_line(map: &Vec<Vec<Type>>) -> Option<usize> {
        for r in 0 .. map.len() {
            if Self::try_line(map, r as i32) {
                return Some(r);
            }
        }

        None
    }

    fn try_line(map: &Vec<Vec<Type>>, row: i32) -> bool {
        // maximal numbers of rows we could try before we reach one of the borders
        let max_matching_rows = (row + 1).min(map.len() as i32 - row - 1);

        // there is nothing to match against
        if max_matching_rows == 0 {
            return  false;
        }

        for offset in 0 .. max_matching_rows {
            let t_row = row - offset;
            let b_row = row + 1 + offset;

            if !Self::rows_equal(map, t_row, b_row) {
                return false;
            }
        }

        // all possible rows are equal, we have a match
        true
    }

    fn rows_equal(map: &Vec<Vec<Type>>, top_row: i32, bottom_row: i32) -> bool {
        if top_row < 0 || bottom_row >= (map.len() as i32) {
            return false;
        }

        let t_row: &Vec<Type> = map.get(top_row as usize).unwrap();
        let b_row: &Vec<Type> = map.get(bottom_row as usize).unwrap();

        t_row.eq(b_row)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Type {
    Ash,
    Rock
}

impl Type {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Unknown type {}", c)
        }
    }
}
