use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

// Part 1

pub fn run_day_4_part_1() -> () {
    let file: File = File::open("inputs/day4.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let mut total_points = 0;
    for line in lines {
        total_points += count_points(line);
    }
    println!("Part 1: The sum of card points in inputs/day4.txt ==> {}", total_points)
}

fn count_points(line: String) -> u32 { 
    let card = Card::from_string(line);
    let matching_nums_count = card.matching_nums.len() as u32;
    // If there are no winning nums, that counts for zero points
    if matching_nums_count < 1 {
        return 0
    }
    // "The first match makes the card worth one point and each match after the first doubles the point value of that card.""
    // Which works out to 2 ** (winning nums count - 1)
    2_u32.pow(matching_nums_count - 1)
}

#[test]
fn test_count_points() {
    let card1 = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    assert_eq!(8, count_points(card1));
    let card2 = String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
    assert_eq!(2, count_points(card2));
}

// Part 2

#[derive(Debug)]
struct Card {
    number: u32,
    winning_nums: Vec<u32>,
    scratched_nums: Vec<u32>,
    matching_nums: Vec<u32>
}

impl Card {
    fn from_string(string: String) -> Self {
        let re = Regex::new(r"(\d+)").unwrap(); 
        let mut string_parts = string.split("|");
        let winning_part = string_parts.next().unwrap();
        let my_part = string_parts.next().unwrap();
    
        let mut winning_nums: Vec<u32> = re.find_iter(&winning_part).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
        let card_num = winning_nums.swap_remove(0); // Remove the first digit, which is the Card's number 
        let scratched_nums: Vec<u32> = re.find_iter(&my_part).map(|m| m.as_str().parse::<u32>().unwrap()).collect();

        let winning_nums_set: HashSet<u32> = winning_nums.clone().into_iter().collect();
        // Find the intersection of scratched_nums with winning_nums
        let matching_nums = scratched_nums.iter().filter(|num| winning_nums_set.contains(&num)).cloned().collect(); 
        return Self {
            number: card_num,
            winning_nums,
            scratched_nums,
            matching_nums
        }
    }
}

pub fn run_day_4_part_2() -> () {
    let file: File = File::open("inputs/day4.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let cards: Vec<Card> = reader.lines().map(|l| Card::from_string(l.expect("Failed to read line"))).collect();
    let total_scratchcards = count_of_scratchcards_won(cards);
    println!("Part 2: Total scratchcards won in inputs/day4.txt ==> {}", total_scratchcards);
}

fn count_of_scratchcards_won(stack: Vec<Card>) -> u32 {
    let mut card_counts = HashMap::<u32, u32>::new();
    let mut total_card_count = 0;
    // Populate the card_hash with cards from the initial stack
    for card in &stack {
        card_counts.insert(card.number, 1);
    }
    // I think this could be further improved with some concurrency stuff...
    for drawn_card in stack {
        // draw the card n times, for 1 (the card in the original deck) + (every time we've added the card from previous matches) 
        for (idx, _matched_num) in drawn_card.matching_nums.iter().enumerate() { 
            let card_to_add_num = drawn_card.number + (idx as u32 + 1);
            if let Some(card_count) = card_counts.get(&card_to_add_num) {
                let drawn_times = card_counts.get(&drawn_card.number).unwrap();
                card_counts.insert(card_to_add_num, card_count + drawn_times);
            }
        }
    }
    for (_card_num, count) in card_counts {
        total_card_count += count;
    }
    return total_card_count 
}

#[test]
fn test_count_of_scratchcards_won() {
    let lines = vec![
        String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
        String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
        String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
        String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
        String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
        String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
    ];
    let cards = lines.iter().map(|l| Card::from_string(l.to_string())).collect();
    assert_eq!(30, count_of_scratchcards_won(cards));
}
