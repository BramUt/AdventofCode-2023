use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_content = fs::read_to_string(file_name).unwrap();

    let mut numbers: Vec<i32> = Vec::new();
    // println!("{}", file_content);
    for line in file_content.lines() {
        let mut n1: Option<char> = None;
        let mut n2: Option<char> = None;
        // Forward loop for first digit.
        for c in line.chars() {
            if c.is_numeric(){
                n1 = Some(c);
                break;
            }
        }
        // Reverse loop for the last digit.
        for c in line.chars().rev() {
            if c.is_numeric(){
                n2 = Some(c);
                break
            }
        }
        if n1.is_some() && n2.is_some() {
            let number = format!("{}{}", n1.unwrap(), n2.unwrap());
            // let t_int: i32 = number.parse().unwrap();
            // numbers.push(t_int);
            // println!("{} {} {} {}", n1.unwrap(), n2.unwrap(), t_int, line)
            numbers.push(number.parse().unwrap());
        } else {
            println!("Something went wrong :/");
            break
        }
    }
    println!("Final number: {}", numbers.iter().sum::<i32>())
}
