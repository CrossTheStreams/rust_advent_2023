use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn run_day_8_part_1() -> () {
    let file: File = File::open("inputs/day8.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines: VecDeque<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let moves = lines.pop_front().unwrap();
    lines.pop_front(); // Drop the empty line below the moves 
    let hashmap = parse_lines_as_node_hashmap(Vec::<String>::from(lines));
    let mut steps = 0;
    let mut current = hashmap.get("AAA").unwrap();
    loop {
        if current.val == "ZZZ" {
            break
        }
        for move_chr in moves.chars() {
            if current.val == "ZZZ" {
                break
            }
            match move_chr {
            'L' => {
                    steps += 1;
                    current = hashmap.get(&current.left).unwrap();
            },
            'R' => {
                    steps += 1;
                    current = hashmap.get(&current.right).unwrap();
            },
            _ => panic!("Invalid move!")
            }
        }
    }
    println!("Part 1: Steps to go from AAA to ZZZ ==> {}", steps);
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    val: String,
    right: String,
    left: String,
}

fn parse_line_as_node(line: String) -> Node {
    let re = Regex::new("[A-Z]{3}").unwrap();
    let vals: Vec<String> = re.find_iter(&line).map(|m| String::from(m.as_str())).collect();
    let node = Node {
        val: vals.get(0).expect("There should be a val here").to_string(),
        left: vals.get(1).expect("There should be a val here").to_string(),
        right: vals.get(2).expect("There should be a val here").to_string(),
    };
    node
}

#[test]
fn test_parse_line_as_node() {
    let line = String::from("AAA = (BBB, CCC)");
    let node = Node {
        val: String::from("AAA"),
        left: String::from("BBB"),
        right: String::from("CCC"),
    };
    assert_eq!(parse_line_as_node(line), node)
}

fn parse_lines_as_node_hashmap(lines: Vec<String>) -> HashMap<String, Node> {
    let mut hashmap = HashMap::<String, Node>::new();
    for line in lines {
        let node = parse_line_as_node(line);
        hashmap.insert(node.val.clone(), node);
    }
    hashmap
}

#[test]
fn test_parse_lines_as_node_hashmap() {
    let lines = vec![
        String::from("AAA = (BBB, CCC)"),
        String::from("BBB = (DDD, EEE)")
    ];
    let mut hashmap = HashMap::<String, Node>::new();
    hashmap.insert(
        String::from("AAA"),
        Node {
            val: String::from("AAA"),
            left: String::from("BBB"),
            right: String::from("CCC"),
        },
    );
    hashmap.insert(
        String::from("BBB"),
        Node {
            val: String::from("BBB"),
            left: String::from("DDD"),
            right: String::from("EEE"),
        },
    );
    assert_eq!(
        parse_lines_as_node_hashmap(lines),
        hashmap
    )
}