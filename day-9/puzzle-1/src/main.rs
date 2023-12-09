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

    let oasis = Oasis::from(&lines);
    let final_result: i64 = oasis.next_values().iter().sum();

    println!("Result is {final_result}");
}

struct Oasis {
    sensor_readings: Vec<Vec<i64>>
}

impl Oasis {
    fn from(lines: &Vec<&str>) -> Self {
        let mut sensor_readings: Vec<Vec<i64>> = Vec::new();

        for line in lines {
            let readings = Self::extract_reading(line);

            sensor_readings.push(readings);
        }

        Oasis { sensor_readings }
    }

    fn next_values(&self) -> Vec<i64> {
        self.sensor_readings.iter().map(|reading| {
            self.next_value(reading)
        }).collect()
    }

    fn next_value(&self, reading: &Vec<i64>) -> i64 {
        let mut expanded_readings: Vec<Vec<i64>> = Vec::new();
        expanded_readings.push(reading.clone());
        let mut current_reading = expanded_readings.last().unwrap();

        // expand reading until we have all zeros
        loop {
            let next_reading = self.explode_reading(&current_reading);
            let next_reading_sum: i64 = next_reading.iter().sum();
            expanded_readings.push(next_reading);
            current_reading = expanded_readings.last().unwrap();

            if next_reading_sum == 0 {
                break;
            }
        }

        // now start by the last expanded reading and generate one additional item for the previous
        // expanded reading

        for i in (1..expanded_readings.len()).rev() {
            let mut sum = 0;
            {
                let reading = expanded_readings.get(i).unwrap();
                sum = sum + reading.last().unwrap();
            }
            let previous_reading = expanded_readings.get_mut(i - 1).unwrap();
            sum = sum + previous_reading.last().unwrap();

            previous_reading.push(sum);
        }

        *expanded_readings.first().unwrap().last().unwrap()
    }

    fn explode_reading(&self, reading: &Vec<i64>) -> Vec<i64> {
        let mut new_reading: Vec<i64> = Vec::new();

        for window in reading.windows(2) {
            new_reading.push(window[1] - window[0]);
        }

        new_reading
    }

    fn extract_reading(line: &str) -> Vec<i64> {
        line.split_whitespace().map(|item|
            item.parse::<i64>().unwrap()
        ).collect()
    }
}