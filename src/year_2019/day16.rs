use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let given = include_str!("../../problem_inputs_2019/day_16.txt");
    (solve01(given), solve02(given))
}

fn solve01(given: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut s = given
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    for _ in 0..100 {
        s = transform(&s);
    }
    let ans = s
        .iter()
        .take(8)
        .map(|x| x.to_string())
        .join("")
        .parse()
        .unwrap();
    (ans, now.elapsed())
}

fn solve02(given: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut s = given
        .chars()
        .cycle()
        .take(10000 * given.len())
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    for _ in 0..100 {
        let now = Instant::now();
        s = transform(&s);
        dbg!(now.elapsed());
    }
    let ans = s
        .iter()
        .take(8)
        .map(|x| x.to_string())
        .join("")
        .parse()
        .unwrap();
    (ans, now.elapsed())
}

fn transform(s: &[u8]) -> Vec<u8> {
    (1..=s.len()).map(|i| process(s, i)).collect::<Vec<_>>()
}

fn process(s: &[u8], stage: usize) -> u8 {
    (s.iter()
        .zip(&get_interpolating_patter(s.len(), stage))
        .map(|(a, b)| *a as isize * *b as isize)
        .sum::<isize>()
        .abs()
        % 10) as u8
}

fn get_interpolating_patter(amount: usize, stage: usize) -> Vec<i8> {
    let pattern = vec![0, 1, 0, -1];
    let mut result = Vec::new();
    for c in pattern {
        for _ in 0..stage {
            result.push(c);
        }
    }
    result
        .iter()
        .cycle()
        .skip(1)
        .take(amount)
        .map(|x| *x as i8)
        .collect_vec()
}
