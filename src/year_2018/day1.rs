pub fn solution(part: u8) -> isize {
    let lines = include_str!("../../../problem_inputs_2018/day_1.txt");
    match part {
        1 => solve01(lines),
        // 2 => solve(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> isize {
    let mut sum = 0;
    for line in lines.lines() {
        sum += if line.chars().next().unwrap() == '+' {
            line[1..].parse::<isize>().unwrap()
        } else {
            line.parse::<isize>().unwrap()
        }
    }
    sum
}

fn solve02(lines: &str) -> usize {
    0
}
