use fxhash::FxHashSet;
use regex::Regex;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_24_test.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut lines = lines.split("\n\n");
    let immune_system = lines.next().unwrap();
    let infection = lines.next().unwrap();
    let immune_system = immune_system
        .lines()
        .skip(1)
        .map(|line| Group::parse(line))
        .collect::<Vec<_>>();
    let infection = infection
        .lines()
        .skip(1)
        .map(|line| Group::parse(line))
        .collect::<Vec<_>>();
    dbg!(immune_system);
    0
}

fn solve02(lines: &str) -> usize {
    0
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
