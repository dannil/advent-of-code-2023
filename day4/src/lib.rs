use std::collections::{HashMap, HashSet};

struct Scratchcard {
    winning_numbers: HashSet<u32>,
    numbers_you_have: HashSet<u32>,
}

impl Scratchcard {
    fn points(&self) -> u32 {
        let matches: u32 = self.matches().len().try_into().unwrap();
        if matches > 0 {
            2_u32.pow(matches - 1)
        } else {
            0
        }
    }

    fn matches(&self) -> HashSet<&u32> {
        self.numbers_you_have
            .intersection(&self.winning_numbers)
            .collect()
    }
}

fn parse_scratchcard(input: &str) -> Scratchcard {
    let (colon_index, _) = input.char_indices().nth(input.find(':').unwrap()).unwrap();
    let (bar_index, _) = input.char_indices().nth(input.find('|').unwrap()).unwrap();

    let winning_numbers_str = &input[colon_index + 2..bar_index - 1];
    let numbers_you_have_str = &input[bar_index + 2..];

    let winning_numbers: HashSet<u32> = winning_numbers_str
        .split_whitespace()
        .map(|n| n.trim().parse::<u32>().unwrap())
        .collect();

    let numbers_you_have: HashSet<u32> = numbers_you_have_str
        .split_whitespace()
        .map(|n| n.trim().parse::<u32>().unwrap())
        .collect();

    Scratchcard {
        winning_numbers,
        numbers_you_have,
    }
}

pub fn part1(input: String) -> u32 {
    let scratchcards: Vec<Scratchcard> =
        input.lines().map(|line| parse_scratchcard(line)).collect();
    scratchcards.into_iter().map(|s| s.points()).sum()
}

pub fn part2(input: String) -> u32 {
    let original_scratchcards: Vec<Scratchcard> =
        input.lines().map(|line| parse_scratchcard(line)).collect();

    let mut all_scratchcards = HashMap::new();
    for original_card_number in 1..=original_scratchcards.len() {
        let original_card =
            Vec::from([original_scratchcards.get(original_card_number - 1).unwrap()]);
        all_scratchcards.insert(original_card_number, original_card);
    }

    for current_card_number in 1..=original_scratchcards.len() {
        let original_card = original_scratchcards.get(current_card_number - 1).unwrap();
        let copies_of_original_card = all_scratchcards.get_mut(&current_card_number).unwrap();
        let copies_to_win = copies_of_original_card.len();
        if copies_to_win > 0 {
            for next_card_number in
                current_card_number + 1..=current_card_number + original_card.matches().len()
            {
                for _ in 0..copies_to_win {
                    let copies_of_next_card = all_scratchcards.get_mut(&next_card_number).unwrap();
                    copies_of_next_card
                        .push(original_scratchcards.get(next_card_number - 1).unwrap());
                }
            }
        }
    }

    all_scratchcards
        .values()
        .map(|v| TryInto::<u32>::try_into(v.len()).unwrap())
        .sum()
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

    #[test]
    pub fn part1_example_input() {
        let file_content = fs::read_to_string(example_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 13);
    }

    #[test]
    pub fn part1_actual_input() {
        let file_content = fs::read_to_string(actual_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 26426);
    }

    #[test]
    pub fn part2_example_input() {
        let file_content = fs::read_to_string(example_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 30);
    }

    #[test]
    pub fn part2_actual_input() {
        let file_content = fs::read_to_string(actual_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 6227972);
    }
}
