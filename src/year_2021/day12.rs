use std::{
    collections::{BTreeMap, HashSet},
    fmt::Debug,
    fmt::Formatter,
    fmt::Result,
    hash::Hash,
};

use fxhash::FxHashMap;
use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_12_test.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut name_to_hash: FxHashMap<usize, String> = FxHashMap::default();
    let caves = CaveSystem::from_input(lines, &mut name_to_hash);
    dbg!(&caves);
    let start = caves
        .caves
        .keys()
        .find(|c| c.name == hash("start"))
        .unwrap();
    let mut paths: HashSet<Path> = HashSet::new();
    paths.insert(Path::new(start));
    
    // let paths = paths
    //     .iter()
    //     .filter(|path| path.caves.last().unwrap().name == hash("end"))
    //     .collect_vec();
    dbg!(&paths, paths.len());
    dbg!(name_to_hash);
    0
}

fn solve02(lines: &str) -> usize {
    0
}

fn is_small_cave(cave: String) -> bool {
    !cave.chars().any(|f| f.is_uppercase())
}

fn is_large_cave(cave: &str) -> bool {
    !cave.chars().any(|f| f.is_lowercase())
}

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Cave {
    name: usize,
    is_small: bool,
    visited: bool,
}

impl Cave {
    fn new(name: &str) -> Self {
        Self {
            name: hash(name),
            is_small: is_small_cave(name.to_string()),
            visited: false,
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct CaveSystem {
    caves: BTreeMap<Cave, Vec<Cave>>,
}

impl CaveSystem {
    fn from_input(lines: &str, name_map: &mut FxHashMap<usize, String>) -> Self {
        let mut caves: BTreeMap<Cave, Vec<Cave>> = BTreeMap::new();
        for line in lines.lines() {
            let (a, b) = line.split_once("-").unwrap();

            name_map.insert(hash(a), a.to_string());
            name_map.insert(hash(b), b.to_string());
            let mut cave_a = Cave::new(a);
            let mut cave_b = Cave::new(b);
            if cave_a.name == hash("start") {
                cave_a.visited = true;
            }
            if cave_a.name == hash("start") {
                cave_a.visited = false;
            }
            if !caves.contains_key(&cave_a) {
                caves.insert(cave_a, vec![cave_b]);
            } else {
                caves.insert(
                    cave_a.clone(),
                    append_and_return(caves.get(&cave_a).unwrap(), cave_b),
                );
            }
            if !caves.contains_key(&cave_b) {
                caves.insert(cave_b, vec![cave_a]);
            } else {
                caves.insert(
                    cave_b.clone(),
                    append_and_return(caves.get(&cave_b).unwrap(), cave_a),
                );
            }
        }
        Self { caves }
    }

    fn get_neighbors(&self, cave: &Cave) -> &Vec<Cave> {
        self.caves.get(cave).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Path<'a> {
    caves: Vec<&'a Cave>,
}

impl<'a> Path<'a> {
    fn new(cave: &'a Cave) -> Path<'a> {
        let mut temp = Vec::new();
        temp.push(cave);
        Self { caves: temp }
    }

    fn go_to(&mut self, cave: &'a Cave) {
        self.caves.push(cave);
    }

    fn go_back(&mut self) {
        self.caves.pop();
    }
}

fn append_and_return(caves: &Vec<Cave>, cave: Cave) -> Vec<Cave> {
    let mut temp = caves.clone();
    temp.push(cave);
    temp
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}

impl Debug for CaveSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for cave in self.caves.keys() {
            write!(f, "{:?} -> {:?}\n", cave, self.caves.get(cave).unwrap())?;
        }
        Ok(())
    }
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| acc + c as usize)
}
