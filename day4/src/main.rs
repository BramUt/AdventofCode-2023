use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    // let file_name = "sampledata.txt";  // Hardcoded for debugging.
    let file_content = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error)
    };

    let mut total_points = 0;
    let mut card_multipliers: HashMap<i32, u32> = HashMap::new();
    let mut total_cards = 0;

    for line in file_content.lines() {
        let (card_str, all_nums) = line.split_once(": ").unwrap();
        let (winning_nums_str, elf_nums_str) = all_nums.split_once(" | ").unwrap();
        let winning_nums: Vec<i32> = match winning_nums_str.split_whitespace().map(|n| n.parse::<i32>()).collect() {
            Ok(r) => r,
            Err(error) => panic!("Error parsing winning numbers for {card_str}: {error:?}")
        };
        let elf_nums: Vec<i32> = match elf_nums_str.split_whitespace().map(|n| n.parse::<i32>()).collect() {
            Ok(r) => r,
            Err(error) => panic!("Error parsing elf numbers for {card_str}: {error:?}")
        };
        let elf_nums_set: HashSet<i32> = HashSet::from_iter(elf_nums.iter().cloned());
        let winning_nums_set: HashSet<i32> = HashSet::from_iter(winning_nums.iter().cloned());

        let intersect = elf_nums_set.intersection(&winning_nums_set).collect::<Vec<_>>();

        // Calculate card points for part 1.
        let mut card_points: i32 = 0;
        if !intersect.is_empty() {
            let l = intersect.len();
            card_points = 2i32.pow(l as u32 - 1);
            total_points += card_points
        }
        println!("{card_str}: {card_points} points, {intersect:?}");

        // Code for part 2.
        let card_num = card_str.split_whitespace().last().unwrap().parse::<u32>().unwrap();
        let current_multiplier = card_multipliers.entry(card_num as i32).or_insert(1).to_owned();

        // Add multiplier for current card.
        total_cards += current_multiplier;


        // Update multipliers for found cards.
        let found_cards_start = card_num+ 1;
        let found_cards_stop = card_num + intersect.len() as u32 + 1;
        for found_card in found_cards_start..found_cards_stop {
            let found_card_multiplier = card_multipliers.entry(found_card as i32).or_insert(1);
            *found_card_multiplier += 1 * current_multiplier
        }
        
    }
    println!("Total points for part 1: {total_points}");
    println!("Total cards for part 2: {total_cards}");
}
