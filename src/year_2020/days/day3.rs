use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_3.txt");
    match part {
        1 => solve(lines, 1, 3),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve(lines: &str, down: u8, right: u8) -> usize {
    let mut count = 0;
    let mut col = 0;
    for line in lines.lines().step_by(down as usize) {
        let width = line.len();
        let row: Vec<bool> = line.chars().map(|c| (c == '#')).collect_vec();
        count += row[col % width] as usize;
        col += right as usize;
    }
    count
}

fn solve02(lines: &str) -> usize {
    solve(lines, 2, 1)
        * solve(lines, 1, 1)
        * solve(lines, 1, 3)
        * solve(lines, 1, 5)
        * solve(lines, 1, 7)
}
