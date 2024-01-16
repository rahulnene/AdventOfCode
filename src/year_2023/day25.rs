use std::iter;

use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_25.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let numbers: Vec<usize> = lines.lines().map(|l| l.parse::<usize>().unwrap()).collect();
    let loop_sizes = numbers.iter().map(|a| find_loop_size(*a)).collect_vec();
    iterate(loop_sizes[1], numbers[1], loop_sizes[0])
}

fn solve02(lines: &str) -> usize {
    0
}

fn iterate(loop_num: usize, subject: usize, loop_size: usize) -> usize {
    let mut a = 1;
    for _ in 0..loop_size {
        a = iterate_once(subject, a);
    }
    a
}

fn iterate_once(subject: usize, number: usize) -> usize {
    (number * subject) % 20201227
}

fn find_loop_size(number: usize) -> usize {
    let mut a = 1;
    let mut loop_size = 0;
    loop {
        loop_size += 1;
        a = iterate_once(7, a);
        if a == number {
            return loop_size;
        }
    }
}
