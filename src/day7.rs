use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten, 
    J, 
    Q,
    K, 
    A, 
}

pub fn run_day_7_part_1() -> () {
    let file: File = File::open("inputs/day7.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    let mut hands: Vec<Hand> = lines.into_iter().map(|l| parse_hand(l)).collect();
    let mut winnings = 0;
    hands.sort();
    for (idx, mut hand) in hands.into_iter().enumerate() {
        hand.rank = (idx + 1) as u32;
        winnings += hand.winnings();
    }
    println!("Part 1: Winngings of all hands in inputs/day7.txt ==> {}", winnings);
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    rank: u32,
    bid: u32,
    cards: Vec<Card>,
    hand_type: HandType
}

impl Hand {
    fn winnings(&self) -> u32 {
        self.rank * self.bid
    }    
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand_type_cmp_opt = self.hand_type.partial_cmp(&other.hand_type);
        if hand_type_cmp_opt.is_some() {
            let hand_type_cmp = hand_type_cmp_opt.unwrap();
            if hand_type_cmp.is_eq() {
                return self.cards.partial_cmp(&other.cards)
            }
        }
        hand_type_cmp_opt
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_type_cmp.is_eq() {
            return self.cards.cmp(&other.cards)
        }
        hand_type_cmp
    }
}

fn parse_hand(line: String) -> Hand {
    let mut hand = Hand {
        rank: 0,
        bid: 0,
        cards: vec![],
        hand_type: HandType::HighCard,
    };
    // parse the cards in hand to derive the hand type
    let mut card_counts = HashMap::<Card, u32>::new();
    for chr in line.chars().into_iter() { 
        if chr == ' ' {
            break
        }
        let card: Card = match chr {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => { panic!("There shouldn't be other card faces here!") }
        };
        hand.cards.push(card);
        *card_counts.entry(card).or_insert(0) += 1;
    }
    let mut card_count_nums: Vec<u32> = card_counts.values().map(|v| v.clone()).collect();
    card_count_nums.sort();
    let hand_type = match card_count_nums[..] {
        [1, 1, 1, 1, 1] => HandType::HighCard,
        [1, 1, 1, 2] => HandType::OnePair,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 3] => HandType::ThreeOfAKind,
        [2, 3] => HandType::FullHouse,
        [1, 4] => HandType::FourOfAKind,
        [5] => HandType::FiveOfAKind,
        _ => { panic!("not a valid hand type") }
    };
    hand.hand_type = hand_type;

    // parse the bid value
    let mut bid_str = String::new();
    for chr in line.chars().into_iter().rev() {
        if chr == ' ' {
            break
        }
        bid_str.push(chr);
    }
    bid_str = bid_str.chars().into_iter().rev().map(|s| s).collect();
    let bid = bid_str.parse::<u32>().unwrap();
    hand.bid = bid;

    hand
}

#[test]
fn test_parse_one_pair_hand() {
    let high_card_str = String::from("32T3K 765");
    // String::from("T55J5 684"),
    // String::from("KK677 28"),
    // String::from("KTJJT 220"),
    // String::from("QQQJA 483"),
    assert_eq!(
        Hand {
            rank: 0,
            bid: 765,
            cards: vec![
                Card::Three,
                Card::Two,
                Card::Ten,
                Card::Three,
                Card::K,
            ],
            hand_type: HandType::OnePair
        },
        parse_hand(high_card_str)
    ); 
}

// let two_pair_str = String::from("T55J5 684");
// String::from("KTJJT 220"),
// String::from("QQQJA 483"),

#[test]
fn test_parse_two_pair_hand() {
    let two_pair_str = String::from("KK677 28");
    assert_eq!(
        Hand {
            rank: 0,
            bid: 28,
            cards: vec![
                Card::K,
                Card::K,
                Card::Six,
                Card::Seven,
                Card::Seven,
            ],
            hand_type: HandType::TwoPair
        },
        parse_hand(two_pair_str)
    ); 
}

#[test]
fn test_parse_three_of_a_kind_hand() {
    let three_of_a_kind = String::from("T55J5 684");
    assert_eq!(
        Hand {
            rank: 0,
            bid: 684,
            cards: vec![
                Card::Ten,
                Card::Five,
                Card::Five,
                Card::J,
                Card::Five,
            ],
            hand_type: HandType::ThreeOfAKind
        },
        parse_hand(three_of_a_kind)
    ); 
}

#[test]
fn test_parse_full_house_hand() {
    let full_house = String::from("QQQJJ 483");
    assert_eq!(
        Hand {
            rank: 0,
            bid: 483,
            cards: vec![
                Card::Q,
                Card::Q,
                Card::Q,
                Card::J,
                Card::J,
            ],
            hand_type: HandType::FullHouse
        },
        parse_hand(full_house)
    ); 
}


#[test]
fn test_parse_four_of_a_kind_hand() {
    let four_of_a_kind = String::from("QQQQJ 420");
    assert_eq!(
        Hand {
            rank: 0,
            bid: 420,
            cards: vec![
                Card::Q,
                Card::Q,
                Card::Q,
                Card::Q,
                Card::J,
            ],
            hand_type: HandType::FourOfAKind
        },
        parse_hand(four_of_a_kind)
    ); 
}


#[test]
fn test_parse_five_of_a_kind() {
    let full_house = String::from("KKKKK 773");
    assert_eq!(
        Hand {
            rank: 0,
            bid: 773,
            cards: vec![
                Card::K,
                Card::K,
                Card::K,
                Card::K,
                Card::K,
            ],
            hand_type: HandType::FiveOfAKind
        },
        parse_hand(full_house)
    ); 
}

#[test]
fn test_compare_cards() {
    assert!(Card::Two < Card::A);
    let full_house1 = Hand {
        rank: 0,
        bid: 10,
        cards: vec![Card::Seven, Card::Seven, Card::Seven, Card::Eight, Card::Eight],
        hand_type: HandType::FullHouse
    };
    let full_house2 = Hand {
        rank: 0,
        bid: 10,
        cards: vec![Card::Seven, Card::Seven, Card::Eight, Card::Eight, Card::Eight],
        hand_type: HandType::FullHouse
    };
    assert!(full_house2 > full_house1)
}

// #[test]
// fn test_winnings() {
//     let 
// }