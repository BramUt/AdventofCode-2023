use std::iter::zip;
use std::{env, fs};
use std::time::Instant;

/// Parse the next line :).
fn parse_line(l: &str) -> Vec<i64> {
    l.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

fn parse_line_part_2(l: &str) -> Vec<i64> {
    let s = l.split_whitespace();
    let str = String::from_iter(s);
    let i = str.parse::<i64>().unwrap();
    vec![i]
}

fn main() {
    // let start = Instant::now();
    // let args: Vec<String> = env::args().collect();
    // let file_name = &args[1];
    let file_name = "day6_input.txt";

    let file_content = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error)
    };
    let mut lines = file_content.lines();

    let times: Vec<i64> = parse_line_part_2(lines.next().unwrap().strip_prefix("Time:").unwrap());
    let distances: Vec<i64> = parse_line_part_2(lines.next().unwrap().strip_prefix("Distance:").unwrap());
    let values: Vec<(i64, i64)> = zip(times, distances).collect();

    println!("{values:?}");
    let mut outcome: Option<i64> = None;


    for (time, distance) in values {
        let d = (time.pow(2) + (-4 * distance)) as f32;
        let k1= (time as f32 - f32::sqrt(d)) / 2 as f32;
        let k2= (time as f32 + f32::sqrt(d)) / 2 as f32;
        let possibilities = k2.ceil() as i64 - k1.floor() as i64 - 1;
        println!("Time: {time}, Distance: {distance}, d: {d}: k1: {k1}, k2: {k2}, Possibilities: {possibilities}");
        outcome = Some(match outcome {
            Some(v) => v * possibilities,
            None => possibilities,
        });
        println!("Outcome: {}", outcome.unwrap())
    }
}
