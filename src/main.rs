#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use clap::Parser;
use crate::day1::{run_day_1_part_1, run_day_1_part_2};
use crate::day2::{run_day_2_part_1, run_day_2_part_2};
use crate::day3::{run_day_3_part_1, run_day_3_part_2};
use crate::day4::{run_day_4_part_1, run_day_4_part_2};
use crate::day5::run_day_5_part_1;
use crate::day6::run_day_6_part_1;

use std::error::Error;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of Advent, a value 1 through 25
    #[arg(short, long)]
    day: u8,

    /// Part 1 or 2 Problem of the Day, value must be either 1 or 2
    #[arg(short, long)]
    part: u8,
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let day = args.day;
    let part = args.part;
    match day {
        1..=25 => (),
        _ => return Err("--day must be a value 1..25".into())
    }
    match part {
        1|2 => (),
        _ => return Err("--part must be 1 or 2".into())
    }

    match (day, part) {
        (1, 1) => {
            run_day_1_part_1();
        },
        (1, 2) => {
            run_day_1_part_2();
        },
        (2, 1) => {
            run_day_2_part_1();
        },
        (2, 2) => {
            run_day_2_part_2();
        },
        (3, 1) => {
            run_day_3_part_1();
        },
        (3, 2) => {
            run_day_3_part_2();
        }
        (4, 1) => {
            run_day_4_part_1();
        },
        (4, 2) => {
            run_day_4_part_2();
        },
        (5, 1) => {
            run_day_5_part_1();
        },
        (6, 1) => {
            run_day_6_part_1();
        }
        _ => {
            println!("Haven't done that one yet 🎅☃️🎄")
        }
    }
    Ok(())
}

