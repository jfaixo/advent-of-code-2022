use std::{env, fs};
use std::error::Error;
use std::process::exit;

struct Cargo {
    stacks: [Vec<char>; 9],
}

impl Cargo {
    fn new() -> Self {
        Cargo {
            stacks: [
                vec!['T', 'D', 'W', 'Z', 'V', 'P'],
                vec!['L', 'S', 'W', 'V', 'F', 'J', 'D'],
                vec!['Z', 'M', 'L', 'S', 'V', 'T', 'B', 'H'],
                vec!['R', 'S', 'J'],
                vec!['C', 'Z', 'B', 'G', 'F', 'M', 'L', 'W'],
                vec!['Q', 'W', 'V', 'H', 'Z', 'R', 'G', 'B'],
                vec!['V', 'J', 'P', 'C', 'B', 'D', 'N'],
                vec!['P', 'T', 'B', 'Q'],
                vec!['H', 'G', 'Z', 'R', 'C'],
            ],
        }
    }
}

fn parse_data(data: String) -> Vec<(usize, usize, usize)> {
    data.lines().map(|line| {
        let parts = line[5..].split(" from ").collect::<Vec<_>>();
        let count = parts[0].parse().unwrap();
        let parts = parts[1].trim().split(" to ").collect::<Vec<_>>();
        (count, parts[0].parse().unwrap(), parts[1].trim().parse().unwrap())
    }).collect()
}

fn solve_part_1(moves: &Vec<(usize, usize, usize)>) -> String {
    let mut cargo = Cargo::new();

    for (count, from, to) in moves {
        let from = from - 1;
        let to = to - 1;
        for _ in 0..*count {
            if cargo.stacks[from].len() > 0 {
                let character = cargo.stacks[from].pop().unwrap();
                cargo.stacks[to].push(character);
            }
        }
    }

    let mut result = String::new();
    for s in cargo.stacks {
        if s.len() > 0 {
            result.push(*s.last().unwrap())
        }
    }
    result
}

fn solve_part_2(moves: &Vec<(usize, usize, usize)>) -> String {
    let mut cargo = Cargo::new();

    for (count, from, to) in moves {
        let from = from - 1;
        let to = to - 1;

        let from_length = cargo.stacks[from].len();
        let from_copy = cargo.stacks[from].clone();
        let crates = &from_copy[from_length - count ..];
        cargo.stacks[to].extend_from_slice(crates);
        cargo.stacks[from].truncate(from_length - count);
    }

    let mut result = String::new();
    for s in cargo.stacks {
        if s.len() > 0 {
            result.push(*s.last().unwrap())
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let file_content =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");

    let ref moves = parse_data(file_content);


    println!("part 1: {}", solve_part_1(moves));
    println!("part 2: {}", solve_part_2(moves));

    Ok(())
}