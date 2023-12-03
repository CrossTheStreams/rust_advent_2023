use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Part 1

pub fn run_day_3_part_1() -> () {
    let file: File = File::open("inputs/day3.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let part_numbers: Vec<u32> = find_part_numbers(lines);
    let answer: u32 = part_numbers.iter().sum();
    println!("Part 1: The sum of part numbers: {}", answer)
}

fn find_part_numbers(schematic_lines: Vec<String>) -> Vec<u32> { 
    let mut part_numbers: Vec<u32> = vec![];
    let mut part_number_coors = HashSet::new();
    let moves: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];
    for (i, line) in schematic_lines.iter().enumerate() { 
        let i_i32 = i as i32;
        for (j, chr) in line.chars().enumerate() {
            let j_i32 = j as i32;
            match chr {
                '0'..='9' => continue,
                '.' => continue,
                _ => {
                    println!("Walking around {:?}", (i, j));
                    // We've arrived at a symbol of some sort
                    // Look for part numbers around this coordinate
                    for m in &moves {
                        let row_idx = i_i32 + m.0;
                        let col_idx = j_i32 + m.1;
                        println!("    👀 {:?}", (row_idx, col_idx));
                        let row = schematic_lines.get(row_idx as usize);
                        if row.is_some() {
                            let row_string: &str = row.unwrap();
                            let col = row_string.chars().nth(col_idx as usize);
                            if col.is_some() {
                                let col_char: char = col.unwrap();
                                if col_char.is_digit(10) {
                                    println!("        Found digit at {:?}", (row_idx, col_idx));
                                    // We've found a digit around the coordinate.
                                    // Walk left to the start of the number, if we need to.
                                    // If that's the case, there will be a spot to our left, 
                                    // and it will be a number.
                                    let mut start_col_idx = col_idx;
                                    loop {
                                        if start_col_idx == 0 {
                                            break;
                                        }
                                        let left_char = row_string.chars().nth((start_col_idx - 1) as usize);
                                        if left_char.is_some() {
                                            if left_char.unwrap().is_digit(10) {
                                                start_col_idx = start_col_idx - 1;
                                                println!("            Walking back to {:?}", (row_idx, start_col_idx));
                                                continue;
                                            }
                                            break;
                                        }
                                    }
                                    println!("                Adding coor to set {:?}", (row_idx, start_col_idx));
                                    let number_coor = (row_idx as usize, start_col_idx as usize);
                                    part_number_coors.insert(number_coor);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for part_number_coor in &part_number_coors {
        let mut part_number_str = String::from("");
        let row_idx = part_number_coor.0;
        let mut col_idx = part_number_coor.1;
        loop {
            if col_idx == schematic_lines.len() {
                break
            }
            let row = schematic_lines.get(row_idx).unwrap();
            let part_number_chr = row.chars().nth(col_idx).unwrap();
            if part_number_chr.is_digit(10) {
                part_number_str.push(part_number_chr);
                col_idx += 1;
                continue
            }
            break
        }
        part_numbers.push(part_number_str.parse::<u32>().unwrap());
    }
    return part_numbers
}

#[test]
fn test_find_part_numbers() {
    let schematic = vec![
        String::from("467..114.."),
        String::from("...*......"),
        String::from("..35..633."),
        String::from("......#..."),
        String::from("617*......"),
        String::from(".....+.58."),
        String::from("..592....."),
        String::from("......755."),
        String::from("...$.*...."),
        String::from(".664.598.."),
    ];
    let part_numbers = find_part_numbers(schematic);
    let part_numbers_sum: u32 = part_numbers.iter().sum();
    // This is directly from the example from the problem.
    // Definitely could break down find_part_numbers into more focused functions, lol.
    assert_eq!(4361, part_numbers_sum);
}