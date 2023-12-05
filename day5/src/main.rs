use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::Lines;

mod structs;
use crate::structs::{ValueRange, MappedRange};


struct ElfMap {
    source: String,
    dest: String,
    mapped_ranges: Vec<MappedRange>
}

impl ElfMap {
    fn new(source: &str, dest: &str) -> Self {
        Self {
            source: source.to_owned(),
            dest: dest.to_owned(),
            mapped_ranges: Vec::new()
        }
    }

    fn get(&self, s: i128) -> i128 {

        let mut range_with_value: Option<MappedRange> = None;
        for range in &self.mapped_ranges {
            if range.num_in_source_range(s) {
                range_with_value = Some(range.clone())
            }
        }

        match range_with_value {
            Some(r) => r.get_mapped_value(s),
            None => s
        }
    }

    fn get_from_range(&self, value_range: ValueRange) -> Vec<ValueRange> {
        let mut temp_range = value_range.clone();

        let mut mapped_values: Vec<ValueRange> = Vec::new();
        
        for mapped_range in &self.mapped_ranges {
            // Mapped ranges should be ordered by source_start, so if this is true then the values should be mapped 1 to 1.
            if temp_range.stop() < mapped_range.source_start {
                mapped_values.push(temp_range);
                break;
            }

            let start_in_range = temp_range.start >= mapped_range.source_start && temp_range.start <= mapped_range.source_stop();
            let stop_in_range = temp_range.stop() >= mapped_range.source_start && temp_range.stop() <= mapped_range.source_stop();

            // Start of given range start falls within mapped range.
            if start_in_range {
                // Stop of given range falls within mapped range aswell.
                if stop_in_range {
                    mapped_values.push(
                        ValueRange { 
                            start: mapped_range.get_mapped_value(temp_range.start),
                            length: temp_range.length
                        }
                    );
                    break;
                // Stop of given range falls outside of mapped range.
                } else {
                    mapped_values.push( 
                        ValueRange {
                            start: mapped_range.get_mapped_value(temp_range.start), 
                            length: mapped_range.source_start - temp_range.start 
                        }
                    );
                    // Create new range with values that were outside of the current mapped range but might fall within the next range.
                    temp_range = ValueRange{
                        start: mapped_range.source_stop() + 1,
                        length: (mapped_range.source_stop() + 1) - temp_range.start}
                }
            } else if stop_in_range {
                let unmapped_length = mapped_range.source_start - temp_range.start;
                // Push unmapped part of the range.
                mapped_values.push(
                    ValueRange{
                        start: temp_range.start,
                        length: unmapped_length
                    }
                );
                // Push mapped part of the range.
                mapped_values.push(
                    ValueRange{
                        start: mapped_range.dest_start,
                        length: temp_range.length - unmapped_length
                    }
                );
                break;
            } else if temp_range.start < mapped_range.source_start && temp_range.stop() > mapped_range.source_stop() {
                panic!(":/")
            }
        }
        if mapped_values.is_empty() {
            mapped_values.push(value_range)
        }
        mapped_values
    }

    fn get_from_many_ranges(&self, ranges: Vec<ValueRange>) -> Vec<ValueRange> {
        let mut results = Vec::new();
        for r in ranges {
            results.extend(self.get_from_range(r))
        }
        results
    }

}

fn parse_seed_line(seed_line: &str) -> Vec<i128> {
    let (_, nums_str) = seed_line.split_once(":").unwrap();
    match nums_str.split_whitespace().map(|n| n.parse::<i128>()).collect() {
        Ok(nums) => nums,
        Err(error) => panic!("Error parsing seed line numbers: {error:?}")
    }
}

fn parse_map_header_line (line: &str) -> ElfMap {
    let (source, dest) = line.strip_suffix(" map:").unwrap().split_once("-to-").unwrap();
    ElfMap::new(source, dest)
}

fn parse_map_content_line (line: &str) -> (i128, i128, i128) {
    let numbers: Vec<i128> = match line.split_whitespace().map(|n| n.parse::<i128>()).collect() {
        Ok(nums) => nums,
        Err(error) => panic!("Error parsing seed line numbers: {error:?}")
    };
    (numbers[0], numbers[1], numbers[2])

}

fn parse_almanac (mut lines: Lines) -> HashMap<String, ElfMap> {
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
            // for i in 0..range_len {
                // cur_map.map.insert(source_range_start + i, dest_range_start + i);
            // }
            cur_map.mapped_ranges.push(MappedRange { 
                source_start: source_range_start, 
                dest_start: dest_range_start, 
                length: range_len 
            })
        }
    }
    almanac.insert(cur_map.source.to_owned(), cur_map);
    almanac
}



fn main() {
    // let args: Vec<String> = env::args().collect();
    // let file_name = &args[1];
    // let file_name = "sampledata.txt";
    let file_name = "day5_input.txt";

    let file_content = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error)
    };

    let mut lines = file_content.lines();
    
    let seed_line = lines.next().unwrap();
    let seed_numbers = parse_seed_line(seed_line);
    let almanac = parse_almanac(lines);

    let seed_map = almanac.get("seed").unwrap();
    let soil_map = almanac.get("soil").unwrap();
    let fertilizer_map = almanac.get("fertilizer").unwrap();
    let water_map = almanac.get("water").unwrap();
    let light_map = almanac.get("light").unwrap();
    let temperature_map = almanac.get("temperature").unwrap();
    let humidity_map = almanac.get("humidity").unwrap();

    // Code for part 1.
    let mut lowest_val_part_1: Option<i128> = None;
    for num in seed_numbers.clone() {        
        // Hardcoded for sanity.
        let soil = seed_map.get(num);
        let fert = soil_map.get(soil);
        let water = fertilizer_map.get(fert);
        let light = water_map.get(water);
        let temp = light_map.get(light);
        let humid = temperature_map.get(temp);
        let location = humidity_map.get(humid);

        let cur_val = location;
        match lowest_val_part_1 {
            None => lowest_val_part_1 = Some(cur_val),
            Some(v) => {
                if cur_val < v {
                    lowest_val_part_1 = Some(cur_val)
                }
            }
        }
    }
    println!("Lowest location number for initial seeds (Part 1): {}", lowest_val_part_1.unwrap());

    // Code for part 2;
    let mut part_2_numbers: Vec<ValueRange> = Vec::new();
    for i in (0..seed_numbers.len()).step_by(2) {
        let start = seed_numbers[i];
        let len = seed_numbers[i+1];
        part_2_numbers.push(ValueRange { start: start, length: len })
    }
    // part_2_numbers = vec![ValueRange{start: 45, length: 10}];
    for value_range in part_2_numbers {
        let soil_ranges = seed_map.get_from_range(value_range);
        let fert_ranges = soil_map.get_from_many_ranges(soil_ranges);
        let water_ranges = fertilizer_map.get_from_many_ranges(fert_ranges);
        let light_ranges = water_map.get_from_many_ranges(water_ranges);
        let temp_ranges = light_map.get_from_many_ranges(light_ranges);
        let humid_ranges = temperature_map.get_from_many_ranges(temp_ranges);
        let location_ranges = humidity_map.get_from_many_ranges(humid_ranges);
        println!("{:?}", location_ranges)
    }
}
