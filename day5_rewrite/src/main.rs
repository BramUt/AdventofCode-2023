use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::Lines;

mod parsers;
use crate::parsers::{parse_seed_line, parse_almanac};




fn main() {
    let file_name = "sampledata.txt";
    let file_content = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error)
    };

    let mut lines = file_content.lines();
    let seed_numbers = parse_seed_line(lines.next().unwrap());
    let almanac = parse_almanac
}
