use std::{env, fs};
use std::collections::HashSet;
use std::error::Error;
use std::process::exit;

#[derive(Debug)]
struct Move {
    n: usize,
    direction: Direction,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_data(data: String) -> Vec<Move> {
    data.lines().map(|line| {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let n = parts[1].parse::<usize>().unwrap();
        match parts[0] {
            "R" => Move { n, direction: Direction::Right },
            "L" => Move { n, direction: Direction::Left },
            "U" => Move { n, direction: Direction::Up },
            _ => Move { n, direction: Direction::Down },
        }
    }).collect()
}

fn solve_part_1(moves: &Vec<Move>) -> usize {
    let mut visited = HashSet::new();
    let mut head_position : (i32, i32) = (0, 0);
    let mut tail_position = (0, 0);

    visited.insert(tail_position);

    for m in moves {
        for i in 0..m.n {
            match m.direction {
                Direction::Up => {
                    head_position.1 -= 1;
                }
                Direction::Down => {
                    head_position.1 += 1;
                }
                Direction::Left => {
                    head_position.0 -= 1;
                }
                Direction::Right => {
                    head_position.0 += 1;
                }
            }

            // Diagonal move
            let dx = head_position.0 - tail_position.0;
            let dy = head_position.1 - tail_position.1;
            let adx = dx.abs();
            let ady = dy.abs();

            if adx > 1 || ady > 1 {
                if adx != 0 {
                    tail_position.0 += dx / adx;
                }

                if ady != 0 {
                    tail_position.1 += dy / ady;
                }
            }

            visited.insert(tail_position);
        }
    }

    visited.len()
}

fn solve_part_2(moves: &Vec<Move>) -> usize {
    let mut visited = HashSet::new();
    let mut snake : [(i32, i32); 10] = [(0, 0); 10];

    visited.insert(snake[9]);

    for m in moves {
        for i in 0..m.n {
            match m.direction {
                Direction::Up => {
                    snake[0].1 -= 1;
                }
                Direction::Down => {
                    snake[0].1 += 1;
                }
                Direction::Left => {
                    snake[0].0 -= 1;
                }
                Direction::Right => {
                    snake[0].0 += 1;
                }
            }

            // moves
            for j in 0..9 {
                // Diagonal move
                let dx = snake[j].0 - snake[j + 1].0;
                let dy = snake[j].1 - snake[j + 1].1;
                let adx = dx.abs();
                let ady = dy.abs();

                if adx > 1 || ady > 1 {
                    if adx != 0 {
                        snake[j + 1].0 += dx / adx;
                    }

                    if ady != 0 {
                        snake[j + 1].1 += dy / ady;
                    }
                }
            }

            visited.insert(snake[9]);
        }
    }

    visited.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let data =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");

    let directions = parse_data(data);

    println!("part 1: {}", solve_part_1(&directions));
    println!("part 2: {}", solve_part_2(&directions));

    Ok(())
}