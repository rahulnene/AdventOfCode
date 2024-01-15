use std::time::{Duration, Instant};
use itertools::Itertools;
use rayon::vec;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2022/day_7.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let fs = vec![Directory::new("root".to_string(), None)];
    let mut terminal_lines = lines.lines().skip(1).collect_vec();
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Directory {
    name: String,
    parent: Option<String>,
    files: Vec<File>,
    directories: Vec<Directory>,
    size: usize,
}

impl Directory {
    fn new(name: String, parent: Option<String>) -> Self {
        Self {
            name,
            parent,
            files: Vec::new(),
            directories: Vec::new(),
            size: 0,
        }
    }
    
    fn get_size(&mut self) {
        self.size = self.files.iter().map(|f| f.size).sum();
        self.directories.iter_mut().for_each(|d| d.get_size());
        self.size += self.directories.iter().map(|d| d.size).sum::<usize>();
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Command {
    ChangeDir(String),
    ChangeDirUp,
    ChangeDirRoot,
    ListDir,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Results {
    File(File),
    Directory(Directory),
}
