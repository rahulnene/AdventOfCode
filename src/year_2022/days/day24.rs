pub fn solution(part: u8) -> isize {
    let lines = include_str!("../../../problem_inputs/day21.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => -1,
    }
}

pub fn solve01(lines: &str) -> isize {
    -1
}

pub fn solve02(lines: &str) -> isize {
    -1
}

fn parse(lines: &str) {}
