use std::str::Lines;

/// Parses the first line form the input file.
pub fn parse_seed_line(seed_line: &str) -> Vec<i128> {
    let (_, nums_str) = seed_line.split_once(":").unwrap();
    match nums_str.split_whitespace().map(|n| n.parse::<i128>()).collect() {
        Ok(nums) => nums,
        Err(error) => panic!("Error parsing seed line numbers: {error:?}")
    }
}

/// Splits a map content line into three numbers
fn parse_map_content_line (line: &str) -> (i128, i128, i128) {
    let numbers: Vec<i128> = match line.split_whitespace().map(|n| n.parse::<i128>()).collect() {
        Ok(nums) => nums,
        Err(error) => panic!("Error parsing map content line numbers: {error:?}")
    };
    (numbers[0], numbers[1], numbers[2])
}

pub fn parse_almanac(mut lines: Lines) {

}

