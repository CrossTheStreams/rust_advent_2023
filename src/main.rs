#![allow(dead_code)]

use crate::day1::get_calibration_value;

mod day1;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/day1.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let calibration_values: Vec<u32> = lines.iter().map(|l| get_calibration_value(l.to_string())).collect();
    let answer: u32 = calibration_values.iter().sum();
    println!("Part 1: This sum of the calibration values: {}", answer)
}

