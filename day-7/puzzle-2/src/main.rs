use std::cmp::Ordering;
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

    let mut hands: Vec<Hand> = lines.iter().map(|line|
        Hand::from(line)
    ).collect();
    hands.sort();
    let ranks: u64 = hands.iter().enumerate().map(|p| {
        let v = (p.0 + 1) as u64 * p.1.bid as u64;
        v
    }).sum();

    println!("Result {:?}", ranks);
}

#[derive(Debug, Eq)]
struct Hand {
    bid: u32,
    cards: [Card; 5],
    kind: Kind
}

impl Hand {
    fn from(line: &str) -> Self {
        let mut split = line.split(" ");

        let cards = Self::cards(split.nth(0).unwrap());
        let bid = split.nth(0).unwrap().parse::<u32>().unwrap();
        let kind = Kind::from(cards);

        Hand { bid, cards, kind }
    }

    fn cards(cards_str: &str) -> [Card; 5] {
        let mut cards: [Card; 5] = [Card::Two; 5];
        let mut cards_split = cards_str.chars();

        cards[0] = Card::from(cards_split.next().unwrap());
        cards[1] = Card::from(cards_split.next().unwrap());
        cards[2] = Card::from(cards_split.next().unwrap());
        cards[3] = Card::from(cards_split.next().unwrap());
        cards[4] = Card::from(cards_split.next().unwrap());

        cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind != other.kind {
            return self.kind.partial_cmp(&other.kind).unwrap();
        }
        // check pairwise for same card values
        for i in 0 .. 5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].partial_cmp(&other.cards[i]).unwrap();
            }
        }

        Ordering::Equal
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.kind != other.kind {
            return false;
        }
        // check pairwise for same card values
        for i in 0 .. 5 {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }

        true
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
enum Kind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

impl Kind {
    fn from(cards: [Card; 5]) -> Self {
        let mut card_values: [u32; 13] = [0; 13];

        for card in cards {
            let card_pos = card as usize;
            card_values[card_pos] = card_values[card_pos] + 1;
        }
        let joker_count = card_values[0] as i32;
        card_values[0] = 0;

        card_values.sort();
        card_values.reverse();
        card_values[0] = card_values[0] + joker_count as u32;

        // now check all possibilities
        if card_values[0] == 5 {
            return Kind::FiveOfAKind;
        } else if card_values[0] == 4 { // pos 1 MUST be 1
            return Kind::FourOfAKind;
        } else if card_values[0] == 3 && card_values[1] == 2 {
            return Kind::FullHouse;
        } else if card_values[0] == 3 && card_values[1] == 1 && card_values[2] == 1 {
            return Kind::ThreeOfAKind;
        } else if card_values[0] == 2 && card_values[1] == 2 { // pos 2 MUST be 1
            return Kind::TwoPair;
        } else if card_values[0] == 2 && card_values[1] == 1 && card_values[2] == 1 { // pos 3 MUST be 1
            return Kind::OnePair;
        } else {
            Kind::HighCard
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
enum Card {
    A = 12,
    K = 11,
    Q = 10,
    J = 0,
    T = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1
}

impl Card {
    fn from(item: char) -> Self {
        match item {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            x => panic!("Unknown card: {}", x)
        }
    }
}
