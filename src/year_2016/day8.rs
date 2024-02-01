use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2016/day_8.txt");
    (solve01(&lines), solve02())
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut tv = TV::new();
    let instrs = lines.lines().map(Instruction::from_str).collect_vec();
    for instr in instrs {
        tv.apply_instr(&instr);
    }
    tv.pretty_print();
    (tv.count_lit(), now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone)]
struct TV {
    screen: VecDeque<VecDeque<bool>>,
}

impl TV {
    fn new() -> Self {
        let mut screen = VecDeque::with_capacity(6);
        for _ in 0..6 {
            let mut row = VecDeque::with_capacity(50);
            for _ in 0..50 {
                row.push_back(false);
            }
            screen.push_back(row);
        }
        Self { screen }
    }

    fn pretty_print(&self) {
        for row in self.screen.iter() {
            for pixel in row.iter() {
                print!("{} ", if *pixel { '#' } else { '.' });
            }
            println!();
        }
    }

    fn count_lit(&self) -> usize {
        self.screen.iter().flatten().filter(|&&pixel| pixel).count()
    }

    fn apply_instr(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Rect(a, b) => {
                for row in self.screen.iter_mut().take(*b) {
                    for pixel in row.iter_mut().take(*a) {
                        *pixel = true;
                    }
                }
            }
            Instruction::RotateRow(r, amount) => {
                let mut row = self.screen[*r].clone();
                row.rotate_right(*amount);
                self.screen[*r] = row;
            }
            Instruction::RotateColumn(c, amount) => {
                let mut col = VecDeque::with_capacity(6);
                for row in self.screen.iter() {
                    col.push_back(row[*c]);
                }
                col.rotate_right(*amount);
                for (i, row) in self.screen.iter_mut().enumerate() {
                    row[*c] = col[i];
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut words = s.split_ascii_whitespace();
        match words.next().unwrap() {
            "rect" => {
                let mut nums = words.next().unwrap().split('x');
                let a = nums.next().unwrap().parse::<usize>().unwrap();
                let b = nums.next().unwrap().parse::<usize>().unwrap();
                Self::Rect(a, b)
            }
            "rotate" => {
                let mut nums = words.clone().nth(1).unwrap().split('=');
                let _ = nums.next();
                let a = nums.next().unwrap().parse::<usize>().unwrap();
                let b = words.clone().last().unwrap().parse::<usize>().unwrap();
                match words.next().unwrap() {
                    "row" => Self::RotateRow(a, b),
                    "column" => Self::RotateColumn(a, b),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
