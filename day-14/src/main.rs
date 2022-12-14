use std::{env, fs};
use std::error::Error;
use std::process::exit;

struct Cave {
    drawing: Vec<bool>,
    width: usize,
    height: usize,
    start_offset: (usize, usize),
}

impl Cave {
    fn parse(data: &str) -> Cave {
        let mut x_min = usize::MAX;
        let mut x_max = 0;
        let mut y_min = usize::MAX;
        let mut y_max = 0;
        for line in data.lines() {
            let points = line.split(" -> ");
            for point in points {
                let coords = point.split(',').map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
                if coords[0] < x_min {
                    x_min = coords[0];
                }
                if coords[0] > x_max {
                    x_max = coords[0];
                }
                if coords[1] < y_min {
                    y_min = coords[1];
                }
                if coords[1] > y_max {
                    y_max = coords[1];
                }
            }
        }
        let width = x_max + 1 - x_min;
        let height = y_max + 1;
        let mut cave = Cave {
            drawing: vec![false; width * height],
            width,
            height,
            start_offset: (x_min, 0),
        };

        for line in data.lines() {
            let points = line.split(" -> ");
            let mut previous_coords = None;
            for point in points {
                let coords = point.split(',').map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
                match &mut previous_coords {
                    None => previous_coords = Some(coords),
                    Some(previous_coords) => {
                        // Horizontal
                        if coords[0] != previous_coords[0] {
                            let mut bounds = [coords[0], previous_coords[0]];
                            bounds.sort();
                            for x in bounds[0]..=bounds[1] {
                                cave.drawing[(coords[1] - cave.start_offset.1) * cave.width + (x - cave.start_offset.0)] = true;
                            }
                        } else {
                            let mut bounds = [coords[1], previous_coords[1]];
                            bounds.sort();
                            for y in bounds[0]..=bounds[1] {
                                cave.drawing[(y - cave.start_offset.1) * cave.width + (coords[0] - cave.start_offset.0)] = true;
                            }
                        }
                        *previous_coords = coords;
                    }
                }
            }
        }

        cave
    }

    fn flood(&self) -> usize {
        let mut sand_count = 0;
        let mut filling_cave = self.drawing.clone();

        'outer: loop {
            let mut current_position = (500, 0);
            loop {
                if filling_cave[(0 - self.start_offset.1) * self.width + 500 - self.start_offset.0]
                    || current_position.1 >= self.start_offset.1 + self.height - 1 || current_position.0 < self.start_offset.0 || current_position.0 > self.start_offset.0 + self.width {
                    // Flooding infinity
                    break 'outer;
                }
                else if  !filling_cave[(current_position.1 + 1 - self.start_offset.1) * self.width + current_position.0 - self.start_offset.0] {
                    // Move down
                    current_position.1 += 1;
                }
                else if !filling_cave[(current_position.1 + 1 - self.start_offset.1) * self.width + current_position.0 - 1 - self.start_offset.0] {
                    // Move down and left
                    current_position.1 += 1;
                    current_position.0 -= 1;
                }
                else if !filling_cave[(current_position.1 + 1 - self.start_offset.1) * self.width + current_position.0 + 1 - self.start_offset.0] {
                    // Move down and right
                    current_position.1 += 1;
                    current_position.0 += 1;
                } else {
                    // Sand still
                    filling_cave[(current_position.1 - self.start_offset.1) * self.width + current_position.0 - self.start_offset.0] = true;
                    sand_count += 1;
                    break;
                }
            }
        }

        sand_count
    }

    fn p2_cave(&self) -> Cave {
        let p2_width = self.width + 2 * (self.height + 2);
        let p2_height = self.height + 2;
        let p2_offset = (self.start_offset.0 - self.height - 2, self.start_offset.1);

        let mut cave = vec![false; p2_width * p2_height];

        for y in 0..self.height {
            for x in 0..self.width {
                if self.drawing[y * self.width + x] {
                    cave[y * p2_width + x + self.height + 2] = true;
                }
            }
        }

        for x in 0..p2_width {
            cave[(p2_height - 1) * p2_width + x] = true;
        }

        Cave {
            drawing: cave,
            width: p2_width,
            height: p2_height,
            start_offset: p2_offset,
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.drawing[y * self.width + x] { '#' } else { '.' });
            }
            println!();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let mut data =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");

    let cave = Cave::parse(&data);

    println!("part 1: {}", cave.flood());
    println!("part 1: {}", cave.p2_cave().flood());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Cave;

    #[test]
    fn sample() {
        let cave = Cave::parse("498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
");

        cave.print();
        assert_eq!(cave.flood(), 24);
        cave.p2_cave().print();

    }
}