use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

// Part 1

type Seed = usize;

pub fn run_day_5_part_1() -> () {
    let file: File = File::open("inputs/day5.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let seeds = parse_seeds(lines.remove(0));
    let seed_map_hash = parse_maps(lines);
    let mut location_nums: Vec<usize> = vec![];
    for seed in seeds.iter() {
        let loc_num = calculate_location_number(*seed, &seed_map_hash);
        location_nums.push(loc_num)
    }
    let min_loc_num = location_nums.iter().min().unwrap();
    println!("Part 1: The smallest location number among seeds in inputs/day5.txt ==> {}", min_loc_num)
}

fn parse_seeds(line: String) -> Vec<usize> {
    let re = Regex::new(r"(\d+)").unwrap();
    let seeds = re.find_iter(&line).map(|m| m.as_str().parse::<Seed>().unwrap()).collect();
    seeds
}

#[test]
fn test_parse_seeds() {
    assert_eq!(vec![123, 456, 789], parse_seeds(String::from("123 456 789")))
}


#[derive(PartialEq, Debug)]
struct SeedMapRange {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

#[derive(PartialEq, Debug)]
struct SeedMap {
    destination: String,
    ranges: Vec<SeedMapRange>,
}

fn parse_maps(lines: Vec<String>) -> HashMap<String, SeedMap> {
    let mut hash = HashMap::<String, SeedMap>::new();
    let mut current_source = String::from("seed");
    for line in lines.iter() { 
        if line.contains("map:") {
            let re = Regex::new(r"(\w+)").unwrap();
            let words: Vec<String> = re.find_iter(&line).map(|m| m.as_str().parse::<String>().unwrap()).collect(); 
            if let Some(source) = words.get(0) {
                current_source = String::from(source);
                if let Some(destination) = words.get(2) {
                    let map = SeedMap {
                        destination: String::from(destination),
                        ranges: vec![]
                    };
                    hash.insert(String::from(source), map);
                }
            }
        } else if line.trim().is_empty() {
            continue
        } else {
            if let Some(map) = hash.get_mut(&current_source) {
                let re = Regex::new(r"(\d+)").unwrap();
                let mut nums: Vec<usize> = re.find_iter(line).map(|s| s.as_str().parse::<usize>().unwrap()).collect();
                let length = nums.pop().unwrap();
                let source_start = nums.pop().unwrap();
                let destination_start = nums.pop().unwrap();
                map.ranges.push(SeedMapRange { source_start, destination_start, length })
            }
        }
    }
    hash 
}

#[test]
fn test_parse_maps() {
    let string = String::from("seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15");
    let soil_map = SeedMap {
        destination: String::from("soil"),
        ranges: vec![
            SeedMapRange {
                source_start: 98,
                destination_start: 50,
                length: 2
            },
            SeedMapRange {
                source_start: 50,
                destination_start: 52,
                length: 48
            }
        ],
    };
    let fert_map = SeedMap {
        destination: String::from("fertilizer"),
        ranges: vec![
            SeedMapRange {
                source_start: 15,
                destination_start: 0,
                length: 37
            },
            SeedMapRange {
                source_start: 52,
                destination_start: 37,
                length: 2
            },
            SeedMapRange {
                source_start: 0,
                destination_start: 39,
                length: 15
            }
        ],
    };
    let mut hash = HashMap::<String, SeedMap>::new();
    hash.insert(String::from("seed"), soil_map);
    hash.insert(String::from("soil"), fert_map);
    let lines: Vec<String> = string.split("\n").map(|s| String::from(s)).collect();
    let seed_map_hash = parse_maps(lines);
    assert_eq!(
        hash,
        seed_map_hash
    );
}

fn calculate_location_number(seed: Seed, map_hash: &HashMap<String, SeedMap>) -> usize {
    let mut loc_num = seed;
    let mut current_source = String::from("seed");
    println!("{} ===> {}", current_source, loc_num);
    while current_source != "location" {
        if let Some(map) = map_hash.get(&current_source) { 
            // Check all the source ranges...
            for range in &map.ranges {
                if loc_num >= range.source_start && loc_num <= (range.source_start + range.length) {
                    let next_num_idx = loc_num - range.source_start;
                    loc_num = range.destination_start + next_num_idx;
                    break
                }
            }
            // set the current_source to the next destination
            current_source = String::from(&map.destination);
            println!("{} ===> {}", current_source, loc_num);
        }
    }
    loc_num
}

#[test]
fn test_calculate_location_number() {
    let string = String::from("seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4");
    let lines: Vec<String> = string.split("\n").map(|s| String::from(s)).collect();
    let seed_map_hash = parse_maps(lines);
    assert_eq!(
        82,
        calculate_location_number(79, &seed_map_hash)
    );
    assert_eq!(
        43,
        calculate_location_number(14, &seed_map_hash)
    );
    assert_eq!(
        86,
        calculate_location_number(55, &seed_map_hash)
    );
    assert_eq!(
        35,
        calculate_location_number(13, &seed_map_hash)
    );
}

// Part 2

pub fn run_day_5_part_2() -> () {
    let file: File = File::open("inputs/day5.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let seeds = parse_seeds(lines.remove(0));
    let seed_map_hash = parse_maps(lines);
    let mut location_nums: Vec<usize> = vec![];
    for seed in seeds.iter() {
        let loc_num = calculate_location_number(*seed, &seed_map_hash);
        location_nums.push(loc_num)
    }
    let min_loc_num = location_nums.iter().min().unwrap();
    println!("Part 1: The smallest location number among seeds in inputs/day5.txt ==> {}", min_loc_num)
}
