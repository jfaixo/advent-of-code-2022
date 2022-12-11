use std::{env, fs};
use std::error::Error;
use std::process::exit;

#[derive(Debug)]
enum Instruction {
    Addx(i64),
    Noop
}

fn parse_data(data: String) -> Vec<Instruction> {
    data.lines().map(|line| {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        if parts[0] == "noop" {
            Instruction::Noop
        }
        else {
            Instruction::Addx(parts[1].parse().unwrap())
        }
    })
        .collect()
}

fn solve_part_1(instructions: &[Instruction]) -> i64 {
    let mut pc = 1;
    let mut x = 1;

    let mut strength = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Addx(v) => {
                if pc <= 220 && (pc - 20) % 40 == 0 {
                    strength += pc * x;
                }
                pc += 1;
                if pc <= 220 && (pc - 20) % 40 == 0 {
                    strength += pc * x;
                }
                pc += 1;
                x += v;
            }
            Instruction::Noop => {
                if pc <= 220 && (pc - 20) % 40 == 0 {
                    strength += pc * x;
                }
                pc += 1;
            }
        }
    }

    strength
}

fn solve_part_2(instructions: &[Instruction]) -> i64 {
    let mut pc : i64 = 0;
    let mut x = 1;

    let mut screen = ['.'; 6 * 40];

    for instruction in instructions {
        match instruction {
            Instruction::Addx(v) => {
                let v = *v;
                if pc % 40 >= x-1 && pc % 40 <= x + 1 {
                    screen[pc as usize] = '#';
                } else {
                    screen[pc as usize] = '.';
                }
                pc += 1;
                if pc % 40 >= x-1 && pc % 40 <= x + 1 {
                    screen[pc as usize] = '#';
                } else {
                    screen[pc as usize] = '.';
                }
                pc += 1;
                x += v;
            }
            Instruction::Noop => {
                if pc % 40 >= x-1 && pc % 40 <= x + 1 {
                    screen[pc as usize] = '#';
                } else {
                    screen[pc as usize] = '.';
                }
                pc += 1;
            }
        }
    }

    for (i, c) in screen.iter().enumerate() {
        print!("{}", c);
        if (i + 1) % 40 == 0 {
            println!();
        }
    }

    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let data =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");

    let instructions = parse_data(data);

    eprintln!("{:?}", instructions);
    println!("part 1: {}", solve_part_1(&instructions));
    println!("part 2: {}", solve_part_2(&instructions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_data, solve_part_1, solve_part_2};

    #[test]
    fn sample_1() {
        let data = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
        let instructions = parse_data(data.to_string());
        let strength = solve_part_1(&instructions);

        assert_eq!(strength, 13140);

        solve_part_2(&instructions);
    }
}