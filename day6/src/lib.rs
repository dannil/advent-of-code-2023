fn get_digits(str: &str) -> Vec<u64> {
    str.split_whitespace()
        .map(|n| n.trim().parse::<u64>().unwrap())
        .collect()
}

pub fn number_of_ways_to_beat(time: u64, distance: u64) -> u64 {
    let mut new_records: u64 = 0;
    for speed in 0..time {
        let travel_time = time - speed;
        let total_distance = speed * travel_time;
        if total_distance > distance {
            new_records += 1;
        }
    }
    new_records
}

pub fn part1(input: Vec<String>) -> u64 {
    let times_str = input[0].strip_prefix("Time:").unwrap();
    let times = get_digits(times_str);

    let distances_str = input[1].strip_prefix("Distance:").unwrap();
    let distances = get_digits(distances_str);

    let mut all_ways_to_beat: Vec<u64> = Vec::new();
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let ways_to_beat = number_of_ways_to_beat(time, distance);
        if ways_to_beat > 0 {
            all_ways_to_beat.push(ways_to_beat);
        }
    }
    all_ways_to_beat.into_iter().product()
}

pub fn part2(input: Vec<String>) -> u64 {
    let times_str = input[0].strip_prefix("Time:").unwrap();
    let times: Vec<String> = get_digits(times_str)
        .iter()
        .map(|&n| n.to_string())
        .collect();

    let distances_str = input[1].strip_prefix("Distance:").unwrap();
    let distances: Vec<String> = get_digits(distances_str)
        .iter()
        .map(|&n| n.to_string())
        .collect();

    let time = times.join("").parse::<u64>().unwrap();
    let distance = distances.join("").parse::<u64>().unwrap();

    number_of_ways_to_beat(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_example_input() {
        let input = example_input()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 288);
    }

    #[test]
    pub fn part1_actual_input() {
        let input: Vec<String> = actual_input()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 140220);
    }

    #[test]
    pub fn part2_example_input() {
        let input = example_input()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 71503);
    }

    #[test]
    pub fn part2_actual_input() {
        let input: Vec<String> = actual_input()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 39570185);
    }

    fn example_input() -> String {
        "
        Time:      7  15   30
        Distance:  9  40  200
        "
        .to_string()
    }

    fn actual_input() -> String {
        "
        Time:        53     83     72     88
        Distance:   333   1635   1289   1532
        "
        .to_string()
    }
}
