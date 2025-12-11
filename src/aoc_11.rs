use std::collections::HashMap;

// const INPUT: &str = include_str!("../inputs/11.txt");
const INPUT: &str = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

fn main() {
    let mut devices: HashMap<String, Vec<String>> = HashMap::new();
    INPUT.split("\n").for_each(|line| {
        devices.insert(
            line[0..3].to_string(),
            line[5..].split(" ").map(|s| s.to_string()).collect(),
        );
    });
}
