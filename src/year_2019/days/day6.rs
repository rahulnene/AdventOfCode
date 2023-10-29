use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_6.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    lines
        .split("\n\n")
        .map(|g| g.chars().unique().filter(|f| f.is_alphabetic()).count())
        .sum()
}

fn solve02(lines: &str) -> usize {
    lines
        .split("\n\n")
        .map(|group| {
            (97..123_u8)
                .filter(|&digit_code| {
                    group
                        .lines()
                        .collect_vec()
                        .iter()
                        .all(|line| line.contains(digit_code as char))
                })
                .count()
        })
        .sum()
}
