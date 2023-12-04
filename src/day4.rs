use std::collections::HashSet;
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
    let re = Regex::new(r"(\d+)").unwrap(); 
    let mut line_parts = line.split("|");
    let winning_part = line_parts.next().unwrap();
    let my_part = line_parts.next().unwrap();

    let mut winning_nums: Vec<u32> = re.find_iter(&winning_part).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
    let _card_num = winning_nums.swap_remove(0); // Remove the first digit, which is the Card's number

    let my_nums: Vec<u32> = re.find_iter(&my_part).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
    let winning_nums_set: HashSet<u32> = winning_nums.into_iter().collect();
    let my_winning_nums: Vec<u32> = my_nums.into_iter().filter(|num| winning_nums_set.contains(num)).collect();
    let my_winning_nums_count: u32 = my_winning_nums.len() as u32;
    if my_winning_nums_count < 1 {
        return 0
    }
    2_u32.pow(my_winning_nums_count - 1)
}

#[test]
fn test_count_points() {
    let card1 = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    assert_eq!(8, count_points(card1));
    let card2 = String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
    assert_eq!(2, count_points(card2));
}