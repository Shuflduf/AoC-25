use std::fs;

// attempts
// 6165
// 6707
// 6664

pub fn run() {
    let mut times_past_zero = 0;
    let mut position = 50;

    let file_contents = fs::read_to_string("inputs/one.txt").unwrap();
    // let file_contents = "R1000";
    file_contents.lines().for_each(|s| {
        let mut chars = s.chars();
        let direction = if chars.next().unwrap() == 'R' { 1 } else { -1 };
        let amount = chars.collect::<String>().parse::<i32>().unwrap();

        for _ in 0..amount {
            position += direction;
            if position < 0 {
                position += 100;
                times_past_zero += 1
            } else if position >= 100 {
                position -= 100;
                times_past_zero += 1
            }
        }
    });

    println!("Passed zero {times_past_zero} times");
}
