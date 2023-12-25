use std::env;
use std::fs;
use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};
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

    let the_storm: Vec<Hail> = lines.iter().map(|line| Hail::from(line) ).collect();

    let mut counter = 0;
    for pair in the_storm.iter().cartesian_product(the_storm.iter()) {
        if pair.0 != pair.1 {
            if let Some(intersection) = pair.0.future_intersect(pair.1) {
                if intersection.0 >= 200000000000000. && intersection.0 <= 400000000000000.
                    && intersection.1 >= 200000000000000. && intersection.1 <= 400000000000000. {
                    counter = counter + 1;
                }
            }
        }
    }

    println!("We got {} intersecting hail paths", counter / 2);
}

#[derive(PartialEq)]
struct Hail  {
    s_x: f64,
    s_y: f64,
    s_z: f64,

    v_x: f64,
    v_y: f64,
    v_z: f64
}

impl Hail {
    fn from(line: &str) -> Self {
        let re = Regex::new(r"(-?[0-9]+),\s+(-?[0-9]+),\s+(-?[0-9]+)\s+@\s+(-?[0-9]+),\s+(-?[0-9]+),\s+(-?[0-9]+)").unwrap();
        let m = re.captures(line).unwrap();

        let s_x = m.get(1).unwrap().as_str().parse::<f64>().unwrap();
        let s_y = m.get(2).unwrap().as_str().parse::<f64>().unwrap();
        let s_z = m.get(3).unwrap().as_str().parse::<f64>().unwrap();

        let v_x = m.get(4).unwrap().as_str().parse::<f64>().unwrap();
        let v_y = m.get(5).unwrap().as_str().parse::<f64>().unwrap();
        let v_z = m.get(6).unwrap().as_str().parse::<f64>().unwrap();

        Hail { s_x, s_y, s_z, v_x, v_y, v_z }
    }

    fn future_intersect(&self, other: &Hail) -> Option<(f64, f64)> {
        // Example solving
        // 18 - 19 = -2x - (-1)y
        // 19 - 13 = 1x - (-1)y
        // ==>
        // -1 = -2x + y
        //  6 = x + y

        // Solutions at: 7/3; 11/3 ==> 2.33; 3.66
        // Intersection: 19 + (7/3)*(-2) = 14.33
        //               13 + (7/3)*(1)  = 15.33


        // Hailstone A: 19, 13, 30 @ -2, 1, -2
        // Hailstone B: 18, 19, 22 @ -1, -1, -2
        // Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).

        let m = Matrix2::new(
            self.v_x, - other.v_x,
            self.v_y, - other.v_y
        );
        let decomposition = m.lu();
        let b = Vector2::new(
            other.s_x - self.s_x, other.s_y - self.s_y
        );
        if let Some(result) = decomposition.solve(&b) {
            // only report an intersection if it happens in the future!
            return if result.x >= 0. && result.y >= 0. {
                let intersection = (self.s_x + result.x * self.v_x, self.s_y + result.x * self.v_y);
                Some(intersection)
            } else {
                None
            }

        }

        None
    }
}