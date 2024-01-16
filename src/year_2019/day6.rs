use fxhash::{FxHashMap, FxHashSet};
pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2019/tests/day_6.txt");
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    name: String,
    parent: String,
    distance: usize,
}

impl Node {
    fn new(name: String, parent: String, distance: usize) -> Self {
        Self {
            name,
            parent,
            distance,
        }
    }

    fn from_str(s: &str) -> Self {
        let mut split = s.split(')');
        let parent = split.next().unwrap();
        let name = split.next().unwrap();
        Self::new(name.to_string(), parent.to_string(), 0)
    }
}
