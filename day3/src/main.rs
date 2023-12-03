use std::collections::HashMap;
use std::env;
use std::fs;
use std::cmp;

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
struct Gear {
    col: i32,
    row: i32
}

#[derive(Debug)]
#[derive(Clone)]
struct PartNum {
    start_col: i32,
    stop_col: i32,
    row: i32,
    num_str: String
}

impl PartNum {
    fn check_next_to_symbol(&self, engine: &Vec<Vec<char>>) -> (bool, Vec<Gear>) {
        let min_row = cmp::max(self.row-1, 0) as usize;
        let max_row = cmp::min(self.row+1, engine.len() as i32 -1) as usize;
        
        let min_col = cmp::max(self.start_col-1, 0) as usize;
        let max_col = cmp::min(self.stop_col+1, engine[0].len() as i32) as usize;

        // println!("{:?}", vec!(min_row..max_row+1));
        let mut next_to_symbol = false;
        let mut neighbouring_stars: Vec<Gear> = Vec::new();

        for cur_row in min_row..max_row+1 {
            print!("\n");
            for cur_col in min_col..max_col {
                print!("{:?}", engine[cur_row][cur_col]);
                let cur_char = engine[cur_row][cur_col];
                if !(cur_char.is_numeric() || cur_char == '.') {
                    next_to_symbol = true;
                }
                
                if cur_char == '*' {
                    neighbouring_stars.push(Gear { col: cur_col as i32, row: cur_row as i32});
                }
            }
        }
        (next_to_symbol, neighbouring_stars)
    }

    fn convert_num_str (&self) -> i32 {
        match self.num_str.parse() {
            Ok(number) => number,
            Err(error) => panic!("Error parsing num_str {:?}. Error: {:?}", &self.num_str, error)
        }
    }
}

fn parse_file_content(file_content: String) -> (Vec<PartNum>, Vec<Vec<char>>) {
    let mut engine: Vec<Vec<char>> = Vec::new();
    let mut parts: Vec<PartNum> = Vec::new();

    for (line_num, line) in file_content.lines().enumerate() {
        engine.push(line.chars().collect());

        let mut current_part_num = String::new();
        let mut part_num_start: usize = 0;
        let mut part_num_stop = 0;

        for (col, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if current_part_num.is_empty() {
                    part_num_start = col;
                }
                current_part_num.push(c);
            
            } else if !(current_part_num.is_empty()) {
                part_num_stop = col;
                parts.push(
                    PartNum { 
                        start_col: part_num_start as i32, 
                        stop_col: part_num_stop as i32, 
                        row: line_num as i32,
                         num_str: current_part_num
                    }
                );
                current_part_num = String::new();
            }
        }
        if !(current_part_num.is_empty()) {
            part_num_stop = line.len();
            parts.push(
                PartNum { 
                    start_col: part_num_start as i32, 
                    stop_col: part_num_stop as i32, 
                    row: line_num as i32,
                     num_str: current_part_num
                }
            );
        }
    }
    (parts, engine)
}

fn main() {
    let collect = env::args().collect();
    let args: Vec<String> = collect;
    let file_name = &args[1];
    // let file_name = "testdata.txt";
    // let file_name = "day3_input.txt";
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    let (parts, engine) = parse_file_content(file_content);
    // println!("{engine:?}");
    let mut part_num_sum = 0;
    let mut gears_with_neighbours: HashMap<Gear, Vec<PartNum>> = HashMap::new();

    for part in parts {
        println!("\n\n{part:?}");
        let (is_viable, neighbouring_gears) = part.check_next_to_symbol(&engine);
        println!("\nShould count towards sum: {is_viable}");
        println!("Neighbouring gears: {:?}", neighbouring_gears);

        if is_viable {
            part_num_sum += part.convert_num_str();
        }

        for gear in neighbouring_gears {
            let gear_entry = gears_with_neighbours.entry(gear).or_insert(Vec::new());
            gear_entry.push(part.clone())
        }

    }
    println!("Sum of part numbers: {part_num_sum}");

    let mut sum_of_gear_ratios = 0;
    for (gear, parts) in gears_with_neighbours.iter() {
        if parts.len() == 2 {
            sum_of_gear_ratios += parts[0].convert_num_str() * parts[1].convert_num_str()
        }
    }
    println!("Sum of gear ratios: {sum_of_gear_ratios}")
}


