use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::Lines;

#[derive(Clone)]
struct MappedRange {
    source_start: i128,
    dest_start: i128,
    length: i128
}

impl MappedRange {
    fn num_in_source_range(&self, s: i128) -> bool {
        s >= self.source_start && s < self.source_start + self.length
    }

    fn get_mapped_value(&self, s: i128) -> i128 {
        let diff = self.dest_start - self.source_start;
        s + diff
    }
}

struct ElfMap {
    source: String,
    dest: String,
    // map: HashMap<u32, u32>
    mapped_ranges: Vec<MappedRange>
}

impl ElfMap {
    fn new(source: &str, dest: &str) -> Self {
        Self {
            source: source.to_owned(),
            dest: dest.to_owned(),
            mapped_ranges: Vec::new()
            // map: HashMap::new()
        }
    }

    fn get(&self, s: i128) -> i128 {
        // match self.map.get(&s) {
        //     Some(v) => v.to_owned(),
        //     None => s,
        // }

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

    // Code for part 2;
    let mut part_2_numbers: Vec<i128> = Vec::new();
    for i in (0..seed_numbers.len()).step_by(2) {
        let start = seed_numbers[i];
        let len = seed_numbers[i+1];
        part_2_numbers.extend(start..start+len)
    }
    let seed_numbers = part_2_numbers;

    let almanac = parse_almanac(lines);

    let mut lowest_val: Option<i128> = None;

    for num in seed_numbers {
        // let mut cur_map = match almanac.get("seed") {
        //     Some(v) => v,
        //     None => panic!("Unable to locate 'seed' in almanac for number {num}")
        // };
        // // let mut cur_val = cur_map.get(num);
        // while true {
        //     cur_map = match almanac.get(&cur_map.dest){
        //         Some(v) => v,
        //         None => panic!("Unable to locate '{}' in almanac for number {num}", cur_map.dest)
        //     };
        //     cur_val = cur_map.get(cur_val);
        //     if cur_map.dest == "location" {
        //         break;
        //     }
        // }
        
        // Hardcoded for sanity.
        let soil = almanac.get("seed").unwrap().get(num);
        let fert = almanac.get("soil").unwrap().get(soil);
        let water = almanac.get("fertilizer").unwrap().get(fert);
        let light = almanac.get("water").unwrap().get(water);
        let temp = almanac.get("light").unwrap().get(light);
        let humid = almanac.get("temperature").unwrap().get(temp);
        let location = almanac.get("humidity").unwrap().get(humid);

        let cur_val = location;
        match lowest_val {
            None => lowest_val = Some(cur_val),
            Some(v) => {
                if cur_val < v {
                    lowest_val = Some(cur_val)
                }
            }
        }

    }

    
    println!("Lowest location number for initial seeds: {}", lowest_val.unwrap())
}
