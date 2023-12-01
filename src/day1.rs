use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_day_1_part_1() -> () {
    let file = File::open("inputs/day1.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let calibration_values: Vec<u32> = lines.iter().map(|l| get_calibration_value(l.to_string())).collect();
    let answer: u32 = calibration_values.iter().sum();
    println!("Part 1: This sum of the calibration values: {}", answer)
}

fn get_calibration_value(line: String) -> u32 {
    let mut first_digit: char = 'a';
    let mut last_digit: char = 'a';
    for char in line.chars() {
        let num = char.to_string().parse::<u32>();
        if num.is_err() {
            continue
        }
        first_digit = char;
        break
    }
    for char in line.chars().rev() {
        let num = char.to_string().parse::<u32>();
        if num.is_err() {
            continue
        }
        last_digit = char;
        break
    }
    let digit = [first_digit.to_string(), last_digit.to_string()].join(""); 
    match digit.parse() {
        Ok(num) => num,
        Err(_) => 0
    }
}

#[test]
fn test_get_calibration_value() {
    // two digits in the string
    let num1 = get_calibration_value("abc3edf5xzy".to_string());
    assert_eq!(35, num1);

    // More than two digits in the string
    let num2 = get_calibration_value("1abc3edf5xzy2".to_string());    
    assert_eq!(12, num2);

    // value is zero when there isn't a digit present
    let num3 = get_calibration_value("abcdef".to_string());    
    assert_eq!(0, num3);

    // single digit in the string counts for both the first and last digit
    let num4 = get_calibration_value("abc5def".to_string());    
    assert_eq!(55, num4);

    let line = "ncbfctqlsnfive1brqpthree4".to_string();
    let num5 = get_calibration_value(line.to_string());
    assert_eq!(14, num5)
}