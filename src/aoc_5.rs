use std::str::Split;

const INPUT: &str = include_str!("../inputs/5.txt");

#[derive(Debug)]
struct Range {
    lower: u64,
    upper: u64,
}

fn main() {
    let (ranges_str, ingredients) = INPUT.split_once("\n\n").unwrap();
    let range_iter = ranges_str.split("\n");
    let ranges = construct_ranges(range_iter);
    let mut part_one_solution = 0;
    'outer: for ingredient_str in ingredients.split("\n") {
        if ingredient_str.len() < 1 {
            break;
        }
        let ingredient = ingredient_str.parse::<u64>().unwrap();

        for range in &ranges {
            if ingredient >= range.lower && ingredient <= range.upper {
                part_one_solution += 1;
                continue 'outer;
            }
        }
    }

    println!("Part 1: {part_one_solution}");
}

fn construct_ranges(range_iter: Split<'_, &str>) -> Vec<Range> {
    range_iter
        .filter(|range_str| range_str.len() > 0)
        .map(|range_str| range_str.split_once("-").unwrap())
        .map(|(lower_str, upper_str)| {
            (
                lower_str.parse::<u64>().unwrap(),
                upper_str.parse::<u64>().unwrap(),
            )
        })
        .map(|(lower, upper)| Range { lower, upper })
        .collect()
}
