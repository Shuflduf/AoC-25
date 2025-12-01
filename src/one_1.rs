use std::fs;

pub fn run() {
    let mut position = 50;

    let file_contents = fs::read_to_string("inputs/one_1.txt").unwrap();
    file_contents.lines().for_each(|s| {
        let mut chars = s.chars();
        let direction = chars.next();
        let amount = chars.collect::<String>().parse::<i32>().unwrap();
        println!("{amount:?}");
    });

    println!("{position}")
}
