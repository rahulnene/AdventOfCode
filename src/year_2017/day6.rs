use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher as _},
    iter::repeat_with,
};

use fxhash::FxHashMap;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2017/day_6.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut mem = Memory::parse(lines);
    repeat_with(|| mem.redistribute().0)
        .position(|x| x == false)
        .unwrap()
        + 1
}

fn solve02(lines: &str) -> usize {
    let mut mem = Memory::parse(lines);
    let offset = solve01(lines);
    let (_, repeat_value) = mem.redistribute();
    let offset_repeat = *mem.states.get(&repeat_value).unwrap();
    offset + 1 - offset_repeat
}

type State = Vec<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Memory {
    bank_state: State,
    states: FxHashMap<usize, usize>,
}

fn hash_state(state: &State) -> usize {
    let mut hasher = DefaultHasher::default();
    state.hash(&mut hasher);
    hasher.finish() as usize
}

impl Memory {
    fn parse(s: &str) -> Self {
        let banks = s
            .split_whitespace()
            .map(|f| f.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut states = FxHashMap::default();
        states.insert(hash_state(&banks), 0);
        Memory {
            bank_state: banks,
            states,
        }
    }

    fn redistribute(&mut self) -> (bool, usize) {
        let (mut max_ind, max_val) = self
            .bank_state
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, v)| **v)
            .unwrap();
        let mut max_val = *max_val;
        self.bank_state[max_ind] = 0;
        while max_val > 0 {
            max_ind = (max_ind + 1) % self.bank_state.len();
            self.bank_state[max_ind] += 1;
            max_val -= 1;
        }
        let hash = hash_state(&self.bank_state);
        if self.states.contains_key(&hash) {
            (false, hash)
        } else {
            self.states.insert(hash, self.states.len());
            (true, hash)
        }
    }

    fn print(&self) {
        println!("{:?}", self.bank_state)
    }
}
