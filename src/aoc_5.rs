use std::str::Split;

const INPUT: &str = include_str!("../inputs/5.txt");
// const INPUT: &str = r#"3-5
// 10-14
// 16-20
// 12-18

// 1
// 5
// 8
// 11
// 17
// 32
// "#;

// 301344220818607267 too high
// 375293460537766 too high
// 371029519341542 too high

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
struct Range {
    lower: u64,
    upper: u64,
}
impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.lower.cmp(&other.lower)
    }
}

fn main() {
    let (ranges_str, ingredients) = INPUT.split_once("\n\n").unwrap();
    let range_iter = ranges_str.split("\n");
    let ranges = {
        let mut t = construct_ranges(range_iter);
        t.sort();
        t
    };
    let compressed_ranges = compress_ranges(&ranges);
    dbg!(&compressed_ranges);

    let mut part_one_solution = 0;
    let mut part_two_solution: u64 = 0;

    'outer: for ingredient_str in ingredients.split("\n") {
        if ingredient_str.len() < 1 {
            break;
        }
        let ingredient = ingredient_str.parse::<u64>().unwrap();

        for range in &compressed_ranges {
            if ingredient >= range.lower && ingredient <= range.upper {
                part_one_solution += 1;
                continue 'outer;
            }
        }
    }

    for range in &compressed_ranges {
        part_two_solution += range.upper - range.lower + 1
    }

    println!("Part 1: {part_one_solution}");
    println!("Part 2: {part_two_solution}");
}

fn compress_ranges(uncompressed: &Vec<Range>) -> Vec<Range> {
    let mut compressed = vec![];
    for range in uncompressed {
        add_range(range, &mut compressed);
    }
    compressed.sort_by(|first, second| first.lower.cmp(&second.lower));
    compressed
}

fn add_range(range_to_add: &Range, set: &mut Vec<Range>) {
    for range in &mut *set {
        if range_to_add.lower >= range.lower && range_to_add.lower <= range.upper {
            if range_to_add.upper >= range.upper {
                range.upper = range_to_add.upper;
                return;
            }
        }
        if range_to_add.upper >= range.lower && range_to_add.upper <= range.upper {
            if range_to_add.lower <= range.lower {
                range.lower = range_to_add.lower;
                return;
            }
        }
    }
    set.push(range_to_add.clone());
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
