use std::cmp::{max, min};

const INPUT: &str = include_str!("../inputs/9.txt");

fn main() {
    let mut part_one_solution = 0;
    let mut part_two_solution = 0;
    let red_tiles: Vec<(i64, i64)> = INPUT
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(",");
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let green_tiles = build_polygon(&red_tiles);

    for i in 0..red_tiles.len() {
        let tile_a = red_tiles[i];
        'outer: for j in i..red_tiles.len() {
            let tile_b = red_tiles[j];
            let area = get_area(&tile_a, &tile_b);
            if area > part_one_solution {
                part_one_solution = area;
            }
            if area > part_two_solution {
                for test_point in &green_tiles {
                    if point_inside(&tile_a, &tile_b, test_point) {
                        continue 'outer;
                    }
                }
                println!("{tile_a:?} {tile_b:?}");
                part_two_solution = area;
            }
        }
    }

    println!("Part 1: {part_one_solution}");
    println!("Part 2: {part_two_solution}");
}

fn get_area(point_a: &(i64, i64), point_b: &(i64, i64)) -> i64 {
    return ((point_a.0 - point_b.0).abs() + 1) * ((point_a.1 - point_b.1).abs() + 1);
}

fn build_polygon(points: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut green_tiles = vec![];
    let mut last = points[0];
    for i in 1..points.len() {
        let next = points[i];
        green_tiles.extend(get_points_between(&last, &next));
        last = next;
    }
    let next = points[0];
    green_tiles.extend(get_points_between(&last, &next));
    green_tiles.dedup();
    green_tiles
}

fn get_points_between(point_a: &(i64, i64), point_b: &(i64, i64)) -> Vec<(i64, i64)> {
    let top_left = (min(point_a.0, point_b.0), min(point_a.1, point_b.1));
    let bottom_right = (max(point_a.0, point_b.0), max(point_a.1, point_b.1));
    let horizontal = point_a.1 == point_b.1;
    let mut points = vec![];
    if horizontal {
        for x in top_left.0..bottom_right.0 {
            points.push((x, top_left.1));
        }
    } else {
        for y in top_left.1..=bottom_right.1 {
            points.push((top_left.0, y));
        }
    }
    points
}

fn point_inside(point_a: &(i64, i64), point_b: &(i64, i64), test_point: &(i64, i64)) -> bool {
    let top_left = (min(point_a.0, point_b.0), min(point_a.1, point_b.1));
    let bottom_right = (max(point_a.0, point_b.0), max(point_a.1, point_b.1));
    return test_point.0 > top_left.0
        && test_point.0 < bottom_right.0
        && test_point.1 > top_left.1
        && test_point.1 < bottom_right.1;
}
