use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    vec,
};

use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_13.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let (coord, instr) = lines.split("\n\n").collect_tuple().unwrap();
    let coords = coord.split('\n').collect_vec();
    let mut instructions: Vec<(String, String)> = Vec::new();
    for i in instr.lines() {
        let (_, _, actual) = i.split(' ').collect_tuple().unwrap();
        let (axis, loc) = actual
            .split('=')
            .map(|f| f.to_string())
            .collect_tuple()
            .unwrap();
        instructions.append(&mut vec![(axis, loc)]);
    }
    // dbg!(&coords);
    let (largest_x, largest_y) = coords.iter().fold((0, 0), |(max_x, max_y), c| {
        let (x, y) = c
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        (max_x.max(x), max_y.max(y))
    });
    // dbg!(largest_x, largest_y);
    let mut paper = Paper::new(largest_x + 1, largest_y + 1);
    for c in coords {
        paper.insert_dot_str(c);
    }
    // dbg!(&paper);
    // dbg!(&paper.x_size, &paper.y_size);
    // dbg!(paper.count());
    for i in instructions {
        match i.0.as_str() {
            "x" => paper.fold_x(i.1.parse::<usize>().unwrap()),
            "y" => paper.fold_y(i.1.parse::<usize>().unwrap()),
            _ => panic!("Invalid axis"),
        }
        // dbg!(&paper);
        dbg!(&paper.x_size, &paper.y_size);
        dbg!(paper.count());
    }
    dbg!(paper);
    0
}

fn solve02(lines: &str) -> usize {
    0
}

type Coord = (usize, usize);

#[derive(Clone)]
struct Paper {
    x_size: usize,
    y_size: usize,
    grid: HashMap<Coord, bool>,
}

impl Paper {
    fn new(x_size: usize, y_size: usize) -> Self {
        let false_map: HashMap<Coord, bool> = (0..=x_size)
            .cartesian_product(0..=y_size)
            .map(|(x, y)| ((x, y), false))
            .collect();
        Paper {
            x_size,
            y_size,
            grid: false_map,
        }
    }

    fn insert_dot_str(&mut self, dot_str: &str) {
        let (x, y) = dot_str.split(',').collect_tuple().unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        self.insert_dot(x, y);
    }

    fn insert_dot(&mut self, x: usize, y: usize) {
        self.grid.insert((x, y), true);
    }

    fn fold_x(&mut self, x: usize) {
        for y in 0..=self.y_size {
            for x in 0..=x {
                self.grid.insert(
                    (x, y),
                    *self.grid.get(&(x, y)).unwrap()
                        || *self.grid.get(&(self.x_size - 1 - x, y)).unwrap(),
                );
            }
        }
        for item in self.grid.iter_mut() {
            if item.0 .0 >= x {
                *item.1 = false;
            }
        }
        self.x_size /= 2;
        if self.x_size % 2 == 0 {
            self.x_size += 1;
        }
    }

    fn fold_y(&mut self, y: usize) {
        for x in 0..=self.x_size {
            for y in 0..=y {
                self.grid.insert(
                    (x, y),
                    *self.grid.get(&(x, y)).unwrap()
                        || *self.grid.get(&(x, self.y_size - y - 1)).unwrap(),
                );
            }
        }
        for item in self.grid.iter_mut() {
            if item.0 .1 >= y {
                *item.1 = false;
            }
        }
        self.y_size /= 2;
        if self.y_size % 2 == 0 {
            self.y_size += 1;
        }
    }

    fn count(&self) -> usize {
        self.grid.values().filter(|&&v| v).count()
    }
}

impl Debug for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                write!(
                    f,
                    "{}",
                    if *self.grid.get(&(x, y)).unwrap_or(&false) {
                        '*'
                    } else {
                        ' '
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
