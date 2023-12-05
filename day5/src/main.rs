use core::num;
use std::collections::HashMap;
use std::collections::hash_map::OccupiedEntry;
use std::env;
use std::fs;
use std::cmp;

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
        self.map.entry(s)
    }
}

fn parse_seed_line(seed_line: &str) -> Vec<u32> {
    let (_, nums_str) = seed_line.split_once(":").unwrap();
    match nums_str.split_whitespace().map(|n| n.parse::<u32>()).collect() {
        Ok(nums) => nums,
        Err(error) => panic!("Error parsing seed line numbers: {error:?}")
    }
}

fn parse_map_content_line (line: &str) -> (u32, u32, u32) {
    let numbers: Vec<u32> = match line.split_whitespace().map(|n| n.parse::<u32>()).collect() {
        Ok(nums) => nums,
        Err(error) => panic!("Error parsing seed line numbers: {error:?}")
    };
    (numbers[0], numbers[1], numbers[2])

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_name = "sampledata.txt";

    let file_content = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error)
    };

    let mut lines = file_content.lines();
    
    let seed_line = lines.next().unwrap();
    let seed_numbers = parse_seed_line(seed_line);
    
    for line in lines {
        let almanac: HashMap<String, ElfMap> = HashMap::new();
        let mut cur_map: ElfMap;

        let c: Vec<char> = line.chars().collect();
        if c[0].is_alphabetic() {
            let (source, dest) = line.strip_suffix(" map:").unwrap().split_once("-to-").unwrap();
            cur_map = ElfMap::new(source, dest);
        } else if  c[0].is_numeric() {
            let (dest_range_start, source_range_start, range_len) = parse_map_content_line(line);
        }
    }
}
