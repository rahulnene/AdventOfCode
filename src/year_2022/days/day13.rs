use serde::Deserialize;
use std::cmp::Ordering;

pub fn solution(part: u8) -> u32 {
    let text = include_str!("../../problem_inputs/day13.txt");
    match part {
        1 => part1(text),
        2 => part2(text),
        _ => 0,
    }
}

fn part1(text: &str) -> u32 {
    let mut sum = 0;
    text.split("\n\n").enumerate().for_each(|(i, groups)| {
        let mut nodes = groups
            .lines()
            .map(|line| serde_json::from_str::<Node>(line).unwrap());
        let (l, r) = (nodes.next().unwrap(), nodes.next().unwrap());
        sum += (i + 1) * usize::from(l < r);
    });
    sum as u32
}

fn part2(text: &str) -> u32 {
    let dividers = vec![
        Node::List(vec![Node::Number(2)]),
        Node::List(vec![Node::Number(6)]),
    ];
    let mut packets = text
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| serde_json::from_str::<Node>(line).unwrap())
        .chain(dividers.iter().cloned())
        .collect::<Vec<Node>>();

    packets.sort_unstable();

    dividers
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>() as u32
}

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

impl Node {
    fn as_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Number(a), Node::Number(b)) => a.partial_cmp(b),
            (l, r) => l.as_slice(|l| r.as_slice(|r| l.partial_cmp(r))),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
