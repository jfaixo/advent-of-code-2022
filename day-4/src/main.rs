use std::error::Error;
use std::{env, fs};
use std::process::exit;

struct ElfRange {
    start: usize,
    end: usize,
}

fn fully_overlap(a: &ElfRange, b: &ElfRange) -> bool {
    (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end)
}

fn partially_overlap(a: &ElfRange, b: &ElfRange) -> bool {
    !(a.end < b.start || b.end < a.start)
}

fn parse_data(data: String) -> Vec<(ElfRange, ElfRange)> {
    data.lines().map(|line| {
        let parts : Vec<&str> = line.trim().split(&[',', '-']).collect();
        (ElfRange {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        },
         ElfRange {
             start: parts[2].parse().unwrap(),
             end: parts[3].parse().unwrap(),
         })
    }).collect()
}

fn solve_part_1(elves: &Vec<(ElfRange, ElfRange)>) -> usize {
    let mut result = 0;
    for (a, b) in elves {
        if fully_overlap(a, b) {
            result += 1;
        }
    }
    result
}

fn solve_part_2(elves: &Vec<(ElfRange, ElfRange)>) -> usize {
    let mut result = 0;
    for (a, b) in elves {
        if partially_overlap(a, b) {
            result += 1;
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

    let ref elf_ranges = parse_data(file_content);

    println!("part 1: {}", solve_part_1(elf_ranges));
    println!("part 2: {}", solve_part_2(elf_ranges));

    Ok(())
}