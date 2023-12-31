use std::collections::BTreeSet;
pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_18.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    0
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Debug, Clone)]
enum Leaf {
    Pair(Pair),
    Number(usize),
}

#[derive(Debug, Clone)]
struct Pair {
    left: Box<Leaf>,
    right: Box<Leaf>,
}

impl Pair {
    fn new(left: Box<Leaf>, right: Box<Leaf>) -> Self {
        Self { left, right }
    }

    fn from(left: Leaf, right: Leaf) -> Self {
        Self::new(Box::new(left), Box::new(right))
    }
    
}
