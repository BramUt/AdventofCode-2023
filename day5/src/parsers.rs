use std::collections::HashMap;
use std::str::Lines;

use crate::ElfMap;
use crate::structs::MappedRange;

pub fn parse_seed_line(seed_line: &str) -> Vec<i128> {
    let (_, nums_str) = seed_line.split_once(":").unwrap();
    match nums_str.split_whitespace().map(|n| n.parse::<i128>()).collect() {
        Ok(nums) => nums,
        Err(error) => panic!("Error parsing seed line numbers: {error:?}")
    }
}

pub fn parse_map_header_line (line: &str) -> ElfMap {
    let (source, _) = line.strip_suffix(" map:").unwrap().split_once("-to-").unwrap();
    ElfMap::new(source)
}

pub fn parse_map_content_line (line: &str) -> (i128, i128, i128) {
    let numbers: Vec<i128> = match line.split_whitespace().map(|n| n.parse::<i128>()).collect() {
        Ok(nums) => nums,
        Err(error) => panic!("Error parsing seed line numbers: {error:?}")
    };
    (numbers[0], numbers[1], numbers[2])
}

pub fn parse_almanac (mut lines: Lines) -> HashMap<String, ElfMap> {
    // Skip the first blank line.
    let _ = lines.next().unwrap();
    // Create the first map.
    let mut cur_map = parse_map_header_line(lines.next().unwrap());

    let mut almanac: HashMap<String, ElfMap> = HashMap::new();
    
    for line in lines {
        let c: Vec<char> = line.chars().collect();
        if line.is_empty() {

        } else if c[0].is_alphabetic() {
            // Sort ranges for part get_from_range.
            cur_map.mapped_ranges.sort_by_key(|m| m.source_start);
            almanac.insert(cur_map.source.to_owned(), cur_map);
            cur_map = parse_map_header_line(line);
        } else if  c[0].is_numeric() {
            let (dest_range_start, source_range_start, range_len) = parse_map_content_line(line);
            cur_map.mapped_ranges.push(MappedRange { 
                source_start: source_range_start, 
                dest_start: dest_range_start, 
                length: range_len 
            })
        }
    }
    cur_map.mapped_ranges.sort_by_key(|m| m.source_start);
    almanac.insert(cur_map.source.to_owned(), cur_map);
    almanac
}