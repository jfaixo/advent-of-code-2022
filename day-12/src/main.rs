use std::{env, fs};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::process::exit;

#[derive(Clone)]
struct HeightMap {
    data: Vec<i32>,
    height: usize,
    width: usize,
    start_location: (usize, usize),
    target_location: (usize, usize),
}

fn parse_data(data: String) -> HeightMap {
    let mut height_map = HeightMap {
        data: vec![],
        height: data.lines().count(),
        width: 0,
        start_location: (0, 0),
        target_location: (0, 0),
    };

    for (y, mut line) in data.lines().enumerate() {
        line = line.trim();
        height_map.width = line.len();
        line.chars().enumerate().for_each(|(x, c)| {
            if c == 'S' {
                height_map.start_location = (x, y);
                height_map.data.push(0)
            } else if c == 'E' {
                height_map.target_location = (x, y);
                height_map.data.push(25)
            } else {
                height_map.data.push(c as i32 - 'a' as i32)
            }
        })
    }

    height_map
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let data =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");

    let height_map = parse_data(data);
    println!("part 1: {}", solve_part_1(&height_map, height_map.start_location));
    println!("part 2: {}", solve_part_2(&height_map));

    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_part_1(height_map: &HeightMap, start_location: (usize, usize)) -> i32 {
    let mut dist: Vec<_> = (0..height_map.width * height_map.height).map(|_| i32::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start_location.1 * height_map.width + start_location.0] = 0;
    heap.push(State { cost: 0, position: start_location });

    while let Some(State { cost, position }) = heap.pop() {
        if position == height_map.target_location {
            return cost;
        }

        if cost > dist[position.1 * height_map.width + position.0] { continue; }

        let current_height = height_map.data[position.1 * height_map.width + position.0];

        if position.0 > 0 && (height_map.data[position.1 * height_map.width + position.0 - 1] - current_height) < 2 {
            let next = State { cost: cost + 1, position: (position.0 - 1, position.1) };
            // If so, add it to the frontier and continue
            if next.cost < dist[next.position.1 * height_map.width + next.position.0] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position.1 * height_map.width + next.position.0] = next.cost;
            }
        }

        if position.0 < height_map.width - 1 && (height_map.data[position.1 * height_map.width + position.0 + 1] - current_height) < 2 {
            let next = State { cost: cost + 1, position: (position.0 + 1, position.1) };
            // If so, add it to the frontier and continue
            if next.cost < dist[next.position.1 * height_map.width + next.position.0] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position.1 * height_map.width + next.position.0] = next.cost;
            }
        }

        if position.1 > 0 && (height_map.data[(position.1 - 1) * height_map.width + position.0] - current_height) < 2 {
            let next = State { cost: cost + 1, position: (position.0, position.1 - 1) };
            // If so, add it to the frontier and continue
            if next.cost < dist[next.position.1 * height_map.width + next.position.0] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position.1 * height_map.width + next.position.0] = next.cost;
            }
        }

        if position.1 < height_map.height - 1 && (height_map.data[(position.1 + 1) * height_map.width + position.0] - current_height) < 2 {
            let next = State { cost: cost + 1, position: (position.0, position.1 + 1) };
            // If so, add it to the frontier and continue
            if next.cost < dist[next.position.1 * height_map.width + next.position.0] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position.1 * height_map.width + next.position.0] = next.cost;
            }
        }
    }

    i32::MAX
}

fn solve_part_2(height_map: &HeightMap) -> i32 {
    let mut global_min = i32::MAX;

    for y in 0..height_map.height {
        for x in 0..height_map.width {
            if height_map.data[y * height_map.width + x] == 0 {
                let result = solve_part_1(height_map, (x, y));
                if result < global_min {
                    global_min = result;
                }
            }
        }
    }

    global_min
}