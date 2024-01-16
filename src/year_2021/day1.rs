use itertools::Itertools;

pub fn solution(part: u8) -> isize {
    let lines = include_str!("../../../problem_inputs_2021/day_1.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => -1,
    }
}

fn solve01(lines: &str) -> isize {
    count_increasing_pairs(&parse_input(lines))
}

fn solve02(lines: &str) -> isize {
    count_increasing_pairs(
        &parse_input(lines)
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .collect::<Vec<isize>>(),
    )
}

fn parse_input(lines: &str) -> Vec<isize> {
    lines.lines().map(|line| line.parse().unwrap()).collect()
}

fn count_increasing_pairs(nums: &[isize]) -> isize {
    nums.windows(2).filter(|w| w[1] > w[0]).count() as isize
}
