use core::num;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(PartialEq, PartialOrd, Debug)]
enum Type {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
}

impl Hand {
    pub fn as_number_representation(&self) -> Vec<u64> {
        let map: HashMap<char, u64> =
            HashMap::from([('T', 10), ('J', 1), ('Q', 12), ('K', 13), ('A', 14)]);
        let mut number_representation = Vec::new();
        for card in &self.cards {
            if map.contains_key(card) {
                let mapped = map.get(card).unwrap();
                number_representation.push(*mapped);
            } else {
                number_representation.push(card.to_digit(10).map(u64::from).unwrap());
            }
        }
        number_representation
    }

    pub fn get_type(&self) -> Type {
        let numbers = &self.as_number_representation();

        let mut occurances = HashMap::<u64, u64>::new();
        for n in numbers.into_iter().filter(|n| **n != 1) {
            let entry = occurances.entry(*n).or_insert(0);
            *entry += 1;
        }

        for n in numbers.into_iter().filter(|n| **n == 1) {
            // For every joker, add 1 to every occurance since it can be used in any combination
            for (key, value) in occurances.clone() {
                let entry = occurances.get_mut(&key).unwrap();
                *entry += 1;
            }
        }

        let values: HashSet<u64> = occurances.values().cloned().collect();
        if values.contains(&5) {
            return Type::FiveOfAKind;
        } else if values.contains(&4) {
            return Type::FourOfAKind;
        } else if values.contains(&3) && values.contains(&2) {
            return Type::FullHouse;
        } else if values.contains(&3) {
            return Type::ThreeOfAKind;
        } else if occurances.iter().filter(|&(_, v)| *v == 2).count() == 2 {
            return Type::TwoPair;
        } else if values.contains(&2) {
            return Type::OnePair;
        }
        Type::HighCard
    }

    pub fn is_stronger_than(&self, other: &Hand) -> Ordering {
        let self_type = &self.get_type();
        let other_type = &other.get_type();
        if self_type > other_type {
            return Ordering::Greater;
        } else if self_type < other_type {
            return Ordering::Less;
        }
        let self_numbers = &self.as_number_representation();
        let other_numbers = &other.as_number_representation();
        for i in 0..self_numbers.len() {
            let self_number = self_numbers[i];
            let other_number = other_numbers[i];
            if self_number > other_number {
                return Ordering::Greater;
            } else if self_number < other_number {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

pub fn part1(input: Vec<String>) -> u64 {
    let mut all_hands = Vec::new();
    for i in 0..input.len() {
        let row = &input[i];
        let (separator_index, _) = row.char_indices().nth(row.find(' ').unwrap()).unwrap();

        let hand_str = &row[0..separator_index];
        let bid_str = &row[separator_index + 1..];

        let cards = hand_str.chars().collect();
        let hand = Hand {
            cards,
            bid: bid_str.parse::<u64>().unwrap(),
        };

        // println!(
        //     "Hand: {:?}, bid: {}, type: {:?}",
        //     hand.cards,
        //     hand.bid,
        //     hand.get_type()
        // );

        // println!("Values: {:?}", hand.as_number_representation());

        all_hands.push(hand);
    }

    all_hands.sort_by(|a, b| a.is_stronger_than(b));

    let total_winnings = all_hands
        .iter()
        .enumerate()
        .fold(0, |s, (i, j)| s + j.bid * (i as u64 + 1));

    for hand in all_hands {
        println!("Cards: {:?}, type: {:?}", hand.cards, hand.get_type())
    }


    println!("{:?}", total_winnings);

    total_winnings
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn inputs_path() -> &'static str {
        "inputs"
    }

    fn example_input_path() -> String {
        format!("{}/example.txt", inputs_path())
    }

    fn actual_input_path() -> String {
        format!("{}/actual.txt", inputs_path())
    }

    // #[test]
    // pub fn part1_example_input() {
    //     let file_content = fs::read_to_string(example_input_path()).unwrap();
    //     let input = file_content.lines().map(|l| l.to_string()).collect();
    //     let result = part1(input);
    //     println!("{}", result);
    //     assert_eq!(result, 6440);
    // }

    // #[test]
    // pub fn part1_actual_input() {
    //     let file_content = fs::read_to_string(actual_input_path()).unwrap();
    //     let input = file_content.lines().map(|l| l.to_string()).collect();
    //     let result = part1(input);
    //     println!("{}", result);
    //     assert_eq!(result, 248113761);
    // }

    #[test]
    pub fn part2_example_input() {
        let file_content = fs::read_to_string(example_input_path()).unwrap();
        let input = file_content.lines().map(|l| l.to_string()).collect();
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 5905);
    }

    #[test]
    pub fn part2_actual_input() {
        let file_content = fs::read_to_string(actual_input_path()).unwrap();
        let input = file_content.lines().map(|l| l.to_string()).collect();
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 246093337);
    }
}
