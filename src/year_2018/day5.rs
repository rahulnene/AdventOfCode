use itertools::Itertools;

#[must_use]
pub fn solution(part: u8) -> usize {
    let line = include_str!("../../../problem_inputs_2018/day_5.txt");
    let polymer = Polymer(line.to_owned());

    match part {
        1 => fully_react(&polymer),
        2 => solve02(line),
        _ => 1,
    }
}

fn fully_react(polymer: &Polymer) -> usize {
    let mut polymer = polymer.clone();
    while let Some(next_poly) = polymer.react() {
        polymer.0 = next_poly;
    }
    polymer.0.len()
}

fn solve02(polymer: &str) -> usize {
    let types = Polymer(polymer.to_owned()).types();
    let mut min_length = usize::MAX;
    for typ in types.iter() {
        let typeless_polymer: Polymer = Polymer(
            polymer
                .chars()
                .filter(|c| c != typ && *c != typ.to_ascii_uppercase())
                .collect(),
        );
        min_length = min_length.min(fully_react(&typeless_polymer));
    }
    min_length
}

#[derive(Clone, Debug)]
struct Polymer(String);

impl Polymer {
    fn react(&self) -> Option<String> {
        self.0
            .chars()
            .tuple_windows()
            .find(|(a, b)| (*a as u8).abs_diff(*b as u8) == 32_u8)
            .map(|(a, b)| self.0.replacen(&format!("{a}{b}"), "", 1))
    }

    fn types(&self) -> Vec<char> {
        return self
            .0
            .chars()
            .flat_map(char::to_lowercase)
            .unique()
            .collect_vec();
    }
}
