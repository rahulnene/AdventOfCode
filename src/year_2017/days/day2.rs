pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2017/day_2.txt");
    match part {
        1 => solve(lines, find_difference),
        2 => solve(lines, find_quotient),
        _ => 1,
    }
}

fn solve(lines: &str, f: impl Fn(&[usize]) -> usize) -> usize {
    lines
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            f(&nums)
        })
        .sum()
}

fn find_quotient(nums: &[usize]) -> usize {
    for (i, num) in nums.iter().enumerate() {
        for (j, num2) in nums.iter().enumerate() {
            if i != j && num % num2 == 0 {
                return *num / *num2;
            }
        }
    }
    0
}

fn find_difference(nums: &[usize]) -> usize {
    nums.iter().max().unwrap() - nums.iter().min().unwrap()
}
