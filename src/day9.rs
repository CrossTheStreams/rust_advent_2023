use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_day_9_part_1() -> () {
    let file: File = File::open("inputs/day9.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let histories: Vec<Vec<isize>> = lines.into_iter().map(|l|
        l.split(" ").into_iter().map(|s|
            s.parse::<isize>().expect("Failed to parse integer!")
        ).collect()
    ).collect();
    let values: Vec<isize> = histories.into_iter().map(|hist| calculate_next_value(hist)).collect();
    let answer = values.into_iter().reduce(|a, b| a + b).expect("Expected to find an answer!");
    println!("Part 1: The sum of the next values in the histories in inputs/day9.txt ==> {}", answer);
}

fn calculate_next_value(history: Vec<isize>) -> isize {
    let mut diffs: Vec<isize> = vec![]; 
    let mut only_zeros = false;
    let mut last_diff = history.clone();
    // Calculate next number of history using the algorithm outlined in the directions...
    // We calculate the diff of each item in the history
    // Then the diffs of those values... until we arrive at a row of all zeros
    while !only_zeros {
        only_zeros = true;
        let mut idx = 0;
        let mut new_diffs: Vec<isize> = vec![];
        for num in &last_diff {
            if idx == 0 {
                idx += 1;
                continue;
            }
            let prev = last_diff.get(idx - 1).unwrap();
            let diff = num - prev;
            if diff != 0 {
                only_zeros = false;
            }
            new_diffs.push(diff);
            if idx == last_diff.len() - 1 {
                break;
            }
            idx += 1;
        }
        last_diff = new_diffs.clone();
        diffs.push(new_diffs.pop().unwrap());
    }
    diffs.push(*history.last().expect("Expected a value in given history!"));
    let mut val = 0 as isize;
    for diff in diffs.into_iter().rev() { 
        val += diff
    } 
    val
}

#[test]
fn test_calculate_next_value() {
    let history = vec![0, 3, 6, 9, 12, 15];
    assert_eq!(
        18,
        calculate_next_value(history)
    )
}