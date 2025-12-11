use std::collections::HashMap;

// const INPUT: &str =
// r"[..##..] (0,5) (1,2,3,4,5) (1,3,4,5) (3,4) (2,3,5) (0,1,2,5) {29,40,23,42,39,52}";
const INPUT: &str = include_str!("../inputs/10.txt");
// const INPUT: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

#[derive(Debug)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
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
            joltages: line[joltage_start + 1..line.len() - 1]
                .split(",")
                .map(|joltage| joltage.parse().unwrap())
                .collect(),
        }
    }

    fn solve_part_one(&self) -> u32 {
        let mut min_presses = u32::MAX;

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

    fn solve_part_two(&self) -> u32 {
        let mut memo = HashMap::new();
        let res = self.get_min_presses_part_two(vec![0; self.joltages.len()], &mut memo);
        println!("Solved {:?} with {res} presses", self.joltages);
        res
    }

    fn get_min_presses_part_two(&self, state: Vec<u32>, memo: &mut HashMap<Vec<u32>, u32>) -> u32 {
        if state == self.joltages {
            return 0;
        }

        if state.iter().zip(&self.joltages).any(|(s, t)| s > t) {
            return u32::MAX;
        }

        if let Some(&result) = memo.get(&state) {
            return result;
        }

        if memo.len() % 100000 == 0 && memo.len() != 0 {
            println!("Explored {} states, current:  {:?}", memo.len(), state);
        }

        let mut min_presses = u32::MAX;

        for button in &self.buttons {
            let mut next_state = state.clone();

            for &counter_idx in button {
                next_state[counter_idx] += 1;
            }

            let over = next_state.iter().zip(&self.joltages).any(|(s, t)| s > t);

            if !over {
                let result = self.get_min_presses_part_two(next_state, memo);
                if result != u32::MAX {
                    min_presses = min_presses.min(1 + result);
                }
            }
        }

        memo.insert(state, min_presses);
        min_presses
    }
}

fn main() {
    let machines = INPUT
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| Machine::new(line))
        .collect::<Vec<Machine>>();
    let part_one_solution: u32 = machines.iter().map(|mach| mach.solve_part_one()).sum();
    let part_two_solution: u32 = machines.iter().map(|mach| mach.solve_part_two()).sum();

    println!("Part 1: {part_one_solution}");
    println!("Part 2: {part_two_solution}");
}
