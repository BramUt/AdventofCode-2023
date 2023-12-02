use std::env;
use std::fs;
use std::cmp;

#[derive(Debug)]
struct Grab {
    r: i32,
    g: i32,
    b: i32,
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

    let mut viable_games: Vec<i32> = Vec::new();

    for line in file_content.lines(){
        let (prefix, suffix) = line.split_once(":").unwrap();
        let game_num: i32 = prefix.split_whitespace().last().unwrap().parse().unwrap();
        let grabs = parse_grab_strs(suffix);

        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);

        for grab in grabs {
            max_r = cmp::max(grab.r, max_r);
            max_g = cmp::max(grab.g, max_g);
            max_b = cmp::max(grab.b, max_b);
        }

        println!("Game {game_num}: max red:{max_r}, max green:{max_g}, max blue:{max_b}");

        // Code needed for part 1:
        // if !(max_r > 12 || max_g > 13 || max_b > 14) {
        //     viable_games.push(game_num)
        // }

        // Code needed for part 2:
        viable_games.push(max_r*max_g*max_b)
    }
    println!("Viable games: {viable_games:?}");
    println!("Sum of viable games: {}", viable_games.iter().sum::<i32>());
}
