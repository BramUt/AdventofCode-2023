use std::fs;
use std::env;
use aho_corasick::{AhoCorasick};

fn get_num(num_str: &str) -> i32 {
    match num_str {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Something went wrong: {}", num_str)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_content = fs::read_to_string(file_name).unwrap();

    // let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let patterns = vec!["1", "2", "3", "4", "5", "6", "7", "8", "8", "9","one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let ac = AhoCorasick::new(patterns.clone()).unwrap();

    let mut numbers: Vec<u64> = Vec::new();
    for line in file_content.lines() {

        let matches: Vec<usize> = ac.find_overlapping_iter(line).map(|mat| mat.pattern().as_usize()).collect();
        println!("{} {:?}", line, matches);

        let n1 = get_num(patterns[matches[0]]);
        let n2 = get_num(patterns[matches.last().unwrap().to_owned()]);
        println!("{} {}", n1, n2);
        let number = format!("{}{}", n1, n2);
        println!("{}", number);
        numbers.push(number.parse().unwrap());
    
    }
    println!("Final number: {}", numbers.iter().sum::<u64>())
}
