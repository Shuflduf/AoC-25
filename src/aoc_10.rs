const INPUT: &str = include_str!("../inputs/10.txt");
// const INPUT: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

#[derive(Debug)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}
impl Machine {
    fn new(line: &str) -> Machine {
        let target_end = line.find("]").unwrap();
        let joltage_start = line.find("{").unwrap();

        Machine {
            target: line[1..target_end].chars().map(|c| c == '#').collect(),
            buttons: line[target_end + 2..joltage_start - 1]
                .split(" ")
                .map(|button| {
                    button[1..button.len() - 1]
                        .split(",")
                        .map(|indic_index| indic_index.parse().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    fn solve(&self) -> u32 {
        let mut min_presses = u32::MAX;

        // i love bitshifting
        for mask in 0..(1 << self.buttons.len()) {
            let mut lights = vec![false; self.target.len()];

            for button_idx in 0..self.buttons.len() {
                if (mask >> button_idx) & 1 == 1 {
                    for &light in &self.buttons[button_idx] {
                        lights[light] = !lights[light];
                    }
                }
            }

            if lights == self.target {
                let presses = (mask as u32).count_ones();
                min_presses = min_presses.min(presses);
            }
        }

        min_presses
    }
}

fn main() {
    let machines = INPUT
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| Machine::new(line))
        .collect::<Vec<Machine>>();
    let part_one_solution: u32 = machines.iter().map(|mach| mach.solve()).sum();

    println!("Part 1: {part_one_solution}")
}
