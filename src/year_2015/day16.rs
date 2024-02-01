use rustc_hash::FxHashMap;
use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};

const LINES: &str = include_str!("../../problem_inputs_2015/day_16.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve(false), solve(true))
}

fn solve(is_part_2: bool) -> (usize, Duration) {
    let now = Instant::now();
    let possible_aunts = LINES
        .lines()
        .map(|a| PossibleAunt::from_str(a, is_part_2))
        .collect::<Vec<_>>();
    let actual_aunt = PossibleAunt::from_str(
        "Sue 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: \
         0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1",
        is_part_2,
    );
    let ans = possible_aunts
        .iter()
        .find(|aunt| aunt.is_real_aunt(&actual_aunt))
        .unwrap()
        .id;
    (ans, now.elapsed())
}

#[derive(Debug, Clone)]
struct PossibleAunt {
    id: usize,
    properties: FxHashMap<Property, Option<(Ordering, usize)>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Property {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// enum PropertyComparison {
//     Equal,
//     GreaterThan,
//     LessThan,
// }

impl PossibleAunt {
    fn from_str(s: &str, is_part_2: bool) -> Self {
        let mut aunt = PossibleAunt {
            id: 0,
            properties: FxHashMap::default(),
        };
        let (id_str, props_str) = s.split_once(':').unwrap();
        aunt.id = id_str
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let props = props_str.split(',').map(|s| s.trim());
        for prop in props {
            let (prop_name, prop_value) = prop.split_once(':').unwrap();
            let prop_name = prop_name.trim();
            let prop_value = prop_value.trim();
            let prop = match prop_name {
                "children" => Property::Children,
                "cats" => Property::Cats,
                "samoyeds" => Property::Samoyeds,
                "pomeranians" => Property::Pomeranians,
                "akitas" => Property::Akitas,
                "vizslas" => Property::Vizslas,
                "goldfish" => Property::Goldfish,
                "trees" => Property::Trees,
                "cars" => Property::Cars,
                "perfumes" => Property::Perfumes,
                _ => panic!("Invalid property name: {}", prop_name),
            };
            let prop_value = match prop_value.parse::<usize>() {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            let prop_comparison = {
                if is_part_2 {
                    match prop {
                        Property::Children
                        | Property::Samoyeds
                        | Property::Akitas
                        | Property::Vizslas
                        | Property::Cars
                        | Property::Perfumes => Ordering::Equal,
                        Property::Cats | Property::Trees => Ordering::Greater,
                        Property::Pomeranians | Property::Goldfish => Ordering::Less,
                    }
                } else {
                    Ordering::Equal
                }
            };
            aunt.properties
                .insert(prop, prop_value.map(|v| (prop_comparison, v)));
        }
        aunt
    }
    fn is_real_aunt(&self, actual_aunt: &PossibleAunt) -> bool {
        for (prop, value) in self.properties.iter() {
            if value.is_some() {
                match value.unwrap() {
                    (Ordering::Equal, v) => {
                        if v != actual_aunt.properties[prop].unwrap().1 {
                            return false;
                        }
                    }
                    (Ordering::Greater, v) => {
                        if v <= actual_aunt.properties[prop].unwrap().1 {
                            return false;
                        }
                    }
                    (Ordering::Less, v) => {
                        if v >= actual_aunt.properties[prop].unwrap().1 {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
