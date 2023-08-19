use std::fmt::Debug;

use fxhash::FxHashMap;
use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_7_test.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut color_hash = FxHashMap::<u32, &str>::default();
    let mut bags = FxHashMap::<u32, Vec<u32>>::default();
    for line in lines.lines() {
        let line = line.split(' ').collect_vec();
        let (color_str, color) = (line[1], hash_color(line[1]));
        color_hash.insert(color, color_str);
        let mut contains = Vec::new();
        if line[4] != "no" {
            for i in (4..line.len()).step_by(4) {
                let (color_str, color) = (line[i + 2], hash_color(line[i + 2]));
                color_hash.insert(color, color_str);
                contains.push(color);
            }
        }
        bags.insert(color, contains);
    }
    let keys = bags.keys().map(|f| color_hash.get(f).unwrap());
    let values = bags
        .values()
        .map(|f| f.iter().map(|f| color_hash.get(f).unwrap()).collect_vec());
    let zipped = keys.zip(values);
    dbg!(zipped.collect_vec());
    0
}

fn solve02(lines: &str) -> usize {
    0
}

fn hash_color(color: &str) -> u32 {
    let mut hash = 0;
    for c in color.chars() {
        hash += c as u32;
    }
    hash
}
