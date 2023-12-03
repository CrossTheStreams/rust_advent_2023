use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn run_day_2_part_1() -> () {
    let file = File::open("inputs/day2.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let games: Vec<Game> = lines.iter().map(|l| game_from_string(l)).collect();
    let answer = sum_of_possible_games(games);
    println!("Part 1: This is the sum of the Game IDs of possible games: {}", answer)
}

#[derive(Debug)]
struct Game {
    game_id: u32,
    rounds: Vec<Round>
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

// A round of the game, which is separated by a semicolon in each line of the input day2.txt
// I got a little mixed up at first about the problem at first.
// The cubes are put *back into the bag* after each round, so we simply check *the cube count of each round*.
impl Round {
    pub fn new() -> Self {
        return Self {
            red: 0,
            green: 0,
            blue: 0
        }
    }
}

impl Game {
    pub fn new(game_id: u32) -> Self {
        return Self {
            game_id,
            rounds: vec![]
        }
    }

    pub fn possible(&self) -> bool {
        for round in &self.rounds {
            if round.red < 13 && round.green < 14 && round.blue < 15 {
                continue
            }
            return false
        }
        return true
    }
}

fn game_from_string(line: &str) -> Game {
    let game_id_re = Regex::new(r"(\d+)").unwrap();
    let game_id_match = game_id_re.find(line).unwrap();
    let game_id_str = game_id_match.as_str();
    let game_id = game_id_str.parse::<u32>().unwrap();
    let round_strs: Vec<&str> = line.split(";").collect();
    let count_re = Regex::new(r"(\d+)\s+(red|blue|green)").unwrap();
    let mut game = Game::new(game_id);
    for round_str in round_strs {
        let mut round = Round::new(); 
        for (_, [count, color]) in count_re.captures_iter(round_str).map(|c| c.extract()) {
            let count_num = count.parse::<u32>().unwrap_or(0);
            match color { 
                "red" => {
                    round.red += count_num;
                },
                "blue" => {
                    round.blue += count_num;
                },
                "green" => {
                    round.green += count_num;
                }
                _ => {
                    panic!("lol what")
                }
            }
        }
        game.rounds.push(round);
    }
    return game
}

fn sum_of_possible_games(games: Vec<Game>) -> u32 {
    let mut sum: u32 = 0;
    for game in games {
        if game.possible() {
            sum += game.game_id;
        }
    }
    return sum
}

#[test]
fn test_game_from_string_counts() {
    let game = game_from_string("Game 1: 3 blue; 4 red; 7 green, 2 red;");
    assert_eq!(game.rounds[0].blue, 3);
    assert_eq!(game.rounds[1].red, 4);
    assert_eq!(game.rounds[2].green, 7);
}

#[test]
fn test_game_from_string_possible() {
    let possible_game1 = game_from_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    let possible_game2 = game_from_string("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    let possible_game3 = game_from_string("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    assert!(possible_game1.possible());
    assert!(possible_game2.possible());
    assert!(possible_game3.possible());
}

#[test]
fn test_game_from_string_impossible() {
    let impossible_game1 = game_from_string("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
    let impossible_game2 = game_from_string("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
    assert!(!impossible_game1.possible());
    assert!(!impossible_game2.possible());
}

#[test]
fn test_sum_of_possible_games() {
    let possible_game1 = game_from_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    let possible_game2 = game_from_string("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    let possible_game3 = game_from_string("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    let impossible_game1 = game_from_string("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
    let impossible_game2 = game_from_string("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
    let games = vec![
        possible_game1,
        possible_game2,
        possible_game3,
        impossible_game1,
        impossible_game2
    ];
    let sum = sum_of_possible_games(games);
    assert_eq!(8, sum)
}
