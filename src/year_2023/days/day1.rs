use regex::Regex;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2023/day_1.txt");
    match part {
        1 => solve(lines),
        2 => solve(&replace_numbers(lines)),
        _ => 1,
    }
}

fn solve(lines: &str) -> usize {
    lines.lines().map(|l| calibration_value(l)).sum()
}


fn calibration_value(line: &str) -> usize {
    let s: Vec<usize> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as usize)
        .collect();
    match s.as_slice() {        
        [single] => 11 * single,
        [tens, .. , ones] => 10 * tens + ones,
        _ => 0,
    }
}

fn replace_numbers(s: &str) -> String {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    let result = re.replace_all(s, |caps: &regex::Captures| {
        match caps.get(0).unwrap().as_str() {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => unreachable!(),
        }
    });
    result.to_string()
}