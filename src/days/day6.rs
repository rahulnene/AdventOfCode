use crate::util::read_lines;
use itertools::Itertools;

pub fn solution(part: u8) -> u32 {
    let line = read_lines("./problem_inputs/day6.txt")
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    match part {
        1 => part1(&line),
        2 => part2(&line),
        _ => 0,
    }
}

fn part1(line: &str) -> u32 {
    for (index, window) in char_windows(line, 4).enumerate() {
        if window.chars().unique().count() == 4 {
            return index as u32 + 4;
        }
    }
    0
}

fn part2(line: &str) -> u32 {
    for (index, window) in char_windows(line, 14).enumerate() {
        if window.chars().unique().count() == 14 {
            return index as u32 + 14;
        }
    }
    0
}

fn char_windows(src: & str, win_size: usize) -> impl Iterator<Item = &'_ str> {
    src.char_indices().filter_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .nth(win_size - 1)
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}
