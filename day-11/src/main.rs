use std::collections::VecDeque;

trait Monkey {
    fn new() -> Self where Self: Sized;
    fn items(&mut self) -> &mut VecDeque<u128>;
    fn operation(&self, worry_level: u128) -> u128;
    fn test(&self, worry_level: u128) -> usize;
}

//<editor-fold desc="Monkey 0">
struct Monkey0 {
    items: VecDeque<u128>,
}

impl Monkey for Monkey0 {
    fn new() -> Self {
        Monkey0 {
            items: VecDeque::from([63, 57]),
        }
    }

    fn items(&mut self) -> &mut VecDeque<u128> {
        &mut self.items
    }

    fn operation(&self, worry_level: u128) -> u128 {
        worry_level * 11
    }

    fn test(&self, worry_level: u128) -> usize {
        if worry_level % 7 == 0 { 6 } else { 2 }
    }
}
//</editor-fold>

//<editor-fold desc="Monkey 1">
struct Monkey1 {
    items: VecDeque<u128>,
}

impl Monkey for Monkey1 {
    fn new() -> Self {
        Monkey1 {
            items: VecDeque::from([
                82,
                66,
                87,
                78,
                77,
                92,
                83]),
        }
    }

    fn items(&mut self) -> &mut VecDeque<u128> {
        &mut self.items
    }

    fn operation(&self, worry_level: u128) -> u128 {
        worry_level + 1
    }

    fn test(&self, worry_level: u128) -> usize {
        if worry_level % 11 == 0 { 5 } else { 0 }
    }
}
//</editor-fold>

//<editor-fold desc="Monkey 2">
struct Monkey2 {
    items: VecDeque<u128>,
}

impl Monkey for Monkey2 {
    fn new() -> Self {
        Monkey2 {
            items: VecDeque::from([
                97,
                53,
                53,
                85,
                58, 54]),
        }
    }

    fn items(&mut self) -> &mut VecDeque<u128> {
        &mut self.items
    }

    fn operation(&self, worry_level: u128) -> u128 {
        worry_level * 7
    }

    fn test(&self, worry_level: u128) -> usize {
        if worry_level % 13 == 0 { 4 } else { 3 }
    }
}
//</editor-fold>

//<editor-fold desc="Monkey 3">
struct Monkey3 {
    items: VecDeque<u128>,
}

impl Monkey for Monkey3 {
    fn new() -> Self {
        Monkey3 {
            items: VecDeque::from([50]),
        }
    }

    fn items(&mut self) -> &mut VecDeque<u128> {
        &mut self.items
    }

    fn operation(&self, worry_level: u128) -> u128 {
        worry_level + 3
    }

    fn test(&self, worry_level: u128) -> usize {
        if worry_level % 3 == 0 { 1 } else { 7 }
    }
}
//</editor-fold>

//<editor-fold desc="Monkey 4">
struct Monkey4 {
    items: VecDeque<u128>,
}

impl Monkey for Monkey4 {
    fn new() -> Self {
        Monkey4 {
            items: VecDeque::from([64, 69, 52, 65, 73]),
        }
    }

    fn items(&mut self) -> &mut VecDeque<u128> {
        &mut self.items
    }

    fn operation(&self, worry_level: u128) -> u128 {
        worry_level + 6
    }

    fn test(&self, worry_level: u128) -> usize {
        if worry_level % 17 == 0 { 3 } else { 7 }
    }
}
//</editor-fold>

//<editor-fold desc="Monkey 5">
struct Monkey5 {
    items: VecDeque<u128>,
}

impl Monkey for Monkey5 {
    fn new() -> Self {
        Monkey5 {
            items: VecDeque::from([57, 91, 65]),
        }
    }

    fn items(&mut self) -> &mut VecDeque<u128> {
        &mut self.items
    }

    fn operation(&self, worry_level: u128) -> u128 {
        worry_level + 5
    }

    fn test(&self, worry_level: u128) -> usize {
        if worry_level % 2 == 0 { 0 } else { 6 }
    }
}
//</editor-fold>

//<editor-fold desc="Monkey 6">
struct Monkey6 {
    items: VecDeque<u128>,
}

impl Monkey for Monkey6 {
    fn new() -> Self {
        Monkey6 {
            items: VecDeque::from([67, 91, 84, 78, 60, 69, 99, 83]),
        }
    }

    fn items(&mut self) -> &mut VecDeque<u128> {
        &mut self.items
    }

    fn operation(&self, worry_level: u128) -> u128 {
        worry_level * worry_level
    }

    fn test(&self, worry_level: u128) -> usize {
        if worry_level % 5 == 0 { 2 } else { 4 }
    }
}
//</editor-fold>

//<editor-fold desc="Monkey 7">
struct Monkey7 {
    items: VecDeque<u128>,
}

impl Monkey for Monkey7 {
    fn new() -> Self {
        Monkey7 {
            items: VecDeque::from([58, 78, 69, 65]),
        }
    }

    fn items(&mut self) -> &mut VecDeque<u128> {
        &mut self.items
    }

    fn operation(&self, worry_level: u128) -> u128 {
        worry_level + 7
    }

    fn test(&self, worry_level: u128) -> usize {
        if worry_level % 19 == 0 { 5 } else { 1 }
    }
}
//</editor-fold>


fn solve_part_1() {
    let mut monkeys: [Box<dyn Monkey>; 8] = [
        Box::new(Monkey0::new()),
        Box::new(Monkey1::new()),
        Box::new(Monkey2::new()),
        Box::new(Monkey3::new()),
        Box::new(Monkey4::new()),
        Box::new(Monkey5::new()),
        Box::new(Monkey6::new()),
        Box::new(Monkey7::new()),
    ];

    let mut inspected = [0; 8];

    for round in 0..20 {
        for monkey in 0..8 {
            loop {
                if monkeys[monkey].items().len() == 0 {
                    break;
                }

                let item = monkeys[monkey].items().pop_front().unwrap();
                inspected[monkey] += 1;

                let mut worry_level = monkeys[monkey].operation(item);
                worry_level /= 3;
                let next_monkey = monkeys[monkey].test(worry_level);
                monkeys[next_monkey].items().push_back(worry_level);
            }
        }
    }

    inspected.sort();
    println!("{}", inspected[6] * inspected[7]);
}

fn solve_part_2() {
    let mut monkeys: [Box<dyn Monkey>; 8] = [
        Box::new(Monkey0::new()),
        Box::new(Monkey1::new()),
        Box::new(Monkey2::new()),
        Box::new(Monkey3::new()),
        Box::new(Monkey4::new()),
        Box::new(Monkey5::new()),
        Box::new(Monkey6::new()),
        Box::new(Monkey7::new()),
    ];

    let mut inspected: [u128; 8] = [0; 8];

    for round in 0..10000 {
        for monkey in 0..8 {
            loop {
                if monkeys[monkey].items().len() == 0 {
                    break;
                }

                let item = monkeys[monkey].items().pop_front().unwrap();
                inspected[monkey] += 1;

                let mut worry_level = monkeys[monkey].operation(item);
                let next_monkey = monkeys[monkey].test(worry_level);
                worry_level = worry_level % (7 * 11 * 13 * 3 * 17 * 2 * 5 * 19);
                monkeys[next_monkey].items().push_back(worry_level);
            }
        }
    }

    inspected.sort();
    println!("{}", inspected[6] * inspected[7]);
}

fn main() {
    solve_part_1();
    solve_part_2();
}
