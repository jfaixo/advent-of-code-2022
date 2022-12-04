mod rucksack;

use std::error::Error;
use std::{env, fs};
use std::process::exit;
use crate::rucksack::{parse_data, solve_part_1, solve_part_2};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let file_content =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");

    let ref rucksacks = parse_data(file_content);

    println!("part 1: {}", solve_part_1(rucksacks));
    println!("part 2: {}", solve_part_2(rucksacks));

    Ok(())
}