use std::error::Error;
use std::{env, fs};
use std::process::exit;

#[derive(Default)]
struct Elves {
    pub elves: Vec<Vec<i32>>,
}

impl Elves {
    fn parse_string(input: String) -> Result<Self, Box<dyn Error>> {
        let mut calories = Elves::default();
        let mut acc = Vec::new();
        for line in input.lines() {
            if line.is_empty() {
                calories.elves.push(acc);
                acc = Vec::new();
            } else {
                let snack = line.trim().parse::<i32>()?;
                acc.push(snack);
            }
        }

        Ok(calories)
    }

    fn solve_part_1(&self) -> i32 {
        self.elves.iter().map(|elf| {
            elf.iter().sum()
        }).max().unwrap()
    }

    fn solve_part_2(&self) -> i32 {
        let mut sum_elves : Vec<i32> = self.elves.iter().map(|elf| {
            elf.iter().sum()
        }).collect();
        sum_elves.sort();
        sum_elves.reverse();

        sum_elves[0] + sum_elves[1] + sum_elves[2]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let file_content =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");
    let input = Elves::parse_string(file_content)?;

    println!("part 1: {}", input.solve_part_1());
    println!("part 2: {}", input.solve_part_2());

    Ok(())
}