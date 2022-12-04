pub struct Rucksack {
    pub all_items: String,
    pub compartment_a: Vec<char>,
    pub compartment_b: Vec<char>,
}

impl Rucksack {
    fn find_duplicated_item(&self) -> char {
        for c in &self.compartment_a {
            if self.compartment_b.contains(c) {
                return *c;
            }
        }
        panic!("Duplicated item not found");
    }
}

fn char_priority(c: char) -> u64 {
    if c.is_lowercase() {
        c as u64 - 'a' as u64 + 1
    } else {
        c as u64 - 'A' as u64 + 27
    }
}

pub fn solve_part_1(rucksacks: &Vec<Rucksack>) -> u64 {
    rucksacks.iter().map(|rucksack| {
        let c = rucksack.find_duplicated_item();
        char_priority(c)
    })
        .sum()
}

pub fn solve_part_2(rucksacks: &Vec<Rucksack>) -> u64 {
    let mut result = 0;

    for i in (0..rucksacks.len()).step_by(3) {
        for c in rucksacks[i].all_items.chars() {
            if rucksacks[i + 1].all_items.contains(c) && rucksacks[i + 2].all_items.contains(c) {
                result += char_priority(c);
                break;
            }
        }
    }

    result
}

pub fn parse_data(data: String) -> Vec<Rucksack> {
    data.lines().map(|line| {
        let line = line.trim();
        let half_size = line.len() / 2;
        Rucksack {
            all_items: line.to_string(),
            compartment_a: line[..half_size].chars().collect(),
            compartment_b: line[half_size..].chars().collect(),
        }
    })
        .collect()
}