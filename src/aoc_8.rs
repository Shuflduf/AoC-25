use std::collections::HashMap;

// 1000 too low

// const INPUT: &str = include_str!("../inputs/8.txt");
const INPUT: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
const CONNECTION_COUNT: usize = 10;

// https://en.wikipedia.org/wiki/Disjoint-set_data_structure
struct Circuits {
    roots: Vec<usize>,
    sizes: Vec<usize>,
}

impl Circuits {
    // start each box as its own curcuit
    fn new(size: usize) -> Self {
        Circuits {
            roots: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    fn find_root(&mut self, box_id: usize) -> usize {
        if self.roots[box_id] != box_id {
            self.roots[box_id] = self.find_root(self.roots[box_id]);
        }
        self.roots[box_id]
    }

    fn connect(&mut self, box_a: usize, box_b: usize) -> bool {
        let root_a = self.find_root(box_a);
        let root_b = self.find_root(box_b);

        if root_a == root_b {
            return false;
        }

        // attach smaller to larger
        if self.sizes[root_a] < self.sizes[root_b] {
            self.roots[root_a] = root_b;
            self.sizes[root_b] += self.sizes[root_a];
        } else {
            self.roots[root_b] = root_a;
            self.sizes[root_a] += self.sizes[root_b];
        }
        true
    }

    fn get_sizes(&mut self) -> Vec<usize> {
        let mut sizes = HashMap::new();
        for i in 0..self.roots.len() {
            let root = self.find_root(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.values().copied().collect()
    }
}

fn main() {
    let boxes: Vec<(i64, i64, i64)> = INPUT
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (parts[0], parts[1], parts[2])
        })
        .collect();

    let box_count = boxes.len();

    let mut edges = vec![];
    for i in 0..box_count {
        for j in i + 1..box_count {
            let dist = distance_squared(&boxes[i], &boxes[j]);
            edges.push((dist, i, j));
        }
    }

    edges.sort_unstable_by_key(|&(dist, _, _)| dist);

    let mut circuits = Circuits::new(box_count);
    let mut connections_left = CONNECTION_COUNT;

    for (_, box_a, box_b) in edges {
        if circuits.connect(box_a, box_b) {
            println!("connected {:?} to {:?}", boxes[box_a], boxes[box_b]);

            connections_left -= 1;
            if connections_left <= 0 {
                break;
            }
        }
    }

    let mut sizes = circuits.get_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    // sizes.sort_unstable();

    println!("{sizes:?}");

    // println!("Made {} successful connections", 1000 - connections_left);
    // println!("Top 3 circuit sizes: {:? }", &sizes[..3]);

    let part_one_solution: usize = sizes.iter().take(3).product();

    // println!("{boxes:?}");
    println!("Part 1: {part_one_solution}");
}

fn distance_squared(first: &(i64, i64, i64), second: &(i64, i64, i64)) -> i64 {
    return (second.0 - first.0).pow(2) + (second.1 - first.1).pow(2) + (second.2 - first.2).pow(2);
}
