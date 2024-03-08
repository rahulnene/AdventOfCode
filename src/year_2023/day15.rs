use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2023/day_15.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let ans = LINES
        .split(',')
        .map(str::trim)
        .map(hash)
        .map(|n| n as usize)
        .sum::<usize>();
    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut boxes = BoxSet::new();
    let instrs = LINES
        .split(',')
        .map(str::trim)
        .map(Instruction::from_str)
        .collect::<Vec<_>>();
    for instr in &instrs {
        boxes.apply(instr);
    }
    (boxes.score(), now.elapsed())
}

#[inline]
fn hash(s: &str) -> u8 {
    let mut h: u8 = 0;
    for c in s.chars() {
        h = h.overflowing_add(c as u8).0;
        h = h.overflowing_add(h << 4).0;
        h &= 255;
    }
    h
}

type FocalLength = Option<u8>;
#[derive(Debug, Copy, Clone)]
enum Operation {
    Dash,
    Equals,
}

fn char_to_op(c: char) -> Operation {
    match c {
        '-' => Operation::Dash,
        '=' => Operation::Equals,
        _ => panic!("Invalid operation"),
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    label: String,
    op: Operation,
    focal_length: FocalLength,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let label_str: String = s.chars().take_while(|c| c.is_alphabetic()).collect();
        let op = char_to_op(s.chars().skip(label_str.len()).next().unwrap());
        let focal_length = s
            .chars()
            .skip(label_str.len() + 1)
            .collect::<String>()
            .parse()
            .ok();
        Self {
            label: label_str.to_string(),
            op,
            focal_length,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    label: String,
    focal_length: FocalLength,
}

impl Lens {
    fn new(label: &str, focal_length: FocalLength) -> Self {
        Self {
            label: label.to_string(),
            focal_length,
        }
    }
}
#[derive(Debug, Clone)]
struct Box {
    contents: Vec<Lens>,
}

impl Box {
    fn new() -> Self {
        Self {
            contents: Vec::new(),
        }
    }
    fn insert(&mut self, lens: Lens) {
        if let Some(pos) = self.contents.iter().position(|x| x.label == lens.label) {
            self.contents[pos] = lens;
        } else {
            self.contents.push(lens);
        }
    }

    fn remove(&mut self, lens: Lens) {
        if let Some(pos) = self.contents.iter().position(|x| *x.label == lens.label) {
            self.contents.remove(pos);
        }
    }
}
#[derive(Debug, Clone)]
struct BoxSet {
    boxes: Vec<Box>,
}
impl BoxSet {
    fn new() -> Self {
        Self {
            boxes: vec![Box::new(); 256],
        }
    }

    fn apply(&mut self, instr: &Instruction) {
        let lens = Lens::new(&instr.label, instr.focal_length);
        let label_hash = hash(&instr.label) as usize;
        match instr.op {
            Operation::Dash => {
                self.boxes[label_hash].remove(lens);
            }
            Operation::Equals => {
                self.boxes[label_hash].insert(lens);
            }
        }
    }
    fn score(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .filter(|(_, b)| !b.contents.is_empty())
            .flat_map(|(box_num, b)| {
                b.contents.iter().enumerate().map(move |(lens_pos, lens)| {
                    (box_num + 1) * (lens_pos + 1) * lens.focal_length.unwrap() as usize
                })
            })
            .sum()
    }

    fn pprint(&self) {
        for (i, b) in self
            .boxes
            .iter()
            .enumerate()
            .filter(|b| b.1.contents.len() > 0)
        {
            print!("Box {}: ", i);
            for l in &b.contents {
                print!("[{:?} {:?}]  ", l.label, l.focal_length.unwrap());
            }
            println!();
        }
        println!();
    }
}
