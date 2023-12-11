use std::collections::HashSet;
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

    let g = Galaxy::from(&lines);
    let mut sum: u64 = g.all_pair_distance().iter().sum();
    sum = sum / 2u64; // i counted everything twice
    println!("Sum is {sum}");
}

struct Galaxy {
    stars: HashSet<(u64, u64)>,
    empty_rows: HashSet<u64>,
    empty_columns: HashSet<u64>
}

impl Galaxy {
    fn from(lines: &Vec<&str>) -> Self {
        let mut stars: HashSet<(u64, u64)> = HashSet::new();

        for (row, line) in lines.iter().enumerate() {
            for (column, item) in line.chars().enumerate() {
                if item == '#' {
                    stars.insert((column as u64, row as u64));
                }
            }
        }

        // find empty columns
        let mut all_x: Vec<u64> = stars.iter().map(|item| item.0 ).collect();
        let empty_columns = Self::extract_empty_fields(&mut all_x);

        // find empty rows
        let mut all_y: Vec<u64> = stars.iter().map(|item| item.1 ).collect();
        let empty_rows = Self::extract_empty_fields(&mut all_y);

        Galaxy { stars, empty_rows, empty_columns }
    }

    fn all_pair_distance(&self) -> Vec<u64> {
        let mut distances: Vec<u64> = Vec::new();

        for star1 in self.stars.iter() {
            for star2 in self.stars.iter() {
                if star1.0 == star2.0 && star1.1 == star2.1 {
                    continue;
                }

                let dist = self.compute_distance(star1, star2);
                distances.push(dist);
            }
        }

        distances
    }

    fn compute_distance(&self, from: &(u64, u64), to: &(u64, u64)) -> u64 {
        let x_distance = from.0.abs_diff(to.0);
        let y_distance = from.1.abs_diff(to.1);

        // the universe is expanding
        // how many empty rows/columns are in between both

        let min_x = from.0.min(to.0);
        let max_x = from.0.max(to.0);

        let min_y = from.1.min(to.1);
        let max_y = from.1.max(to.1);

        // count rows between min/max
        let rows_between = self.empty_rows.iter().filter(|&&item| {
            min_y < item && item < max_y
        }).count();
        let columns_between = self.empty_columns.iter().filter(|&&item| {
            min_x < item && item < max_x
        }).count();

        x_distance + y_distance + rows_between as u64 + columns_between as u64
    }

    fn extract_empty_fields(all_elements: &mut Vec<u64>) -> HashSet<u64> {
        let mut empty_fields: HashSet<u64> = HashSet::new();

        all_elements.sort();
        all_elements.dedup();
        for p in all_elements.windows(2) {
            // the difference between both is the gap in rows
            let diff = p[1] - p[0];
            for i in 1..diff {
                empty_fields.insert(p[0] + i);
            }
        }

        empty_fields
    }
}
