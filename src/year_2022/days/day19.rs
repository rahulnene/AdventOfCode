use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex, UnGraph};
use regex::Regex;
use std::collections::HashMap;

pub fn solution(part: u8) -> u32 {
    let lines = include_str!("../../problem_inputs/day19.txt");
    match part {
        1 => part1(lines),
        2 => part2(lines),
        _ => 0,
    }
}

fn part1(lines: &str) -> u32 {
    let re = Regex::new(r"Blueprint (?P<id>\d+): Each ore robot costs (?P<orebot_cost_ore>\d+) ore. Each clay robot costs (?P<claybot_cost_ore>\d+) ore. Each obsidian robot costs (?P<obsbot_cost_ore>\d+) ore and (?P<obsbot_cost_clay>\d+) clay. Each geode robot costs (?P<geobotcost_ore>\d+) ore and (?P<geobotcost_clay>\d+) obsidian.").unwrap();
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for (
        _,
        [id, orebot_cost_ore, claybot_cost_ore, obsbot_cost_ore, obsbot_cost_clay, geobotcost_ore, geobotcost_clay],
    ) in re.captures_iter(lines).map(|c| c.extract())
    {
        let orebot = OreBot {
            ore_cost: orebot_cost_ore.parse().unwrap(),
            count: 0,
        };
        let claybot = ClayBot {
            ore_cost: claybot_cost_ore.parse().unwrap(),
            count: 0,
        };
        let obsbot = ObsidianBot {
            ore_cost: obsbot_cost_ore.parse().unwrap(),
            clay_cost: obsbot_cost_clay.parse().unwrap(),
            count: 0,
        };
        let geobot = GeoBot {
            ore_cost: geobotcost_ore.parse().unwrap(),
            obs_cost: geobotcost_clay.parse().unwrap(),
            count: 0,
        };
        let blueprint = Blueprint {
            id: id.parse().unwrap(),
            orebot,
            claybot,
            obsbot,
            geobot,
        };
        blueprints.push(blueprint);
    }
    blueprints.iter().map(|bp| bp.quality_level()).sum()
}

fn part2(lines: &str) -> u32 {
    todo!()
}

impl Blueprint {
    fn quality_level(&self) -> u32 {
        self.id * self.largest_geode_count()
    }

    fn largest_geode_count(&self) -> u32 {

        0   
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Name([u8; 2]);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Blueprint {
    id: u32,
    orebot: OreBot,
    claybot: ClayBot,
    obsbot: ObsidianBot,
    geobot: GeoBot,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct OreBot {
    ore_cost: u32,
    count: u32,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct ClayBot {
    ore_cost: u32,
    count: u32,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct ObsidianBot {
    ore_cost: u32,
    clay_cost: u32,
    count: u32,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct GeoBot {
    ore_cost: u32,
    obs_cost: u32,
    count: u32,
}
