use std::{env, fs};
use std::error::Error;
use std::process::exit;

fn solve_part_1(data: String) -> usize {
    let mut diff_count = 0;
    let mut previous = ['0', '0', '0'];

    for i in 0..data.len() {
        let c = data.chars().nth(i).unwrap();
        let mut same = usize::MAX;
        for j in 0..diff_count {
            if c == previous[j] {
                same = j;
            }
        }

        if same != usize::MAX {
            for j in same..diff_count - 1 {
                previous[j - same] = previous[1 + j];
            }
            diff_count -= same + 1;
        }

        diff_count += 1;
        if diff_count == 4 {
            return i + 1;
        }
        previous[diff_count - 1] = data.chars().nth(i).unwrap();
    }

    0
}

fn solve_part_2(data: String) -> usize {
    let mut diff_count = 0;
    let mut previous = ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'];

    for i in 0..data.len() {
        let c = data.chars().nth(i).unwrap();
        let mut same = usize::MAX;
        for j in 0..diff_count {
            if c == previous[j] {
                same = j;
            }
        }

        if same != usize::MAX {
            for j in same..diff_count - 1 {
                previous[j - same] = previous[1 + j];
            }
            diff_count -= same + 1;
        }

        diff_count += 1;
        if diff_count == 14 {
            return i + 1;
        }
        previous[diff_count - 1] = data.chars().nth(i).unwrap();
    }

    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let data_stream =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");

    println!("part 1: {}", solve_part_1(data_stream.clone()));
    println!("part 2: {}", solve_part_2(data_stream.clone()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::solve_part_1;

    #[test]
    fn sample_1() {
        let result = solve_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());

        assert_eq!(result, 5);
    }

    #[test]
    fn sample_2() {
        let result = solve_part_1("nppdvjthqldpwncqszvftbrmjlhg".to_string());

        assert_eq!(result, 6);
    }

    #[test]
    fn sample_3() {
        let result = solve_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string());

        assert_eq!(result, 10);
    }

    #[test]
    fn sample_4() {
        let result = solve_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string());

        assert_eq!(result, 11);
    }
}