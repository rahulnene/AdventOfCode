use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let line = include_str!("../../../problem_inputs_2018/day_5.txt");
    match part {
        1 => fully_react(line),
        2 => solve02(line),
        _ => 1,
    }
}

fn fully_react(polymer: &str) -> usize {
    let mut polymer = Polymer(polymer.to_owned());
    while let Some(next_poly) = polymer.react() {
        polymer.0 = next_poly;
    }
    polymer.0.len()
}

fn solve02(polymer: &str) -> usize {
    let mut polymer = Polymer(polymer.to_owned());
    dbg!(polymer.types());
    0    
}

#[derive(Clone, Debug)]
struct Polymer(String);

impl Polymer {
    fn react(&self) -> Option<String> {
        match self
            .0
            .chars()
            .tuple_windows()
            .find(|(a, b)| (*a as u8).abs_diff(*b as u8) == 32_u8)
        {
            Some((a, b)) => {
                Some(self.0.replacen(&format!("{a}{b}"), "", 1))
            }
            None => {
                None
            }
        }
    }

    fn types(&self) -> Vec<char> {
        return self.0.chars().flat_map(char::to_lowercase).unique().collect_vec();
    }
}
