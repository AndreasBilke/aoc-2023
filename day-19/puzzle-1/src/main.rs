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

    let mut input_split = lines.split(|line|
        line.len() == 0
    );

    let rules = input_split.next().unwrap();
    let items = input_split.next().unwrap();

    let items: Vec<Item> = items.iter().map(|item| Item::from(item)).collect();
    let dm = DecisionMaker::from(rules);

    let sum: u32 = items.iter().filter(|item| {
        dm.accept_item(item)
    })
        .map(|item| item.value())
        .sum();

    println!("Result {}", sum);
}

#[derive(Debug)]
struct Item {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

impl Item {
    fn from(line: &str) -> Self {
        // {x=1679,m=44,a=2067,s=496}
        let re = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)}").unwrap();
        let matches = re.captures(line).unwrap();

        let x = matches.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let m = matches.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let a = matches.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let s = matches.get(4).unwrap().as_str().parse::<u32>().unwrap();

        Item { x, m, a, s }
    }

    fn value(&self) -> u32 {
        return self.x + self.m + self.a + self.s;
    }
}

struct DecisionMaker {
    rules: HashMap<String, RuleSet>
}

impl DecisionMaker {
    fn from(lines: &[&str]) -> Self {
        // px{a<2006:qkq,m>2090:A,rfg}

        let mut rules: HashMap<String, RuleSet> = HashMap::new();

        let re = Regex::new(r"([a-z]+)\{(.+)}").unwrap();
        for line in lines {
            let rule_split = re.captures(line).unwrap();
            let rule_name = String::from(rule_split.get(1).unwrap().as_str());
            let rule_set = RuleSet::from(rule_split.get(2).unwrap().as_str());

            rules.insert(rule_name, rule_set);
        }

        DecisionMaker { rules }
    }

    fn accept_item(&self, item: &Item) -> bool {
        let mut current_rule = self.rules.get("in").unwrap();
        loop {
            let next_rule = current_rule.evaluate(item);
            if next_rule == "A" {
                return true;
            }
            if next_rule == "R" {
                return false;
            }

            current_rule = self.rules.get(&next_rule).unwrap();
        }
    }
}

struct RuleSet {
    rule_set: Vec<Rule>
}

impl RuleSet {
    fn from(rules: &str) -> Self {
        let rule_set = rules.split(",").map(|rule|
            Rule::from(rule)
        ).collect();

        RuleSet { rule_set }
    }

    fn evaluate(&self, item: &Item) -> String {
        for rule in self.rule_set.iter() {
            if rule.evaluate(item) {
                return rule.next_rule.clone();
            }
        }

        panic!("Each rule set should end with some next rule");
    }
}

struct Rule {
    cmp: Option<Comparison>,
    next_rule: String
}

impl Rule {
    fn from(rule: &str) -> Self {
        let split: Vec<&str> = rule.split(":").collect();
        if split.len() == 1 {
            // no comparison. Just the next rule

            return Rule { cmp: None, next_rule: split.get(0).unwrap().to_string() }
        }

        let next_rule = split.get(1).unwrap().to_string();
        let cmp = Some(Comparison::from(split.get(0).unwrap()));

        Rule { cmp, next_rule }
    }

    fn evaluate(&self, item: &Item) -> bool {
        return if let Some(cmp) = &self.cmp {
            cmp.compare(item)
        } else {
            true // no comparison needed, is a just forward rule
        }
    }
}

enum Comparison {
    Less(Variable, u32),
    Larger(Variable, u32)
}

impl Comparison {
    fn from(cmp: &str) -> Self {
        let chars: Vec<char> = cmp.chars().collect();
        let variable = Variable::from(chars[0]);
        let amount = cmp.split_at(2).1.parse::<u32>().unwrap();

        match chars[1] {
            '<' => Comparison::Less(variable, amount),
            '>' => Comparison::Larger(variable, amount),
            _ => panic!("Unknown comparator")
        }
    }

    fn compare(&self, item: &Item) -> bool {
        match self {
            Self::Less(variable, number) => {
                match variable {
                    Variable::X => &item.x < number,
                    Variable::M => &item.m < number,
                    Variable::A => &item.a < number,
                    Variable::S => &item.s < number
                }
            },
            Self::Larger(variable, number) => {
                match variable {
                    Variable::X => &item.x > number,
                    Variable::M => &item.m > number,
                    Variable::A => &item.a > number,
                    Variable::S => &item.s > number
                }
            }
        }
    }
}

enum Variable {
    X,
    M,
    A,
    S
}

impl Variable {
    fn from(v: char) -> Self {
        match v {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("Unknown variable: {}", v)
        }
    }
}