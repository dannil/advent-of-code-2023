use std::{cmp, collections::HashMap};

fn get_id(line: &String) -> u32 {
    let l_index = line.find("Game ").unwrap() + 5;
    let r_index = line.find(":").unwrap();
    return line[l_index..r_index].to_string().parse::<u32>().unwrap();
}

fn get_content(line: &String) -> String {
    let cubes_start = line.find(":").unwrap() + 1;
    let game_content = line[cubes_start..].trim().to_string();
    return game_content;
}

fn get_amount_color_pair_from_subset(subset: &String) -> (u32, &str) {
    let trimmed = subset.trim();
    let separator_index = trimmed.find(' ').unwrap();
    let amount = &trimmed[..separator_index].parse::<u32>().unwrap();
    let color = &trimmed[separator_index + 1..];
    return (*amount, color);
}

fn part1(input: String) -> u32 {
    fn is_game_possible(game: &String, max_cubes: &HashMap<String, u32>) -> bool {
        let sets = game.split(';');
        for subset in sets {
            let subsets = subset.split(',');
            for cube in subsets {
                let cube_string = &cube.to_string();
                let (amount, color) = get_amount_color_pair_from_subset(cube_string);

                let max = max_cubes.get(color).unwrap();
                if amount > *max {
                    return false;
                }
            }
        }
        return true;
    }

    let max_cubes_allowed: HashMap<String, u32> = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    let mut sum: u32 = 0;
    for line in input.lines() {
        let line_string = line.to_string();
        let game_id = get_id(&line_string);
        let game_content = get_content(&line_string);

        let possible = is_game_possible(&game_content, &max_cubes_allowed);
        if possible {
            sum += game_id;
        }
    }
    return sum;
}

fn part2(input: String) -> u32 {
    fn minimum_cubes(game: &String) -> u32 {
        let mut minimum_cubes_needed: HashMap<String, u32> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);
        let sets = game.split(';');
        for subset in sets {
            let subsets = subset.split(',');
            for cube in subsets {
                let cube_string = &cube.to_string();
                let (amount, color) = get_amount_color_pair_from_subset(cube_string);

                let current_cubes = minimum_cubes_needed.get(color).unwrap();
                let new_minimum = cmp::max(current_cubes, &amount);
                minimum_cubes_needed.insert(color.to_string(), *new_minimum);
            }
        }
        return minimum_cubes_needed.values().clone().product();
    }

    let mut sum: u32 = 0;
    for line in input.lines() {
        let line_string = line.to_string();
        let game_content = get_content(&line_string);

        let minimum = minimum_cubes(&game_content);
        sum += minimum;
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

    #[test]
    fn part1_actual_input() {
        let file_content = fs::read_to_string(actual_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 2528);
    }

    #[test]
    fn part1_example_input() {
        let file_content = fs::read_to_string(example_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_actual_input() {
        let file_content = fs::read_to_string(actual_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 67363);
    }

    #[test]
    fn part2_example_input() {
        let file_content = fs::read_to_string(example_input_path()).unwrap();
        let input = file_content.lines().collect::<Vec<&str>>().join("\n");
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 2286);
    }
}
