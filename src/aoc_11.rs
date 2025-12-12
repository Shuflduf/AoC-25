use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../inputs/11.txt");
// const INPUT: &str = r"aaa: you hhh
// you: bbb ccc
// bbb: ddd eee
// ccc: ddd eee fff
// ddd: ggg
// eee: out
// fff: out
// ggg: out
// hhh: ccc fff iii
// iii: out";

type Devices = HashMap<String, Vec<String>>;
fn main() {
    let mut devices: Devices = HashMap::new();
    INPUT
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            devices.insert(
                line[0..3].to_string(),
                line[5..].split(" ").map(|s| s.to_string()).collect(),
            );
        });

    println!("{devices:?}");

    let part_one_solution = count_paths_out(&devices, HashSet::new(), "you");

    println!("Part 1: {part_one_solution}");
}

fn count_paths_out(devices: &Devices, explored: HashSet<&str>, position: &str) -> u32 {
    if position == "out" {
        return 1;
    }

    let available_paths: Vec<&str> = devices[position]
        .iter()
        .map(|s| s.as_str())
        .filter(|target| !explored.contains(target))
        .collect();
    println!("{explored:?} {available_paths:?}");

    let mut paths_out = 0;

    for path in available_paths {
        let mut new_explored = explored.clone();
        new_explored.insert(path);
        paths_out += count_paths_out(devices, new_explored.clone(), path)
    }
    paths_out
}
