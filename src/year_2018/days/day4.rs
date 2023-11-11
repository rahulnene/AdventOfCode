use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2019/day_4.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let (a, b) = lines
        .lines()
        .next()
        .unwrap()
        .split("-")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    (a..=b).filter(|&n| is_valid01(n)).count()
}

fn solve02(lines: &str) -> usize {
    let (a, b) = lines
        .lines()
        .next()
        .unwrap()
        .split("-")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    (a..=b)
        .filter(|&n| is_valid02(n))
        .for_each(|n| println!("{}", n));
    (a..=b).filter(|&n| is_valid02(n)).count()
}

fn is_valid01(number: usize) -> bool {
    let digits = number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    if digits.len() != 6 {
        return false;
    }
    if digits.iter().tuple_windows().any(|(a, b)| a > b) {
        return false;
    }
    if digits.iter().tuple_windows().all(|(a, b)| a != b) {
        return false;
    }

    true
}

fn is_valid02(number: usize) -> bool {
    let digits = number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    if digits.len() != 6 {
        return false;
    }
    if digits.iter().tuple_windows().any(|(a, b)| a > b) {
        return false;
    }
    if digits.iter().tuple_windows().all(|(a, b)| a != b) {
        return false;
    }
    todo!();

    false
}
