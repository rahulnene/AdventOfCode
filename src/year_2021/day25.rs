use std::fmt::Debug;

use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_25.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut m = Map::new(lines);
    let mut steps = 0;
    loop {
        if !m.step() {
            break;
        }
        steps += 1;
    }

    steps + 1
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Clone)]
struct Map {
    map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(lines: &str) -> Self {
        let mut map: Vec<Vec<u8>> = Vec::new();
        for line in lines.lines() {
            map.push(
                line.chars()
                    .map(|f| match f {
                        '.' => 0,
                        '>' => 1,
                        'v' => 2,
                        _ => panic!("Invalid input"),
                    })
                    .collect_vec(),
            );
        }
        let height = map.len();
        let width = map[0].len();
        Map { map, width, height }
    }

    fn get_sc(&self, row: isize, col: isize) -> u8 {
        self.map[row.rem_euclid(self.height as isize) as usize]
            [col.rem_euclid(self.width as isize) as usize]
    }

    fn set_sc(&mut self, row: isize, col: isize, val: u8) {
        self.map[row.rem_euclid(self.height as isize) as usize]
            [col.rem_euclid(self.width as isize) as usize] = val;
    }

    fn step(&mut self) -> bool {
        let mut any_moved = false;
        let mut new_map = self.clone();
        for row in 0..self.height as isize {
            for col in 0..self.width as isize {
                match self.get_sc(row, col) {
                    0 => (),
                    1 => {
                        if self.get_sc(row, col + 1) == 0 {
                            new_map.set_sc(row, col + 1, 1);
                            new_map.set_sc(row, col, 0);
                            any_moved = true;
                        }
                    }
                    2 => (),
                    _ => panic!("Invalid state"),
                }
            }
        }
        *self = new_map;
        let mut new_map = self.clone();
        for row in 0..self.height as isize {
            for col in 0..self.width as isize {
                match self.get_sc(row, col) {
                    0 => (),
                    1 => (),
                    2 => {
                        if self.get_sc(row + 1, col) == 0 {
                            new_map.set_sc(row + 1, col, 2);
                            new_map.set_sc(row, col, 0);
                            any_moved = true;
                        }
                    }
                    _ => panic!("Invalid state"),
                }
            }
        }
        *self = new_map;
        any_moved
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Width: {}, Height: {}", self.width, self.height)?;
        writeln!(f)?;
        for line in &self.map {
            for c in line {
                match c {
                    0 => write!(f, ".")?,
                    1 => write!(f, ">")?,
                    2 => write!(f, "v")?,
                    _ => panic!("Invalid input"),
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
