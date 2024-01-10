use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let given = "1113122113";
    (solve(&given, 40), solve(&given, 50))
}

fn solve(given: &str, cycles: usize) -> (usize, Duration) {
    let now = Instant::now();
    let mut given = given.to_owned();
    for _ in 0..cycles {
        given = look_and_say(&given);
    }
    (given.len(), now.elapsed())
}

fn look_and_say(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        let mut count = 1;
        while let Some(&next) = chars.peek() {
            if next == c {
                count += 1;
                chars.next();
            } else {
                break;
            }
        }
        output.push_str(&count.to_string());
        output.push(c);
    }
    output
}
