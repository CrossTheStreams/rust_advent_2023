use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Part 1

pub fn run_day_1_part_1() -> () {
    let file = File::open("inputs/day1.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let calibration_values: Vec<u32> = lines.iter().map(|l| get_calibration_value_part1(l.to_string())).collect();
    let answer: u32 = calibration_values.iter().sum();
    println!("Part 1: This sum of the calibration values: {}", answer)
}

fn get_calibration_value_part1(line: String) -> u32 {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;
    for char in line.chars() {
        if char.is_digit(10) {
            first_digit = Some(char);
            break
        }    
    }
    for char in line.chars().rev() {
        if char.is_digit(10) {
            last_digit = Some(char);
            break
        }    
    }
    let digit = [
        first_digit.unwrap_or('a').to_string(),
        last_digit.unwrap_or('a').to_string()
    ].join(""); 
    match digit.parse() {
        Ok(num) => num,
        Err(_) => 0
    }
}

#[test]
fn test_get_calibration_value_part1() {
    // two digits in the string
    let num1 = get_calibration_value_part1("abc3edf5xzy".to_string());
    assert_eq!(35, num1);

    // More than two digits in the string
    let num2 = get_calibration_value_part1("1abc3edf5xzy2".to_string());    
    assert_eq!(12, num2);

    // value is zero when there isn't a digit present
    let num3 = get_calibration_value_part1("abcdef".to_string());    
    assert_eq!(0, num3);

    // single digit in the string counts for both the first and last digit
    let num4 = get_calibration_value_part1("abc5def".to_string());    
    assert_eq!(55, num4);

    let line = "ncbfctqlsnfive1brqpthree4".to_string();
    let num5 = get_calibration_value_part1(line.to_string());
    assert_eq!(14, num5)
}

// Part 2

pub fn run_day_1_part_2() -> () {
    let file = File::open("inputs/day1.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let calibration_values: Vec<u32> = lines.iter().map(|l| get_calibration_value_part2(l.to_string())).collect();
    let answer: u32 = calibration_values.iter().sum();
    println!("Part 2: This *correct* sum of the calibration values: {}", answer)
}

fn get_calibration_value_part2(line: String) -> u32 {
    let digits = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    // A mapping of the wordy digit strings to their char counterparts
    // Similar to part 1, except we catch these words as digits also
    // we concat the digit chars at the end just like part 1, then parse that as the u32, then
    // return that value.
    let mut chars_from_front: String = String::from("");
    let mut chars_from_back: String = String::from("");
    let mut first_char: Option<char> = None;
    let mut last_char: Option<char> = None;
    for char in line.chars() {
        if first_char.is_some() {
            break
        }
        if char.is_digit(10) {
            first_char = Some(char)
        }
        chars_from_front += &char.to_string();
        for (digit_word, digit_char) in &digits {
            if chars_from_front.contains(digit_word) {
                first_char = Some(*digit_char);
                break
            }
        }
    }
    for char in line.chars().rev() {
        if last_char.is_some() {
            break
        }
        if char.is_digit(10) {
            last_char = Some(char);
            break
        }
        // We need to arrange the wordy digits in the correct direction
        // e.g. "nine" and not "enin"
        chars_from_back = [char.to_string(), chars_from_back].join("");
        for (digit_word, digit_char) in &digits {
            if chars_from_back.contains(digit_word) {
                last_char = Some(*digit_char);
                break
            }
        }
    }
    let digit = [
        first_char.unwrap_or('a').to_string(),
        last_char.unwrap_or('a').to_string()
    ].join(""); 
    match digit.parse() {
        Ok(num) => num,
        Err(_) => 0
    }
}

#[test]
fn test_get_calibration_value_part2() {
    assert_eq!(29, get_calibration_value_part2("two1nine".to_string()));
    assert_eq!(83, get_calibration_value_part2("eightwothree".to_string()));
    assert_eq!(13, get_calibration_value_part2("abcone2threexyz".to_string()));
    assert_eq!(24, get_calibration_value_part2("xtwone3four".to_string()));
    assert_eq!(42, get_calibration_value_part2("4nineeightseven2".to_string()));
    assert_eq!(14, get_calibration_value_part2("zoneight234".to_string()));
    assert_eq!(76, get_calibration_value_part2("7pqrstsixteen".to_string()));
}