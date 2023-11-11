use regex::Regex;
pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_17_test.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    0
}
fn solve02(lines: &str) -> usize {
    0
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
    active: bool,
}

struct Reactor {
    cubes: Vec<Cube>,
}
