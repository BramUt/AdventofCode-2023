use std::env;
use std::fs;
use std::time::Instant;

mod structs;
use crate::structs::{ValueRange, MappedRange};

mod parsers;
use crate::parsers::{parse_seed_line, parse_almanac};


struct ElfMap {
    source: String,
    mapped_ranges: Vec<MappedRange>
}

impl ElfMap {
    fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            mapped_ranges: Vec::new()
        }
    }

    fn get(&self, s: i128) -> i128 {

        let mut range_with_value: Option<MappedRange> = None;
        for range in &self.mapped_ranges {
            if range.num_in_source_range(s) {
                range_with_value = Some(range.clone());
                break;
            }
        }

        match range_with_value {
            Some(r) => r.get_mapped_value(s),
            None => s
        }
    }

    fn get_from_range(&self, value_range: ValueRange) -> Vec<ValueRange> {
        let mut temp_range_option = Some(value_range.clone());

        let mut mapped_values: Vec<ValueRange> = Vec::new();
        
        for mapped_range in &self.mapped_ranges {
            if temp_range_option.is_none() {
                break;
            }
            let temp_range = temp_range_option.clone().expect("Loop should have stopped.");

            // Mapped ranges should be ordered by source_start, so if this is true then the values should be mapped 1 to 1.
            if temp_range.stop() < mapped_range.source_start {
                mapped_values.push(temp_range);
                temp_range_option = None;
                break;
            }

            let start_in_range = temp_range.start >= mapped_range.source_start && temp_range.start <= mapped_range.source_stop();
            let stop_in_range = temp_range.stop() >= mapped_range.source_start && temp_range.stop() <= mapped_range.source_stop();

            if start_in_range { 
                if stop_in_range { // Full overlap.
                    mapped_values.push(
                        ValueRange { 
                            start: mapped_range.get_mapped_value(temp_range.start),
                            length: temp_range.length
                        }
                    );
                    temp_range_option = None;
                    break;
                } else { // Tail to head overlap.
                    let mapped_length = mapped_range.source_stop() - temp_range.start + 1;
                    // Create range with value that fall within this mapped range.
                    mapped_values.push(
                        ValueRange {
                            start: mapped_range.get_mapped_value(temp_range.start), 
                            length: mapped_length
                        }
                    );
                    // Create new range with values that were outside of the current mapped range but might fall within the next range.
                    temp_range_option = Some(ValueRange{
                        start: mapped_range.source_stop() + 1,
                        length: temp_range.length - mapped_length
                    });
                }
            } else if stop_in_range { // Head to tail overlap.
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
                temp_range_option = None;
                break;
            } else if temp_range.start < mapped_range.source_start && temp_range.stop() > mapped_range.source_stop() {
                // Given value range fully eclipses the mapped range.
                let unmapped_length_left = mapped_range.source_start - temp_range.start;
                let mapped_length = mapped_range.length;
                let unmapped_length_right = temp_range.stop() - mapped_range.source_stop();
                // Push unmapped part of the range.
                mapped_values.push(
                    ValueRange{
                        start: temp_range.start,
                        length: unmapped_length_left
                    }
                );
                // Push mapped part of the range.
                mapped_values.push(
                    ValueRange{
                        start: mapped_range.dest_start,
                        length: mapped_length
                    }
                );
                // Create new range with values that were outside of the current mapped range but might fall within the next range.
                temp_range_option = Some(ValueRange{
                    start: mapped_range.source_stop() + 1,
                    length: unmapped_length_right
                });
            }
        }
        if mapped_values.is_empty() {
            mapped_values.push(value_range)
        } else if temp_range_option.is_some() {
            mapped_values.push(temp_range_option.unwrap())
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

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

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
        println!("Seed: {num}");
        let soil = seed_map.get(num);
        println!("\tSoil: {soil}");
        let fert = soil_map.get(soil);
        println!("\tFert: {fert}");
        let water = fertilizer_map.get(fert);
        println!("\tWater: {water}");
        let light = water_map.get(water);
        println!("\tLight: {light}");
        let temp = light_map.get(light);
        println!("\tTemp: {temp}");
        let humid = temperature_map.get(temp);
        println!("\tHumidity: {humid}");
        let location = humidity_map.get(humid);
        println!("\tLocation: {location}");

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

    let mut lowest_val_part_2: Option<i128> = None;
    // part_2_numbers = vec![ValueRange{start: 1198254212, length: 504239157}]; // TODO remove
    for value_range in part_2_numbers {
        println!("\n\nSeed range: {:?}", value_range);
        let soil_ranges = seed_map.get_from_range(value_range);
        println!("\tSoil ranges: {:?}", soil_ranges);
        let fert_ranges = soil_map.get_from_many_ranges(soil_ranges);
        println!("\tFert ranges: {:?}", fert_ranges);
        let water_ranges = fertilizer_map.get_from_many_ranges(fert_ranges);
        println!("\tWater ranges: {:?}", water_ranges);
        let light_ranges = water_map.get_from_many_ranges(water_ranges);
        println!("\tLight ranges: {:?}", light_ranges);
        let temp_ranges = light_map.get_from_many_ranges(light_ranges);
        println!("\tTemp ranges: {:?}", temp_ranges);
        let humid_ranges = temperature_map.get_from_many_ranges(temp_ranges);
        println!("\tHumid ranges: {:?}", humid_ranges);
        let location_ranges = humidity_map.get_from_many_ranges(humid_ranges);
        println!("\tLocation ranges: {:?}", location_ranges);
        for r in location_ranges {
            match lowest_val_part_2 {
                None => lowest_val_part_2 = Some(r.start),
                Some(v) => {
                    if r.start < v{
                        lowest_val_part_2 = Some(r.start)
                    }
                }
            }
        }
    }
    println!("Lowest location number for initial seeds (Part 2): {}", lowest_val_part_2.unwrap());
    println!("Execution time: {:?}", start.elapsed())
}
