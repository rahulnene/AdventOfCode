pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_2.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut count = 0;
    for line in lines.lines() {
        let (policy, password) = line.split_once(": ").unwrap();
        count += check1(policy, password) as usize;
    }

    count
}

fn solve02(lines: &str) -> usize {
    let mut count = 0;
    for line in lines.lines() {
        let (policy, password) = line.split_once(": ").unwrap();
        count += check2(policy, password) as usize;
    }

    count
}

fn check1(policy: &str, password: &str) -> bool {
    let (range, letter) = policy.split_once(" ").unwrap();
    let (min, max) = range.split_once("-").unwrap();
    let min = min.parse::<usize>().unwrap();
    let max = max.parse::<usize>().unwrap();

    let mut count = 0;
    for c in password.chars() {
        if c == letter.chars().next().unwrap() {
            count += 1;
        }
    }

    count >= min && count <= max
}

fn check2(policy: &str, password: &str) -> bool {
    let (range, letter) = policy.split_once(" ").unwrap();
    let (first, second) = range.split_once("-").unwrap();
    let first = first.parse::<usize>().unwrap();
    let second = second.parse::<usize>().unwrap();

    let first_true = password.chars().nth(first - 1).unwrap() == letter.chars().next().unwrap();
    let second_true = password.chars().nth(second - 1).unwrap() == letter.chars().next().unwrap();

    xor(first_true, second_true)
}

fn xor(a: bool, b: bool) -> bool {
    a && !b || !a && b
}
