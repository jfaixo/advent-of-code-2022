use std::{env, fs};
use std::error::Error;
use std::process::exit;

#[derive(Default, Debug)]
struct FileSystem {
    nodes: Vec<FSNode>
}

#[derive(Debug)]
enum NodeData {
    File { size: usize },
    Directory { childs: Vec<usize> },
}

#[derive(Debug)]
struct FSNode {
    parent_node: usize,
    name: String,
    node_data: NodeData,
}

fn parse_data(data: String) -> FileSystem {
    let mut filesystem = FileSystem::default();
    filesystem.nodes.push(FSNode {
        parent_node: usize::MAX,
        name: "/".to_string(),
        node_data: NodeData::Directory { childs: vec![] },
    });
    let mut current_node = 0;

    let mut parsing_ls = false;
    let mut list : Vec<&str> = Vec::new();
    for line in data.lines() {
        let line = line.trim();

        if parsing_ls && &line[0..1] != "$" {
            list.push(line);
        }
        else {
            parsing_ls = false;

            for &l in &list {
                if &l[0..1] == "d" {
                    // Parse dir
                    let mut found = false;
                    match &filesystem.nodes[current_node].node_data {
                        NodeData::File { .. } => {}
                        NodeData::Directory { childs } => {
                            // Check if it already exists
                            for child_id in childs {
                                if filesystem.nodes[*child_id].name == &l[4..] {
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !found {
                        filesystem.nodes.push(FSNode {
                            parent_node: current_node,
                            name: l[4..].to_string(),
                            node_data: NodeData::Directory { childs: vec![] },
                        });

                        let new_id = filesystem.nodes.len() - 1;
                        match &mut filesystem.nodes[current_node].node_data {
                            NodeData::File { .. } => {}
                            NodeData::Directory { childs } => { childs.push(new_id); }
                        }
                    }
                }
                else {
                    // Parse file
                    let parts = l.split_ascii_whitespace().collect::<Vec<_>>();
                    let filename = parts[1].to_string();
                    let filesize = parts[0].parse().unwrap();

                    let mut found = false;
                    match &filesystem.nodes[current_node].node_data {
                        NodeData::File { .. } => {}
                        NodeData::Directory { childs } => {
                            // Check if it already exists
                            for child_id in childs {
                                if filesystem.nodes[*child_id].name == filename {
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !found {
                        filesystem.nodes.push(FSNode {
                            parent_node: current_node,
                            name: filename,
                            node_data: NodeData::File { size: filesize },
                        });

                        let new_id = filesystem.nodes.len() - 1;
                        match &mut filesystem.nodes[current_node].node_data {
                            NodeData::File { .. } => {}
                            NodeData::Directory { childs } => { childs.push(new_id); }
                        }
                    }
                }
            }
            list.clear();
        }

        match &line[2..4] {
            "cd" => {
                match &line[5..] {
                    ".." => {
                        current_node = filesystem.nodes[current_node].parent_node;
                    }
                    "/" => {
                        current_node = 0;
                    }
                    _ => {
                        let mut found = false;
                        match &filesystem.nodes[current_node].node_data {
                            NodeData::File { .. } => {}
                            NodeData::Directory { childs } => {
                                for child_id in childs {
                                    if filesystem.nodes[*child_id].name == &line[5..] {
                                        current_node = *child_id;
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }
                        if !found {
                            filesystem.nodes.push(FSNode {
                                parent_node: current_node,
                                name: line[5..].to_string(),
                                node_data: NodeData::Directory { childs: vec![] },
                            });

                            let new_id = filesystem.nodes.len() - 1;
                            match &mut filesystem.nodes[current_node].node_data {
                                NodeData::File { .. } => {}
                                NodeData::Directory { childs } => { childs.push(new_id); }
                            }
                            current_node = new_id;
                        }
                    }
                }
            }
            "ls" => {
                parsing_ls = true;
            }
            _ => {}
        }
    }

    filesystem
}

fn get_directory_size(filesystem: &FileSystem, i: usize) -> usize {
    match &filesystem.nodes[i].node_data {
        NodeData::File { size } => { *size }
        NodeData::Directory { childs } => {
            let mut s = 0;
            for c in childs {
                s += get_directory_size(filesystem, *c);
            }
            s
        }
    }
}

fn solve_part_1(filesystem: &FileSystem) -> usize {
    let mut sum = 0;

    for n in 1..filesystem.nodes.len() {
        match filesystem.nodes[n].node_data {
            NodeData::File { .. } => {}
            NodeData::Directory { .. } => {
                let dir_size = get_directory_size(filesystem, n);
                if dir_size <= 100000 {
                    sum += dir_size;
                }
            }
        }
    }

    sum
}

fn solve_part_2(filesystem: &FileSystem) -> usize {
    let used_memory = get_directory_size(filesystem, 0);
    let available = 70000000 - used_memory;
    let to_free = 30000000 - available;

    let mut smallest_to_delete = usize::MAX;
    for n in 1..filesystem.nodes.len() {
        match filesystem.nodes[n].node_data {
            NodeData::File { .. } => {}
            NodeData::Directory { .. } => {
                let dir_size = get_directory_size(filesystem, n);
                if dir_size >= to_free && dir_size < smallest_to_delete {
                    smallest_to_delete = dir_size;
                }
            }
        }
    }

    smallest_to_delete
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let data =
        fs::read_to_string(args[1].clone()).expect("Error while reading the data file");

    let filesystem = parse_data(data);

    println!("part 1: {}", solve_part_1(&filesystem));
    println!("part 2: {}", solve_part_2(&filesystem));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_data;

    #[test]
    fn sample_1() {
        eprintln!("{:?}", parse_data("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
".to_string()));
    }
}