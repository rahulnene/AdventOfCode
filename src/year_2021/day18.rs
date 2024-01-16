use std::time::{Instant, Duration};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2021/day_18.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SnailFishNumber {
    left: SnailFishNumberPart,
    right: SnailFishNumberPart,
    depth: usize
}

impl SnailFishNumber {
    fn new() -> Self {
        Self {
            left: SnailFishNumberPart::Number(0),
            right: SnailFishNumberPart::Number(0),
            depth: 0
        }
    }

    
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailFishNumberPart {
    Number(usize),
    Pair(Box<SnailFishNumber>)
}