<<<<<<< HEAD
use std::time::{Duration, Instant};

use itertools::Itertools;
use rustc_hash::FxHashMap;
=======
use std::{
    hash::{Hash, Hasher},
    time::{Duration, Instant},
};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHasher};
>>>>>>> 999489142de4762b84f4c4a0854368f2632193c5

const LINES: &str = include_str!("../../problem_inputs_2022/day_7.txt");

fn prepare() -> (
    FxHashMap<String, Vec<ContentType>>,
    FxHashMap<String, usize>,
) {
    let now = Instant::now();
    let mut fs: FxHashMap<String, Vec<ContentType>> = FxHashMap::default();
    let mut dir_to_parent: FxHashMap<String, String> = FxHashMap::default();
    dir_to_parent.insert("/root".to_owned(), "/root".to_string());
    let mut file_sizes: FxHashMap<String, usize> = FxHashMap::default();
    let mut current_dir = vec!["/root".to_string()];
    for out_str in LINES.lines() {
        let msg = TerminalMessage::from_str(out_str);
        match msg {
            TerminalMessage::Command(comm) => match comm.op {
                Operation::Cd => match comm.path {
                    Path::Root => current_dir = vec!["/root".to_string()],
                    Path::Parent => {
                        current_dir.pop();
                    }
                    Path::Absolute(s) => current_dir.push(format!("/{s}")),
                    Path::Current => (),
                },
                Operation::Ls => {}
            },
            TerminalMessage::Output(content) => {
                fs.entry(current_dir.clone().into_iter().collect())
                    .and_modify(|v| v.push(content.clone()))
                    .or_insert(vec![content.clone()]);
                match content {
                    ContentType::Directory(ref dir_name) => {
                        dir_to_parent
                            .insert(dir_name.clone(), current_dir.clone().into_iter().collect());
                    }
                    ContentType::File(ref name, size) => {
                        file_sizes.insert(name.clone(), size);
                    }
                }
            }
        }
    }
    println!("Preparation took {:?}", now.elapsed());
    (fs, file_sizes)
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
<<<<<<< HEAD
    let (fs, mut file_sizes) = prepare();
=======
    let mut fs: FxHashMap<String, Vec<ContentType>> = FxHashMap::default();
    let mut dir_to_parent: FxHashMap<String, String> = FxHashMap::default();
    let root_hash = hash("/root");
    dir_to_parent.insert("/root".to_owned(), "/root".to_string());
    let mut file_sizes: FxHashMap<String, usize> = FxHashMap::default();
    let mut current_dir = vec!["/root".to_string()];
    for out_str in LINES.lines() {
        let msg = TerminalMessage::from_str(out_str);
        match msg {
            TerminalMessage::Command(comm) => match comm.op {
                Operation::Cd => match comm.path {
                    Path::Root => current_dir = vec!["/root".to_string()],
                    Path::Parent => {
                        current_dir.pop();
                    }
                    Path::Absolute(s) => current_dir.push(format!("/{s}")),
                    Path::Current => (),
                },
                Operation::Ls => {}
            },
            TerminalMessage::Output(content) => {
                fs.entry(current_dir.clone().into_iter().collect())
                    .and_modify(|v| v.push(content.clone()))
                    .or_insert(vec![content.clone()]);
                match content {
                    ContentType::Directory(ref dir_name) => {
                        dir_to_parent
                            .insert(dir_name.clone(), current_dir.clone().into_iter().collect());
                    }
                    ContentType::File(ref name, size) => {
                        file_sizes.insert(name.clone(), size);
                    }
                }
            }
        }
    }
>>>>>>> 999489142de4762b84f4c4a0854368f2632193c5
    calculate_size(&fs, "/root", &mut file_sizes);
    (solve01(&fs, &mut file_sizes), solve02(&fs, &mut file_sizes))
}

fn solve01(
    fs: &FxHashMap<String, Vec<ContentType>>,
    file_sizes: &mut FxHashMap<String, usize>,
) -> (usize, Duration) {
    let now = Instant::now();
    let ans: usize = fs
        .keys()
        .filter_map(|d| {
            let dir_size = calculate_size(fs, d, file_sizes);
            if dir_size <= 100_000 {
                Some(dir_size)
            } else {
                None
            }
        })
        .sum();
    (ans, now.elapsed())
}

fn solve02(
    fs: &FxHashMap<String, Vec<ContentType>>,
    file_sizes: &mut FxHashMap<String, usize>,
) -> (usize, Duration) {
    let now = Instant::now();
    let root_size = calculate_size(fs, "/root", file_sizes);
    let needed_size = root_size.abs_diff(40_000_000);
    let directory_to_size = fs
        .keys()
        .map(|d| (d.clone(), calculate_size(fs, d, file_sizes)))
        .sorted_unstable_by_key(|d| d.1)
        .find(|d| d.1 >= needed_size)
        .unwrap();
    (directory_to_size.1, now.elapsed())
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Cd,
    Ls,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Path {
    Absolute(String),
    Root,
    Parent,
    Current,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Command {
    op: Operation,
    path: Path,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum TerminalMessage {
    Command(Command),
    Output(ContentType),
}

impl TerminalMessage {
    fn from_str(s: &str) -> Self {
        if s.starts_with('$') {
            let mut parts = s.split_whitespace();
            parts.next();
            let op = match parts.next().unwrap() {
                "cd" => Operation::Cd,
                "ls" => Operation::Ls,
                _ => unreachable!(),
            };
            let path = match parts.next() {
                Some("/") => Path::Root,
                Some("..") => Path::Parent,
                Some(s) => Path::Absolute(s.to_string()),
                None => Path::Current,
            };
            TerminalMessage::Command(Command { op, path })
        } else if s.starts_with("dir") {
            let (_, rest) = s.split_once(' ').unwrap();
            TerminalMessage::Output(ContentType::Directory(rest.to_string()))
        } else {
            let (size, name) = s.split_whitespace().collect_tuple().unwrap();
            let size = size.parse().unwrap();
            TerminalMessage::Output(ContentType::File(name.to_owned(), size))
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum ContentType {
    Directory(String),
    File(String, usize),
}

fn calculate_size(
    fs: &FxHashMap<String, Vec<ContentType>>,
    dir: &str,
    file_sizes: &mut FxHashMap<String, usize>,
) -> usize {
    if file_sizes.contains_key(dir) {
        return *file_sizes.get(dir).unwrap();
    }
    let mut size = 0;
    for content in fs.get(dir).unwrap() {
        match content {
            ContentType::Directory(dir_name) => {
                size += calculate_size(fs, &format!("{dir}/{dir_name}"), file_sizes);
            }
            ContentType::File(_, file_size) => {
                size += file_size;
            }
        }
    }
    file_sizes.insert(dir.to_string(), size);
    size
}
<<<<<<< HEAD
=======

fn hash(s: &str) -> usize {
    let mut hasher = FxHasher::default();
    s.hash(&mut hasher);
    hasher.finish() as usize
}
>>>>>>> 999489142de4762b84f4c4a0854368f2632193c5
