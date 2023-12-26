pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2023/day_20.txt");
    match part {
        1 => solve01(lines),
        // 2 => solve(lines, 50),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    0
}

#[derive(Clone, Copy)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}
#[derive(Clone, Copy)]
enum Pulse {
    Low,
    High,
}
