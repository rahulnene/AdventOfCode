use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_10.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut adapters: Vec<usize> = lines
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    adapters.sort_unstable();
    let diffs = adapters
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<usize>>();

    (diffs.iter().filter(|&&x| x == 1).count() + 1)
        * (diffs.iter().filter(|&&x| x == 3).count() + 1)
}

fn solve02(lines: &str) -> usize {
    0
}
