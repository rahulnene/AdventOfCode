use crate::util::read_lines;
use itertools::Itertools;

pub fn solution(part: u8) -> u32 {
    let lines = read_lines("./problem_inputs/day3.txt").unwrap();
    let score = match part {
        1 => lines
            .flatten()
            .map(|line| {
                let (part1, part2) = line.split_at(line.len() / 2);
                let char_set = part1.chars().collect::<Vec<_>>();
                part2.chars().find(|&c| char_set.contains(&c)).map(priority)
            })
            .flatten()
            .sum(),
        2 => lines
            .tuples()
            .map(|(a, b, c)| {
                let char_set_a = a.unwrap().chars().collect::<Vec<_>>();
                let char_set_b = b.unwrap().chars().collect::<Vec<_>>();
                c.unwrap()
                    .chars()
                    .find(|&c| char_set_a.contains(&c) && char_set_b.contains(&c))
                    .map(priority)
            })
            .flatten()
            .sum(),
        _ => 0,
    };
    score
}

fn priority(letter: char) -> u32 {
    match letter.is_ascii_uppercase() {
        true => return letter as u32 - 38,
        false => return letter as u32 - 96,
    };
}
