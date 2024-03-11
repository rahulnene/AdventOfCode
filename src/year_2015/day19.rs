use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use rustc_hash::{FxHashMap, FxHashSet};

const LINES: &str = include_str!("../../problem_inputs_2015/day_19.txt");
lazy_static! {
    static ref SOURCE: String = LINES.lines().last().unwrap().to_string();
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let mut replacers = FxHashMap::default();
    let mut ancestors = FxHashMap::default();
    let mut lines = LINES.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(" => ");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        replacers
            .entry(from.to_owned())
            .or_insert_with(Vec::new)
            .push(to.to_owned());
        ancestors
            .entry(to.to_owned())
            .or_insert_with(Vec::new)
            .push(from.to_owned());
    }
    // let largest_replacement = replacers.values().flatten().max_by_key(|v| v.len());
    // dbg!(largest_replacement.unwrap().len());
    (solve01(&replacers), solve02())
}

fn solve01(replacers: &FxHashMap<String, Vec<String>>) -> (usize, Duration) {
    let now = Instant::now();
    let mut molecules: FxHashSet<String> = FxHashSet::default();
    molecules.insert(SOURCE.to_owned());
    molecules = generate_replacements(&molecules, replacers);
    (molecules.len(), now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let molecule = tokenize();
    dbg!(&molecule);
    let tokens = molecule.len();
    let parens = molecule
        .iter()
        .filter(|f| **f == "Rn" || **f == "Ar")
        .count();
    let commas = molecule.iter().filter(|f| **f == "Y").count();
    dbg!(tokens, parens, commas);
    let ans = tokens - parens - 2 * commas - 1;
    (ans, now.elapsed())
}

fn tokenize() -> Vec<String> {
    println!("{:?}", SOURCE.as_str());
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    for i in 0..SOURCE.len() {
        let c: char = SOURCE.chars().nth(i).unwrap();
        dbg!(c);
        if c.is_lowercase() {
            current.push(c);
        } else {
            tokens.push(current.clone());
            current.clear();
            current.push(c);
        }
    }
    tokens
}

fn generate_replacements(
    molecules: &FxHashSet<String>,
    replacers: &FxHashMap<String, Vec<String>>,
) -> FxHashSet<String> {
    let mut new_molecules = FxHashSet::default();
    for molecule in molecules {
        for i in 0..molecule.len() {
            for j in 0..10 {
                if j > molecule.len() {
                    break;
                }
                if i < molecule.len() - j {
                    if let Some(replacements) = replacers.get(&molecule[i..=i + j]) {
                        for replacement in replacements {
                            let mut new_molecule = molecule.to_string();
                            new_molecule.replace_range(i..=i + j, replacement);
                            new_molecules.insert(new_molecule.to_owned());
                        }
                    }
                }
            }

            // if i < molecule.len() {
            //     if let Some(replacements) = replacers.get(&molecule[i..i + 1]) {
            //         for replacement in replacements {
            //             let mut new_molecule = molecule.to_string();
            //             new_molecule.replace_range(i..i + 1, replacement);
            //             new_molecules.insert(new_molecule.to_owned());
            //         }
            //     }
            // }
            // if i < molecule.len() - 1 {
            //     if let Some(replacements) = replacers.get(&molecule[i..i + 2]) {
            //         for replacement in replacements {
            //             let mut new_molecule = molecule.to_string();
            //             new_molecule.replace_range(i..i + 2, replacement);
            //             new_molecules.insert(new_molecule.to_owned());
            //         }
            //     }
            // }
        }
    }
    new_molecules
}
