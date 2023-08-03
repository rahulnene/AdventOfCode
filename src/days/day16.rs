use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex, UnGraph};
use regex::Regex;
use std::collections::HashMap;

pub fn solution(part: u8) -> u32 {
    let lines = include_str!("../../problem_inputs/day16.txt");
    match part {
        1 => part1(lines),
        2 => part2(lines),
        _ => 0,
    }
}

fn part1(lines: &str) -> u32 {
    let re = Regex::new(r"Valve (?P<name>\w+) has flow rate=(?P<flow_rate>\d+); tunnel[s]? lead[s]? to valve[s]? (?P<connections>(?:\w+(?:, )?)+)").unwrap();
    let mut valves: Vec<Valve> = Vec::new();
    for (_, [name, flow_rate, connections]) in re.captures_iter(lines).map(|c| c.extract()) {
        valves.push(Valve::new(
            name.to_string(),
            flow_rate.parse::<u32>().unwrap(),
            connections
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        ));
    }
    // dbg!(valves);
    todo!();

    0
}

fn part2(lines: &str) -> u32 {
    todo!()
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    flow_rate: u32,
    connections: Vec<String>,
}

impl Valve {
    fn new(name: String, flow_rate: u32, connections: Vec<String>) -> Self {
        Self {
            name,
            flow_rate,
            connections,
        }
    }
}
