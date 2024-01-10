use std::time::{Duration, Instant};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2015/day_19.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let (replacements_str, molecule) = lines.split("\n\n").collect_tuple().unwrap();
    let mut replacement_rules = FxHashMap::default();
    for line in replacements_str.lines() {
        let (from, to) = line.split(" => ").collect_tuple().unwrap();
        replacement_rules
            .entry(from)
            .or_insert_with(Vec::new)
            .push(to);
    }
    let rule = replacement_rules.keys().skip(5).next().unwrap();
    let a = replacement_with_rule(molecule, rule, replacement_rules.get(rule).unwrap()[0]);
    dbg!(a);
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn replacement_with_rule(molecule: &str, from: &str, to: &str) -> Vec<String> {
    molecule
        .match_indices(from)
        .map(|(i, _)| {
            let mut new_molecule = molecule.to_string();
            new_molecule.replace_range(i..i + from.len(), to);
            new_molecule
        })
        .collect()
}
