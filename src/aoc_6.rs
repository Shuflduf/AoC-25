use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/6.txt");

fn main() {
    let rows_iter = INPUT.split("\n");
    let mut number_lists: Vec<Vec<u64>> = vec![];
    let mut instructions: Vec<char> = vec![];

    let mut part_one_solution: u64 = 0;
    let mut part_two_solution: u64 = 0;

    let mut spaces: HashSet<usize> = HashSet::new();
    let mut row_length: usize = 0;
    for row in rows_iter.clone() {
        if row_length == 0 {
            row_length = row.len();
        }
        for (i, token) in row.split_whitespace().enumerate() {
            if token == "*" || token == "+" {
                instructions.push(token.chars().next().unwrap());
            } else {
                if spaces.is_empty() {
                    spaces = get_spaces_set(row);
                } else {
                    spaces = spaces.intersection(&get_spaces_set(row)).cloned().collect();
                }
                if i >= number_lists.len() {
                    number_lists.push(vec![]);
                }
                number_lists[i].push(token.parse::<u64>().unwrap())
            }
        }
    }

    for (i, instruction) in instructions.iter().enumerate() {
        if instruction == &'+' {
            part_one_solution += number_lists[i].iter().sum::<u64>();
        } else {
            part_one_solution += number_lists[i].iter().product::<u64>();
        }
    }

    // number_lists = vec![];
    let mut str_number_lists: Vec<Vec<String>> = vec![];

    let mut spaces: Vec<usize> = spaces.into_iter().collect();
    spaces.sort();
    spaces.push(row_length);

    'outer: for row in rows_iter {
        let mut offset = 0;
        let mut remaining_row = row;
        for (problem_index, space_index) in spaces.iter().enumerate() {
            let split_index = space_index - offset;
            let (new_num, new_row) = remaining_row.split_at(split_index);
            if new_num.contains("*") || new_num.contains("+") {
                break 'outer;
            }
            let new_num = new_num.replace(" ", "x");
            remaining_row = new_row;
            offset = *space_index;
            if problem_index >= str_number_lists.len() {
                str_number_lists.push(vec![]);
            }
            str_number_lists[problem_index].push(new_num.to_string())
        }
    }

    for (problem_index, instruction) in instructions.iter().enumerate() {
        let digit_count = str_number_lists[problem_index][0].len();
        let mut problem_solution: u64 = 0;
        for digit_index in 0..digit_count {
            let mut new_num: u64 = 0;
            for num in &str_number_lists[problem_index] {
                let target_digit = num.chars().nth(digit_index).unwrap();
                if !(target_digit == 'x') {
                    new_num *= 10;
                    new_num += target_digit.to_string().parse::<u64>().unwrap();
                }
            }
            if instruction == &'+' {
                problem_solution += new_num;
            } else {
                if problem_solution == 0 {
                    problem_solution = 1;
                }
                problem_solution *= if new_num != 0 { new_num } else { 1 };
            }
        }
        part_two_solution += problem_solution
    }

    println!("Part 1: {part_one_solution}");
    println!("Part 2: {part_two_solution}");
}

fn get_spaces_set(line: &str) -> HashSet<usize> {
    let mut set = HashSet::new();
    line.char_indices()
        .filter(|(_, could_be_space)| could_be_space == &' ')
        .map(|(i, _)| i)
        .for_each(|i| {
            set.insert(i);
        });
    set
}
