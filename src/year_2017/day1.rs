pub fn solution(part: u8) -> usize {
    let line = include_str!("../../../problem_inputs_2017/day_1.txt");
    match part {
        1 => solve01(line),
        2 => solve02(line),
        _ => 1,
    }
}

fn solve01(line: &str) -> usize {
    let digits = line.chars().collect::<Vec<_>>();

    digits
        .iter()
        .zip(digits.iter().cycle().skip(1))
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a.to_digit(10).unwrap() as usize)
        .sum()
}

fn solve02(line: &str) -> usize {
    line.chars()
        .cycle()
        .skip(line.len() / 2)
        .take(line.len())
        .zip(line.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a.to_digit(10).unwrap() as usize)
        .sum()
}
