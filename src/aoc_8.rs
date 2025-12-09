use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/8.txt");
const CONNECTION_COUNT: usize = 1000;

fn main() {
    let mut part_one_solution = 0;
    let mut part_two_solution = 0;

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

    let mut circuits: Vec<usize> = (0..box_count).collect();
    let mut connections_left = CONNECTION_COUNT;

    for (_, box_a, box_b) in edges {
        let root_a = find_root(&circuits, box_a);
        let root_b = find_root(&circuits, box_b);

        if root_a != root_b {
            for i in 0..circuits.len() {
                if circuits[i] == root_b {
                    circuits[i] = root_a;
                }
            }
        }

        if connections_left > 0 {
            connections_left -= 1;
        }
        if connections_left == 0 && part_one_solution == 0 {
            let mut sizes = get_sizes(&circuits);
            sizes.sort_unstable_by(|a, b| b.cmp(a));

            part_one_solution = sizes.iter().take(3).product();
        }
        if all_connected(&circuits) {
            part_two_solution = boxes[box_a].0 * boxes[box_b].0;
            break;
        }
    }

    println!("Part 1: {part_one_solution}");
    println!("Part 2: {part_two_solution}");
}

fn distance_squared(first: &(i64, i64, i64), second: &(i64, i64, i64)) -> i64 {
    return (second.0 - first.0).pow(2) + (second.1 - first.1).pow(2) + (second.2 - first.2).pow(2);
}

fn get_sizes(circuits: &[usize]) -> Vec<u32> {
    let mut sizes = HashMap::new();
    for root_id in circuits.iter() {
        *sizes.entry(root_id).or_insert(0) += 1;
    }
    sizes.values().copied().collect()
}

fn find_root(circuits: &[usize], box_id: usize) -> usize {
    if circuits[box_id] == box_id {
        box_id
    } else {
        find_root(circuits, circuits[box_id])
    }
}

fn all_connected(circuits: &[usize]) -> bool {
    let first = circuits[0];
    circuits.iter().all(|v| *v == first)
}
