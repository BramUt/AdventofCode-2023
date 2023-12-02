use std::env;
use std::fs;
use std::cmp;

#[derive(Debug)]
struct Grab {
    r: u32,
    g: u32,
    b: u32,
}

fn parse_grab_strs(full_grab_str: &str) -> Vec<Grab> {
    let mut grabs: Vec<Grab> = Vec::new();
    for single_grab_str in full_grab_str.split(";") {
        for part in single_grab_str.split(",") {
            let (mut r, mut g, mut b) = (0, 0, 0);
            // println!("{part}");
            let (num, col) = part.trim().split_once(" ").unwrap();
            // println!("{num}:{col}");
            match col {
                "red" => r = num.parse().unwrap(),
                "green" => g = num.parse().unwrap(),
                "blue" => b = num.parse().unwrap(),
                _ => panic!("Invalid color string: {}", col)
            }
            grabs.push(Grab{r, g, b});
        }
    }
    grabs
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_content = fs::read_to_string(file_name).unwrap();

    for line in file_content.lines(){
        let (prefix, suffix) = line.split_once(":").unwrap();
        let game_num: i32 = prefix.split_whitespace().last().unwrap().parse().unwrap();
        let grabs = parse_grab_strs(suffix);

        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
        for grab in grabs {
            max_r = cmp::max(grab.r, max_r);
            max_g = cmp::max(grab.g, max_g);
            max_b = cmp::max(grab.r, max_b);
        }

        println!("{game_num} red:{max_r}, green:{max_g}, blue:{max_b}");

    }
}
