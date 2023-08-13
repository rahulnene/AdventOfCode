pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_17.txt");
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
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Probe {
    loc: Coords,
    vx: usize,
    vy: usize,
}
