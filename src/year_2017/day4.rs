use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2017/day_4.txt");
    match part {
        1 => solve(lines, part_1),
        2 => solve(lines, part_2),
        _ => 1,
    }
}

fn solve(lines: &str, is_valid: impl Fn(&str) -> usize) -> usize {
    lines
        .lines()
        .filter(|line| is_valid(line) == line.split(' ').count())
        .count()
}

fn part_1(line: &str) -> usize {
    line.split(' ').unique().count()
}

fn part_2(line: &str) -> usize {
    line.split(' ')
        .map(|f| f.chars().sorted_unstable().collect::<String>())
        .unique()
        .count()
}
