use std::{env, fs};
use std::cmp::Ordering;
use std::error::Error;
use std::ops::Index;
use std::process::exit;

#[derive(PartialEq, Clone)]
enum Packet {
    Value(i32),
    SubPacket(Vec<usize>),
}

#[derive(Default, Clone)]
struct Packets {
    packets: Vec<Packet>,
    packet_pairs: Vec<(usize, usize)>,
}

fn parse_data(data: &str) -> Packets {
    let mut packets = Packets::default();
    let lines = data.lines().filter(|&line| !line.is_empty()).collect::<Vec<_>>();

    for l in (0..lines.len()).step_by(2) {
        let mut c = 0;
        let packet_1 = parse_line(&mut packets, lines[l], &mut c);
        c = 0;
        let packet_2 = parse_line(&mut packets, lines[l + 1], &mut c);
        packets.packet_pairs.push((packet_1.unwrap(), packet_2.unwrap()));
    }

    packets
}

fn parse_line(packets: &mut Packets, line: &str, c: &mut usize) -> Option<usize> {
    if line.chars().nth(*c) == Some(',') {
        None
    }
    else if line.chars().nth(*c) == Some('[') {
        let mut sub_packets = Vec::new();
        loop {
            *c += 1;
            if line.chars().nth(*c) == Some(']') {
                break;
            }
            let p = parse_line(packets, line, c);
            match p {
                None => {}
                Some(p) => {
                    sub_packets.push(p);
                }
            }
        }

        packets.packets.push(Packet::SubPacket(sub_packets));
        Some(packets.packets.len() - 1)
    }
    else {
        let mut v = 0;
        loop {
            let ch= line.chars().nth(*c).unwrap();
            if ch.is_numeric() {
                let d = ch as i32 - '0' as i32;
                v = v * 10 + d;
                *c += 1;
            }
            else {
                break;
            }
        }
        *c -= 1;

        packets.packets.push(Packet::Value(v));
        Some(packets.packets.len() - 1)
    }
}

fn solve_part_1(packets: &mut Packets) -> usize {
    let mut result = 0;
    for i in 0..packets.packet_pairs.len() {
        let ordering = compare(packets, packets.packet_pairs[i].0,packets.packet_pairs[i].1);
        if ordering != Ordering::Greater {
            result += i + 1;
        }
    }

    result
}

fn compare(packets: &mut Packets, a: usize, b: usize) -> Ordering {
    let p_a = packets.packets[a].clone();
    let p_b = packets.packets[b].clone();

    let a_is_value = std::mem::discriminant(&p_a) == std::mem::discriminant(&Packet::Value(0));
    let b_is_value = std::mem::discriminant(&p_b) == std::mem::discriminant(&Packet::Value(0));

    if a_is_value && b_is_value {
        let v_a = match p_a {
            Packet::Value(v) => { v }
            Packet::SubPacket(_) => { panic!("") }
        };
        let v_b = match p_b {
            Packet::Value(v) => { v }
            Packet::SubPacket(_) => { panic!("") }
        };
        v_a.cmp(&v_b)

    } else {
        if a_is_value {
            let v_a = match p_a {
                Packet::Value(v) => { v }
                Packet::SubPacket(_) => { panic!("") }
            };
            packets.packets.push(Packet::Value(v_a));
            packets.packets[a] = Packet::SubPacket(vec![packets.packets.len() - 1]);
        }
        if b_is_value {
            let v_b = match p_b {
                Packet::Value(v) => { v }
                Packet::SubPacket(_) => { panic!("") }
            };
            packets.packets.push(Packet::Value(v_b));
            packets.packets[b] = Packet::SubPacket(vec![packets.packets.len() - 1]);
        }

        let sp_a = match &packets.packets[a] {
            Packet::Value(_) => { panic!() }
            Packet::SubPacket(sp) => { sp.clone() }
        };
        let sp_b = match &packets.packets[b] {
            Packet::Value(_) => { panic!() }
            Packet::SubPacket(sp) => { sp.clone() }
        };

        for i in 0..sp_a.len() {
            if i >= sp_b.len() {
                return Ordering::Greater;
            }

            let ordering = compare(packets, sp_a[i], sp_b[i]);
            if ordering != Ordering::Equal {
                return ordering;
            }
        }

        if sp_a.len() < sp_b.len() {
            return Ordering::Less;
        }

        return Ordering::Equal
    }
}

fn solve_part_2(packets: &mut Packets) -> usize {
    let mut all_packets = packets.packet_pairs.iter().flat_map(|p| [p.0, p.1]).collect::<Vec<_>>();

    all_packets.sort_by(|a, b| {
        let mut packets = packets.clone();
        compare(&mut packets, *a, *b)
    });

    let last_pair = packets.packet_pairs[packets.packet_pairs.len() - 1];

    let m1 = all_packets.iter().position(|m| *m == last_pair.0).unwrap();
    let m2 = all_packets.iter().position(|m| *m == last_pair.1).unwrap();

    (m1 + 1) * (m2 + 1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let mut data =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");

    let mut packets = parse_data(&data);
    println!("part 1: {}", solve_part_1(&mut packets));

    data.extend("\
    \
    [[2]]
[[6]]
".chars());
    let mut packets = parse_data(&data);

    println!("part 2: {}", solve_part_2(&mut packets.clone()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_data, solve_part_1};

    #[test]
    fn sample_1() {
        let mut packets = parse_data("[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
".to_string());
        assert_eq!(solve_part_1(&mut packets), 13);
    }

    #[test]
    fn sample_2() {
        let mut packets = parse_data("[[],[1],[[[1,3],2,1,3]]]
[[[],6,[3,8]],[]]
".to_string());
        assert_eq!(solve_part_1(&mut packets), 1);
    }

    #[test]
    fn sample_3() {
        let mut packets = parse_data("[[1,[[7,6,3,4],9,[]]],[6],[],[[10],[3,[7,9],[8,0,1,6,7],3,[7,8,4,5]],3],[[4,[8,1,0,7],6]]]
[[[1]],[4,5,2,[0]]]
".to_string());
        assert_eq!(solve_part_1(&mut packets), 0);
    }

    #[test]
    fn sample_4() {
        let mut packets = parse_data("[[9,[0,[1,10,9,8]]],[[3]]]
[[],[[[],[5,3,3],[]]]]
".to_string());
        assert_eq!(solve_part_1(&mut packets), 0);
    }

    #[test]
    fn sample_5() {
        let mut packets = parse_data("[[1,9,[5,5,[0,5,0],[0,7,1,3,1]],3],[7,[],1,[4,7,6]],[[2,2,2,7,[8]],[[9],[5,9,5,2],6,7],[[7,3,10,1],2,0,[1,3],5],6,6]]
[[],[[[5,2],3,[],1,10],1,9,[3,6,4,[],[4,1,2,7]]],[],[[[9,9,10,2,5],[3]],[8,1,[6,0]]],[[],[],[2,[5,2,6,6],[4,9,6,3,9],2],6]]

".to_string());
        assert_eq!(solve_part_1(&mut packets), 0);
    }

    #[test]
    fn sample_6() {
        let mut packets = parse_data("[[2,[8]],[[],0,7,7],[7,[[],[6,9,7],0],5],[8,[5,[1,1,2],[1,0,6,1,0],2,[4]],6,[7,7],[10,[9,6,8,1,7],[]]],[1]]
[[4,10,[],[[9],[8]],[9,[10,3,4,4,3]]],[6,[[2,9]],[],8,4],[[6,[],7,10]]]

".to_string());
        assert_eq!(solve_part_1(&mut packets), 1);
    }

    #[test]
    fn sample_7() {
        let mut packets = parse_data("[[[[10,9,9,1],5,7,3,[3,10,3]],5,10,5,0]]
[[1,[[],4,[],5],3]]

".to_string());
        assert_eq!(solve_part_1(&mut packets), 0);
    }

    #[test]
    fn sample_8() {
        let mut packets = parse_data("[[1,[[7,6,3,4],9,[]]],[6],[],[[10],[3,[7,9],[8,0,1,6,7],3,[7,8,4,5]],3],[[4,[8,1,0,7],6]]]
[[[1]],[4,5,2,[0]]]
".to_string());
        assert_eq!(solve_part_1(&mut packets), 0);
    }
}