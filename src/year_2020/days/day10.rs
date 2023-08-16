use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_10.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    lines
        .lines()
        .map(|line| match find_error(line).0 {
            Some(c) => score_err(c),
            None => 0,
        })
        .sum()
}

fn solve02(lines: &str) -> usize {
    median(
        &lines
            .lines()
            .filter_map(|line| Some(find_error(line).1))
            .map(|mut autocomplete| {
                autocomplete.reverse();
                score_autocomplete(&autocomplete)
            })
            .collect::<Vec<_>>(),
    )
}

fn find_error(line: &str) -> (Option<char>, Vec<char>) {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '<' | '(' | '{' | '[' => stack.push(c),
            '>' | ')' | '}' | ']' => {
                let last = stack.pop();
                if last.is_none() | !{ get_pair_and_is_pair(last.unwrap(), c).1 } {
                    return (Some(c), stack);
                }
            }
            _ => (),
        }
    }
    return (None, stack);
}

fn get_pair_and_is_pair(open: char, close: char) -> (char, bool) {
    let pair = match open {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => panic!("Invalid char"),
    };
    let is_pair = pair == close;
    (pair, is_pair)
}

fn score_err(char: char) -> usize {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_autocomplete(stack: &[char]) -> usize {
    stack.iter().fold(0, |score, c| {
        score * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!(),
            }
    })
}

fn median(v: &[usize]) -> usize {
    let mut v = v.to_vec();
    v.sort();
    v[v.len() / 2]
}
