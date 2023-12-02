#![allow(dead_code)]

use clap::Parser;
use crate::day1::{run_day_1_part_1, run_day_1_part_2};

mod day1;

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
        }
        _ => {
            println!("Haven't done that one yet 🎅☃️🎄")
        }
    }
    Ok(())
}

