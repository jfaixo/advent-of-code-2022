use std::{env, fs};
use std::error::Error;
use std::process::exit;

#[derive(Debug, Copy, Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn from_string(string: &str) -> Self {
        match string {
            "A" | "X" => Action::Rock,
            "B" | "Y" => Action::Paper,
            "C" | "Z" => Action::Scissors,
            _ => panic!("Unknown pattern {}", string)
        }
    }

    fn shape_score(&self) -> u64 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Loose,
    Draw,
    Win,
}

impl Outcome {
    fn from_string(string: &str) -> Self {
        match string {
            "X" => Outcome::Loose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unknown pattern {}", string)
        }
    }

    fn score(&self) -> u64 {
        match self {
            Outcome::Loose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn action_to_play(&self, a: &Action) -> Action {
        match self {
            Outcome::Loose => {
                match a {
                    Action::Rock => Action::Scissors,
                    Action::Paper => Action::Rock,
                    Action::Scissors => Action::Paper,
                }
            }
            Outcome::Draw => *a,
            Outcome::Win => {
                match a {
                    Action::Rock => Action::Paper,
                    Action::Paper => Action::Scissors,
                    Action::Scissors => Action::Rock,
                }
            }
        }
    }
}

fn round_score(a: &Action, b: &Action) -> u64 {
    match a {
        Action::Rock => {
            match b {
                Action::Rock => 3,
                Action::Paper => 6,
                Action::Scissors => 0
            }
        }
        Action::Paper => {
            match b {
                Action::Rock => 0,
                Action::Paper => 3,
                Action::Scissors => 6
            }
        }
        Action::Scissors => {
            match b {
                Action::Rock => 6,
                Action::Paper => 0,
                Action::Scissors => 3
            }
        }
    }
}

#[derive(Debug)]
struct EncryptedStrategyGuide {
    guide: Vec<(Action, Action, Outcome)>,
}

impl EncryptedStrategyGuide {
    fn parse_data(data: String) -> Self {
        let guide = data.lines().map(|line| {
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
            (Action::from_string(parts[0]), Action::from_string(parts[1]), Outcome::from_string(parts[1]))
        })
            .collect();

        EncryptedStrategyGuide {
            guide
        }
    }

    fn solve_part_1(&self) -> u64 {
        self.guide.iter().map(|(a, b, _)| {
            b.shape_score() + round_score(a, b)
        }).sum()
    }

    fn solve_part_2(&self) -> u64 {
        self.guide.iter().map(|(a, _, o)| {
            o.action_to_play(a).shape_score() + o.score()
        }).sum()
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

    let guide = EncryptedStrategyGuide::parse_data(file_content);

    println!("part 1: {}", guide.solve_part_1());
    println!("part 2: {}", guide.solve_part_2());

    Ok(())
}