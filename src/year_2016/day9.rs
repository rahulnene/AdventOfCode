use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::{Duration, Instant};

lazy_static! {
    static ref RE: Regex = Regex::new(r"\(([^)]*)\)").expect("Bad regex pattern");
    static ref LINE: String = include_str!("../../problem_inputs_2016/day_9_test.txt").to_owned();
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(&LINE), solve02(&LINE))
}

fn solve01(line: &str) -> (usize, Duration) {
    let now = Instant::now();
    let out_str = decompress(line);
    (out_str.len(), now.elapsed())
}

fn solve02(line: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut out_str = line.to_owned();
    let mut total_len = 0;
    
    while !out_str.is_empty() {
        // let now = Instant::now();
        out_str = decompress(&out_str);
        let extension_len = out_str.chars().take_while(|c| c.is_alphabetic()).count();
        out_str = out_str[extension_len..].to_owned();
        total_len += extension_len;
        // dbg!(now.elapsed());
    }
    (total_len, now.elapsed())
}

fn decompress(s: &str) -> String {
    let mut chars = s.chars().peekable();
    let mut out_str = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_alphabetic() {
            out_str.push(chars.next().unwrap());
        } else if c == '(' {
            chars.next(); // consume '('
            let amount_to_take: usize = chars
                .by_ref()
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap();
            let times_to_repeat: usize = chars
                .by_ref()
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap();
            let to_repeat: String = chars.by_ref().take(amount_to_take).collect();
            out_str.push_str(&to_repeat.repeat(times_to_repeat));
            out_str.extend(chars);
            return out_str
        }
    }
    out_str
}
