use std::fs;

pub fn run() {
    let mut times_at_zero = 0;
    let mut times_past_zero = 0;
    let mut position = 50;

    let file_contents = fs::read_to_string("inputs/one.txt").unwrap();
    file_contents.lines().for_each(|s| {
        let mut chars = s.chars();
        let direction = chars.next().unwrap();
        let amount = chars.collect::<String>().parse::<i32>().unwrap();

        if direction == 'R' {
            times_past_zero += handle_rotation(&mut position, amount);
        } else {
            times_past_zero += handle_rotation(&mut position, -amount);
        }

        if position == 0 {
            times_at_zero += 1;
            times_past_zero += 1;
        }
    });

    println!("Arrived at zero {times_at_zero} times");
    println!("Passed zero {times_past_zero} times");
}

fn handle_rotation(current_rotation: &mut i32, difference: i32) -> i32 {
    let mut times_past_zero = 0;
    *current_rotation += difference;
    while *current_rotation >= 100 {
        times_past_zero += 1;
        *current_rotation -= 100;
    }
    while *current_rotation < 0 {
        times_past_zero += 1;
        *current_rotation += 100;
    }
    times_past_zero
}
