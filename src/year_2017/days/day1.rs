use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let line = include_str!("../../../problem_inputs_2017/day_1.txt");
    match part {
        1 => solve01(line),
        2 => solve02(line),
        _ => 1,
    }
}

fn solve01(line: &str) -> usize {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .tuple_windows()
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .sum::<usize>()
        + if line.chars().next().unwrap() == line.chars().last().unwrap() {
            line.chars().next().unwrap().to_digit(10).unwrap() as usize
        } else {
            0
        }
}

fn solve02(line: &str) -> usize {
    0
}
