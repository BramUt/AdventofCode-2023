use core::num;
use std::collections::HashMap;
use std::collections::hash_map::OccupiedEntry;
use std::env;
use std::fs;
use std::cmp;
use std::str::Lines;

struct ElfMap {
    source: String,
    dest: String,
    map: HashMap<u32, u32>
}

impl ElfMap {
    fn new(source: &str, dest: &str) -> Self {
        Self {
            source: source.to_owned(),
            dest: dest.to_owned(),
            map: HashMap::new()
        }
    }

    fn get(&self, s: u32) -> u32 {
        match self.map.get(&s) {
            Some(v) => v.to_owned(),
            None => s,
        }
    }
}

fn parse_seed_line(seed_line: &str) -> Vec<u32> {
    let (_, nums_str) = seed_line.split_once(":").unwrap();
    match nums_str.split_whitespace().map(|n| n.parse::<u32>()).collect() {
        Ok(nums) => nums,
        Err(error) => panic!("Error parsing seed line numbers: {error:?}")
    }
}

fn parse_map_header_line (line: &str) -> ElfMap {
    let (source, dest) = line.strip_suffix(" map:").unwrap().split_once("-to-").unwrap();
    ElfMap::new(source, dest)
}

fn parse_map_content_line (line: &str) -> (u32, u32, u32) {
    let numbers: Vec<u32> = match line.split_whitespace().map(|n| n.parse::<u32>()).collect() {
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
            for i in 0..range_len {
                cur_map.map.insert(source_range_start + i, dest_range_start + i);
            }
        }
    }
    almanac.insert(cur_map.source.to_owned(), cur_map);
    almanac
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    // let file_name = "sampledata.txt";

    let file_content = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error)
    };

    let mut lines = file_content.lines();
    
    let seed_line = lines.next().unwrap();
    let seed_numbers = parse_seed_line(seed_line);

    let almanac = parse_almanac(lines);

    let mut lowest_val: Option<u32> = None;

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
