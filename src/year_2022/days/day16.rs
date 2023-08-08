use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex, UnGraph};
use regex::Regex;
use std::collections::HashMap;

pub fn solution(part: u8) -> u32 {
    let lines = include_str!("../../../problem_inputs/day16.txt");
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
    // dbg!(&valves);
    dbg!(Network::new(valves));
    todo!();

    0
}

fn part2(lines: &str) -> u32 {
    todo!()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Name([u8; 2]);

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Valve {
    name: Name,
    flow_rate: u32,
    connections: Vec<Name>,
}

impl Valve {
    fn new(name: String, flow_rate: u32, connections: Vec<String>) -> Self {
        Self {
            name: Name([name.as_bytes()[0], name.as_bytes()[1]]),
            flow_rate,
            connections: connections
                .iter()
                .map(|conn| Name([conn.as_bytes()[0], conn.as_bytes()[1]]))
                .collect::<Vec<Name>>(),
        }
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Network {
    valves: HashMap<Name, Valve>,
}
impl Network {
    fn new(valve_list: Vec<Valve>) -> Self {
        let mut valves = HashMap::new();
        valve_list.into_iter().for_each(|v| {
            valves.insert(v.name, v);
        });
        Self { valves }
    }

    fn connections(&self, start_valve: Name) -> HashMap<Name, Path> {
        let mut conns: HashMap<Name, Path>= HashMap::new();
        conns.insert(start_valve, (Name::default(), Name::default()));
        conns
    }
}

pub type Path = (Name, Name);
