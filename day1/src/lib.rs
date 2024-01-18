use std::collections::HashMap;

use regex::Regex;

fn part1(input: String) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut digits = Vec::new();
        for character in line.chars() {
            if character.is_digit(10) {
                digits.push(character.to_digit(10).unwrap());
            }
        }
        if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
            let calibration_value = format!("{}{}", first.to_string(), last.to_string());
            sum += calibration_value.parse::<u32>().unwrap();
        }
    }
    return sum;
}

fn part2(input: String) -> u32 {
    fn get_digit(line: &str, regex: &Regex, mappings: &HashMap<String, u32>) -> u32 {
        let first_group = regex.captures(line).unwrap().get(1);
        let parsed_string = first_group.unwrap().as_str();

        if let Ok(parsed_int) = parsed_string.parse::<u32>() {
            return parsed_int;
        } else {
            return mappings.get(parsed_string).unwrap().clone();
        }
    }

    let mappings: HashMap<String, u32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);

    let mappings_reversed: HashMap<String, u32> = mappings
        .clone()
        .into_iter()
        .map(|(key, value)| (key.chars().rev().collect(), value))
        .collect();

    let digits_as_strings_matches = mappings
        .clone()
        .into_keys()
        .collect::<Vec<String>>()
        .join("|")
        .to_string();

    let digits_as_strings_matches_reversed = mappings_reversed
        .clone()
        .into_keys()
        .collect::<Vec<String>>()
        .join("|")
        .to_string();

    let regex = Regex::new(&format!(r"({}|[0-9])", digits_as_strings_matches)).unwrap();
    let regex_reversed =
        Regex::new(&format!(r"({}|[0-9])", digits_as_strings_matches_reversed)).unwrap();

    let mut sum: u32 = 0;
    for line in input.lines() {
        let first = get_digit(line, &regex, &mappings);

        let line_reversed = &line.chars().rev().collect::<String>();
        let last = get_digit(line_reversed, &regex_reversed, &mappings_reversed);

        let calibration_value = format!("{}{}", first.to_string(), last.to_string());
        sum += calibration_value.parse::<u32>().unwrap();
    }
    return sum;
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

    fn custom_input_path() -> String {
        format!("{}/custom.txt", inputs_path())
    }

    #[test]
    fn part1_actual_input() {
        let file_content = fs::read_to_string(actual_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 55017);
    }

    #[test]
    fn part2_example_input() {
        let file_content = fs::read_to_string(example_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 281);
    }

    #[test]
    fn part2_custom_input() {
        let file_content = fs::read_to_string(custom_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 195);
    }

    #[test]
    fn part2_actual_input() {
        let file_content = fs::read_to_string(actual_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 53539);
    }
}
