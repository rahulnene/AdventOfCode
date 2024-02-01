use std::time::{Duration, Instant};

use itertools::Itertools;
use lazy_static::lazy_static;
use rustc_hash::FxHashSet;

const LINES: &str = include_str!("../../problem_inputs_2015/day_19.txt");

lazy_static! {
    static ref LINES_VEC: Vec<&'static str> = LINES.lines().collect::<Vec<_>>();
    static ref REPLACEMENTS: Vec<(&'static str, &'static str)> = LINES_VEC
        .iter()
        .filter(|l| l.contains("=>"))
        .map(|l| {
            let mut split = l.split(" => ");
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect::<Vec<_>>();
    static ref MOLECULE: &'static str = LINES_VEC.last().unwrap();
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut new_molecules = FxHashSet::default();
    for r in REPLACEMENTS.iter() {
        let changed_molecules = replace(*MOLECULE, r);
        for m in changed_molecules {
            new_molecules.insert(m);
        }
    }
    (new_molecules.len(), now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut steps = 0;
    let target_molecule = MOLECULE.to_owned();
    let mut current_molecules = vec![target_molecule.clone()];
    while !current_molecules.contains(&"e".to_owned()) {
        let mut new_molecules = FxHashSet::default();
        for m in current_molecules {
            for r in REPLACEMENTS.iter() {
                let changed_molecules = get_possible_ancestors(&m, r);
                for m in changed_molecules {
                    new_molecules.insert(m);
                }
            }
        }
        current_molecules = new_molecules
            .into_iter()
            // .filter(|s| s.len() < target_molecule.len())
            .collect_vec();
        // dbg!(&current_molecules);
        steps += 1;
        dbg!(steps);
    }
    (steps, now.elapsed())
}

fn replace(molecule: &str, replacement: &(&str, &str)) -> Vec<String> {
    let mut ans = Vec::new();
    let replacement_indices = molecule
        .match_indices(replacement.0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    for i in replacement_indices {
        let mut temp = String::new();
        temp.push_str(&molecule[..i]);
        temp.push_str(replacement.1);
        temp.push_str(&molecule[i + replacement.0.len()..]);
        ans.push(temp);
    }
    ans
}

fn get_possible_ancestors(molecule: &str, replacement: &(&str, &str)) -> Vec<String> {
    let mut ans = Vec::new();
    let replacement_indices = molecule
        .match_indices(replacement.1)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    for i in replacement_indices {
        let mut temp = String::new();
        temp.push_str(&molecule[..i]);
        temp.push_str(replacement.0);
        temp.push_str(&molecule[i + replacement.1.len()..]);
        ans.push(temp);
    }
    ans
}
