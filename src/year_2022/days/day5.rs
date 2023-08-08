use crate::util::read_lines;
use itertools::Itertools;

pub fn solution(part: u8) -> u32 {
    let mut lines = read_lines("./problem_inputs/day4.txt").unwrap();
    match part {
        1 => part1(lines),
        // 2 => part2(lines),
        _ => 0,
    }
}

fn part1(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> u32 {
    todo!()
}
