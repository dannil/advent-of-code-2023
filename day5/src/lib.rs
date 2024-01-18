use std::cmp::min;

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug)]
struct ResourceMap {
    ranges: Vec<Range>,
}

impl ResourceMap {
    pub fn get_destination(&self, source: u64) -> u64 {
        for range in &self.ranges {
            let (destination_start, source_start) = (range.destination_start, range.source_start);
            let source_end = source_start + range.length;
            if source >= source_start && source < source_end {
                let source_start_offset = source - source_start;
                return destination_start + source_start_offset;
            }
        }
        source
    }
}

fn extract_ranges(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut ranges_str: Vec<Vec<String>> = Vec::new();
    let mut start_index = 0;
    for index in 0..lines.len() {
        let line = &lines[index];
        if line.ends_with("map:") {
            // Beginning of map
            start_index = index + 1;
        } else if line.is_empty() || index + 1 == lines.len() {
            // End of map
            let end_index = index;
            let range_str: Vec<String> = lines[start_index..end_index].to_vec();
            ranges_str.push(range_str);
        }
    }
    ranges_str
}

fn parse_resource_map(ranges_str: &Vec<String>) -> ResourceMap {
    let mut ranges: Vec<Range> = Vec::new();
    for index in 0..ranges_str.len() {
        let line = &ranges_str[index];

        let range_numbers: Vec<u64> = line
            .split_whitespace()
            .map(|n| n.trim().parse::<u64>().unwrap())
            .collect();

        let range = Range {
            destination_start: range_numbers[0],
            source_start: range_numbers[1],
            length: range_numbers[2],
        };

        ranges.push(range);
    }
    ResourceMap { ranges }
}

pub fn part1(input: Vec<String>) -> u64 {
    let seeds: Vec<u64> = input[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.trim().parse::<u64>().unwrap())
        .collect();

    let lines: Vec<String> = input.into_iter().skip(1).map(|l| l).collect();

    let resource_maps: Vec<ResourceMap> = extract_ranges(&lines)
        .into_iter()
        .map(|r| parse_resource_map(&r))
        .collect();

    let mut lowest = u64::MAX;
    for seed in seeds {
        let mut location_number = seed;
        for map in &resource_maps {
            location_number = map.get_destination(location_number);
        }
        lowest = min(lowest, location_number);
    }
    lowest
}

pub fn part2(input: Vec<String>) -> u64 {
    let seed_ranges: Vec<u64> = input[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.trim().parse::<u64>().unwrap())
        .collect();

    let lines: Vec<String> = input.into_iter().skip(1).map(|l| l).collect();

    let resource_maps: Vec<ResourceMap> = extract_ranges(&lines)
        .into_iter()
        .map(|r| parse_resource_map(&r))
        .collect();

    // This solution brute-forces the lowest location number for every single seed
    // in every seed range.
    // It isn't quick but works; running the release build on a Ryzen 3700X
    // with 32 GB of RAM @ 3200 MHz, it produces the correct output in the interval
    // of 150 and 170 seconds, depending on background load.
    let mut lowest = u64::MAX;
    for i in (0..seed_ranges.len()).step_by(2) {
        let start = seed_ranges[i];
        let end = start + seed_ranges[i + 1];
        for seed in start..end {
            let mut location_number = seed;
            for map in &resource_maps {
                location_number = map.get_destination(location_number);
            }
            lowest = min(lowest, location_number);
        }
    }
    lowest
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
        let input = file_content.lines().map(|l| l.to_string()).collect();
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 35);
    }

    #[test]
    pub fn part1_actual_input() {
        let file_content = fs::read_to_string(actual_input_path()).unwrap();
        let input = file_content.lines().map(|l| l.to_string()).collect();
        let result = part1(input);
        println!("{}", result);
        assert_eq!(result, 111627841);
    }

    #[test]
    pub fn part2_example_input() {
        let file_content = fs::read_to_string(example_input_path()).unwrap();
        println!("{}", file_content);
        let input = file_content.lines().map(|l| l.to_string()).collect();
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 46);
    }

    #[test]
    pub fn part2_actual_input() {
        let file_content = fs::read_to_string(actual_input_path()).unwrap();
        let input = file_content.lines().map(|l| l.to_string()).collect();
        let result = part2(input);
        println!("{}", result);
        assert_eq!(result, 69323688);
    }
}
