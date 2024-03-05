use regex::Regex;
use rustc_hash::FxHashSet;

use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2018/day_24_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut groups = Vec::new();
    let mut lines = LINES.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() || line.contains(':') {
            break;
        }
        groups.push(Group::parse(line));
    }
    dbg!(groups);
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
    unit_count: usize,
    hit_points: usize,
    weaknesses: FxHashSet<String>,
    immunities: FxHashSet<String>,
    attack: Attack,
    initiative: usize,
}

impl Group {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"(?P<unit_count>\d+) units each with (?P<hit_points>\d+) hit points \((?P<weakness>.*?)\) with an attack that does (?P<attack_num>\d+) (?P<attack_type>\w+) damage at initiative (?P<initiative>\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let unit_count = caps.name("unit_count").unwrap().as_str().parse().unwrap();
        let hit_points = caps.name("hit_points").unwrap().as_str().parse().unwrap();
        let (weaknesses, immunities) =
            parse_weaknesses_and_immunities(caps.name("weakness").unwrap().as_str());
        let attack = Attack {
            damage: caps.name("attack_num").unwrap().as_str().parse().unwrap(),
            attack_type: caps.name("attack_type").unwrap().as_str().to_string(),
        };
        let initiative = caps.name("initiative").unwrap().as_str().parse().unwrap();
        Group {
            unit_count,
            hit_points,
            weaknesses,
            immunities,
            attack,
            initiative,
        }
    }
}

fn parse_weaknesses_and_immunities(attack: &str) -> (FxHashSet<String>, FxHashSet<String>) {
    let mut weaknesses = FxHashSet::default();
    let mut immunities = FxHashSet::default();

    (weaknesses, immunities)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Attack {
    damage: usize,
    attack_type: String,
}

impl Attack {
    fn parse(s: &str) -> Self {
        let (damage, attack_type) = s.split_once(' ').unwrap();
        Attack {
            damage: damage.parse().unwrap(),
            attack_type: attack_type.to_string(),
        }
    }
}
