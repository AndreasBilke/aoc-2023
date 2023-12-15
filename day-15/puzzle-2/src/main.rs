use std::collections::HashMap;
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

    let elements: Vec<&str> = lines[0].split(",").collect();

    let mut boxes: HashMap<u128, Box> = HashMap::new();
    elements.iter().for_each(|item| {
        let operation = Operation::from(item);
        let box_id = match &operation {
            Operation::New(label, _) => to_hash(label.as_str()),
            Operation::Remove(label) => to_hash(label.as_str())
        };

        if !boxes.contains_key(&box_id) {
            boxes.insert(box_id, Box::new(box_id));
        }
        let box_item = boxes.get_mut(&box_id).unwrap();

        match &operation {
            Operation::Remove(label) => {
                box_item.remove(label.as_str());
            },
            Operation::New(label, value) => {
                let lens = Lens { label: String::from(label), value: value.clone() };
                box_item.add_or_replace(lens);
            }
        }
    });

    let result: u128 = boxes.iter().map(|box_item| {
        let sum: u128 = box_item.1.focal_power().iter().sum();

        sum
    }).sum();

    println!("Result {result}");
}

fn to_hash(element: &str) -> u128 {
    element.chars().fold(0u128, |agg, item|
        ((agg + item as u128) * 17) % 256
    )
}

struct Box {
    id: u128,
    lenses: Vec<Lens>
}

impl Box {
    fn new(id: u128) -> Self {
        let lenses: Vec<Lens> = Vec::new();

        Box { id, lenses }
    }

    fn remove(&mut self, label: &str) {
        if let Some(pos) = self.lenses.iter().position(|lens|
            lens.label.as_str() == label
        ) {
            self.lenses.remove(pos);
        }
    }

    fn add_or_replace(&mut self, lens: Lens) {
        if let Some(pos) = self.lenses.iter().position(|item|
            item.label == lens.label
        ) {
            self.lenses[pos] = lens;
        } else {
            self.lenses.push(lens);
        }
    }

    fn focal_power(&self) -> Vec<u128> {
        let mut power_values: Vec<u128> = Vec::new();

        for (index, item) in self.lenses.iter().enumerate() {
            let power = (self.id + 1) * (index as u128 + 1) * item.value;
            power_values.push(power);
        }

        power_values
    }
}

struct Lens {
    label: String,
    value: u128
}

enum Operation {
    Remove(String),
    New(String, u128)
}

impl Operation {
    fn from(item: &str) -> Self {
        if item.ends_with("-") {
            let label = item[0..item.len() - 1].to_string();

            return Operation::Remove(label);
        }

        let pos = item.find("=").unwrap();
        let label = item[0..pos].to_string();
        let value = item[pos + 1 .. item.len()].parse::<u128>().unwrap();

        Operation::New(label, value)
    }
}
