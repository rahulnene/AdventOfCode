use crate::util::read_lines;
use itertools::Itertools;

pub fn solution(part: u8) -> u32 {
    let lines = read_lines("./problem_inputs/day3.txt").unwrap();
    match part {
        1 => part1(lines),
        2 => part2(lines),
        _ => 0,
    }
}

fn part1(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> u32 {
    lines
        .flatten()
        .filter_map(|line| {
            let (part1, part2) = line.split_at(line.len() / 2);
            let char_set = part1.chars().collect::<Vec<_>>();
            part2.chars().find(|&c| char_set.contains(&c)).map(priority)
        })
        .sum()
}

fn part2(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> u32 {
    lines
        .tuples()
        .filter_map(|(a, b, c)| {
            let char_set_a = a.unwrap().chars().collect::<Vec<_>>();
            let char_set_b = b.unwrap().chars().collect::<Vec<_>>();
            c.unwrap()
                .chars()
                .find(|&c| char_set_a.contains(&c) && char_set_b.contains(&c))
                .map(priority)
        })
        .sum()
}

fn priority(letter: char) -> u32 {
    if letter.is_ascii_uppercase() {
        letter as u32 - 38
    } else {
        letter as u32 - 96
    }
}
