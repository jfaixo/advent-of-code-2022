use std::{env, fs};
use std::error::Error;
use std::process::exit;

struct Grid {
    width: usize,
    height: usize,
    trees: Vec<i8>,
}

fn parse_data(data: String) -> Grid {
    let height = data.lines().count();
    let width = data.lines().nth(0).unwrap().trim().len();

    let mut grid = Grid {
        width,
        height,
        trees: vec![-1; width * height],
    };

    for (y, line) in data.lines().enumerate() {
        for x in 0..width {
            grid.trees[y * width + x] = line[x..x+1].parse().unwrap();
        }
    }

    grid
}

fn solve_part_1(grid: &Grid) -> usize {
    let mut visible_trees = vec![0; grid.width * grid.height];

    // Horizontal
    for y in 0..grid.height {
        // Left to right
        let mut highest = -1;
        for x in 0..grid.width {
            let tree = grid.trees[grid.width * y + x];
            if tree > highest {
                visible_trees[grid.width * y + x] = 1;
                highest = tree;
            }
        }

        // Right to left
        let mut highest = -1;
        for x in (0..grid.width).rev() {
            let tree = grid.trees[grid.width * y + x];
            if tree > highest {
                visible_trees[grid.width * y + x] = 1;
                highest = tree;
            }
        }
    }

    // Verticalfn solve_part_1(grid: &Grid) -> usize {
    for x in 0..grid.width {
        // Top to bottom
        let mut highest = -1;
        for y in 0..grid.height {
            let tree = grid.trees[grid.width * y + x];
            if tree > highest {
                visible_trees[grid.width * y + x] = 1;
                highest = tree;
            }
        }

        // Bottom to top
        let mut highest = -1;
        for y in (0..grid.height).rev() {
            let tree = grid.trees[grid.width * y + x];
            if tree > highest {
                visible_trees[grid.width * y + x] = 1;
                highest = tree;
            }
        }
    }

    visible_trees.iter().sum()
}

fn solve_part_2(grid: &Grid) -> usize {
    let mut scenic_scores = vec![0; grid.width * grid.height];

    for y in 0..grid.height {
        for x in 0..grid.width {

            // Top
            let mut top_view = 0;
            let mut highest = grid.trees[y * grid.width + x];
            if y > 0 {
                for dy in (0..y).rev() {
                    let tree= grid.trees[dy * grid.width + x];
                    top_view += 1;

                    if tree >= highest {
                        break;
                    }
                }
            }

            // Bottom
            let mut bottom_view = 0;
            let mut highest = grid.trees[y * grid.width + x];
            if y < grid.height {
                for dy in (y + 1..grid.height) {
                    let tree= grid.trees[dy * grid.width + x];
                    bottom_view += 1;

                    if tree >= highest {
                        break;
                    }
                }
            }

            // Left view
            let mut left_view = 0;
            let mut highest = grid.trees[y * grid.width + x];
            if x > 0 {
                for dx in (0..x).rev() {
                    let tree= grid.trees[y * grid.width + dx];
                    left_view += 1;

                    if tree >= highest {
                        break;
                    }
                }
            }

            // Right
            let mut right_view = 0;
            let mut highest = grid.trees[y * grid.width + x];
            if x < grid.width {
                for dx in x + 1..grid.width {
                    let tree= grid.trees[y * grid.width + dx];
                    right_view += 1;

                    if tree >= highest {
                        break;
                    }
                }
            }

            scenic_scores[y * grid.width + x] = top_view * bottom_view * left_view * right_view;
        }
    }

    *scenic_scores.iter().max().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let data =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");

    let grid = parse_data(data);

    println!("part 1: {}", solve_part_1(&grid));
    println!("part 2: {}", solve_part_2(&grid));

    Ok(())
}