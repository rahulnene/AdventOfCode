use itertools::Itertools;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

const LINES: &str = include_str!("../../problem_inputs_2022/day_5.txt");

pub fn solution() -> ((String, Duration), (String, Duration)) {
    let (boxes, instrs) = LINES.split_once("\r\n\r\n").unwrap();
    let mut box_vec = Vec::new();
    for line in boxes.lines() {
        let a = line.chars().chunks(4);
        for chunk in &a {
            box_vec.push(chunk.collect::<String>());
        }
    }
    box_vec = boxes
        .lines()
        .map(|line| {
            line.chars()
                .chunks(4)
                .into_iter()
                .map(|c| c.collect::<String>())
                .collect_vec()
        })
        .flatten()
        .collect_vec();
    let box_count = box_vec
        .iter()
        .last()
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();
    box_vec = box_vec[0..box_vec.len() - box_count].to_vec();
    let mut boxes = vec![Vec::new(); box_count];
    for (ind, box_str) in box_vec.iter().enumerate() {
        let char = box_str.chars().filter(|c| c.is_alphabetic()).collect_vec();
        if char.len() == 1 {
            boxes[ind % box_count].push(char[0]);
        }
    }
    let boxes = boxes.into_iter().map(|b| VecDeque::from(b)).collect_vec();
    let instrs = instrs
        .lines()
        .map(|l| {
            let mut iter = l.split(' ');
            let amount = iter.nth(1).unwrap().parse::<usize>().unwrap();
            let source_box = iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let target_box = iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            (amount, source_box, target_box)
        })
        .collect_vec();
    (solve01(&instrs, &boxes), solve02(&instrs, &boxes))
}

fn solve01(instrs: &[(usize, usize, usize)], boxes: &[VecDeque<char>]) -> (String, Duration) {
    let now = Instant::now();
    let mut boxes = boxes.to_owned();
    for i in instrs.iter() {
        let (amount, source_box, target_box) = *i;
        for _ in 0..amount {
            let moved = boxes[source_box].pop_front().unwrap();
            boxes[target_box].push_front(moved);
        }
    }

    (format_boxes(&boxes), now.elapsed())
}

fn solve02(instrs: &[(usize, usize, usize)], boxes: &[VecDeque<char>]) -> (String, Duration) {
    let now = Instant::now();
    let mut boxes = boxes.to_owned();
    for i in instrs.iter() {
        let (amount, source_box, target_box) = *i;
        let mut temp = VecDeque::new();
        for _ in 0..amount {
            let moved = boxes[source_box].pop_front().unwrap();
            temp.push_front(moved);
        }
        for _ in 0..amount {
            let moved = temp.pop_front().unwrap();
            boxes[target_box].push_front(moved);
        }
    }
    (format_boxes(&boxes), now.elapsed())
}

fn format_boxes(boxes: &[VecDeque<char>]) -> String {
    boxes
        .iter()
        .map(|b| b.iter().next().unwrap_or(&' '))
        .collect::<String>()
}
