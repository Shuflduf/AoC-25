use std::collections::HashSet;

// const INPUT: &str = include_str!("../inputs/7.txt");
const INPUT: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
............... ";

fn main() {
    let mut current_beams = HashSet::new();
    let mut line_iter = INPUT.split("\n");
    current_beams.insert(line_iter.next().unwrap().find("S").unwrap());

    let mut part_one_solution = 0;
    let mut part_two_solution = 0;

    let mut is_blank_line = false;
    for line in line_iter {
        is_blank_line = !is_blank_line;
        if is_blank_line {
            continue;
        }

        for (i, c) in line.chars().enumerate() {
            if c == '^' {
                if current_beams.contains(&i) {
                    part_two_solution += 2;
                    current_beams.insert(i - 1);
                    current_beams.insert(i + 1);
                    current_beams.remove(&i);
                    part_one_solution += 1;
                }
            }
        }
    }

    println!("{current_beams:?}");
    println!("Part 1: {part_one_solution}");
    println!("Part 2: {part_two_solution}");
}
