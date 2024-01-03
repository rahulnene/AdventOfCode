use std::time::Instant;

use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_12_test.txt");
    let mut rule_set = FxHashMap::default();
    let (initial_state, rule_lines): (&str, &str) = lines.split("\n\n").collect_tuple().unwrap();
    let initial_state = initial_state
        .split(' ')
        .collect_tuple::<(&str, &str, &str)>()
        .unwrap()
        .2;
    for rule_line in rule_lines.lines() {
        let (condition_str, _, output) = rule_line.split(' ').collect_tuple().unwrap();
        let condition = condition_str.chars().collect_vec();
        rule_set.insert(condition, output.chars().at_most_one().unwrap().unwrap());
    }
    for i in [1000, 2000, 8000, 16000, 32000] {
        let now = Instant::now();
        solve(initial_state.chars().collect_vec(), &rule_set, i);
        println!("{i} generations -> {:?}", Instant::now() - now);
    }

    match part {
        1 => solve(initial_state.chars().collect_vec(), &rule_set, 20),
        2 => solve(initial_state.chars().collect_vec(), &rule_set, 1000),
        _ => 1,
    }
}

fn solve01(initial_state: Vec<char>, ruleset: &FxHashMap<Vec<char>, char>) -> usize {
    let mut initial_state = initial_state;
    for _ in 0..20 {
        initial_state = step(&initial_state, ruleset);
    }
    // println!(
    //     "{:?}",
    //     initial_state
    //         .iter()
    //         .enumerate()
    //         .filter(|f| *f.1 == '#')
    //         .map(|f| f.0 as isize - 20)
    //         .collect_vec()
    // );
    let a: isize = initial_state
        .iter()
        .enumerate()
        .filter(|f| *f.1 == '#')
        .map(|f| f.0 as isize - 20)
        .sum();
    // println!("{:?}", &initial_state);
    a as usize
}

fn solve(
    initial_state: Vec<char>,
    ruleset: &FxHashMap<Vec<char>, char>,
    generations: usize,
) -> usize {
    let mut initial_state = initial_state;
    for _ in 0..generations {
        initial_state = step(&initial_state, ruleset);
    }
    let a: isize = initial_state
        .iter()
        .enumerate()
        .filter(|f| *f.1 == '#')
        .map(|f| f.0 as isize - generations as isize)
        .sum();
    // println!("{:?}", &initial_state);
    a as usize
}

fn step(state: &Vec<char>, ruleset: &FxHashMap<Vec<char>, char>) -> Vec<char> {
    let mut temp_state = Vec::with_capacity(state.len() + 6);
    temp_state.resize(3, '.');
    temp_state.extend(state);
    temp_state.resize(temp_state.len() + 3, '.');
    temp_state
        .windows(5)
        .map(|w| *ruleset.get(w).unwrap_or_else(|| &'.'))
        .collect_vec()
}
