use std::time::{Duration, Instant};

use fxhash::FxHashMap;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2015/day_18.txt");
    (solve(&lines, false), solve(&lines, true))
}

fn solve(lines: &str, is_part_2: bool) -> (usize, Duration) {
    let now = Instant::now();
    let mut grid = Grid::new();
    for (row_num, line) in lines.lines().enumerate() {
        for (col_num, c) in line.char_indices() {
            let light = Light::parse(c);
            grid.lights
                .insert((row_num as isize, col_num as isize), light);
        }
    }

    for _ in 0..100 {
        grid.update(is_part_2);
    }
    (grid.count_on(is_part_2), now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    lights: FxHashMap<(isize, isize), Light>,
}

impl Grid {
    fn new() -> Self {
        Self {
            lights: FxHashMap::default(),
        }
    }

    fn pprint(&self) {
        for row in 0..6 {
            for col in 0..6 {
                match self.get(row, col, false) {
                    Light::On => print!("#"),
                    Light::Off => print!("."),
                }
            }
            println!();
        }
        println!("----------")
    }

    fn count_on(&mut self, is_part_2: bool) -> usize {
        if is_part_2 {
            self.lights.insert((0, 0), Light::On);
            self.lights.insert((0, 99), Light::On);
            self.lights.insert((99, 0), Light::On);
            self.lights.insert((99, 99), Light::On);
        }
        self.lights.values().filter(|l| **l == Light::On).count()
    }

    fn get(&self, row: isize, col: isize, is_part_2: bool) -> &Light {
        if is_part_2 && (row == 0 || row == 99) && (col == 0 || col == 99) {
            return &Light::On;
        }
        self.lights.get(&(row, col)).or(Some(&Light::Off)).unwrap()
    }

    fn get_neighbor_count(&self, row: isize, col: isize, is_part_2: bool) -> usize {
        let mut count = 0;
        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                if r == row && c == col {
                    continue;
                }
                if *self.get(r, c, is_part_2) == Light::On {
                    count += 1;
                }
            }
        }
        count
    }

    fn update(&mut self, is_part_2: bool) {
        let mut new_lights = FxHashMap::default();
        for (row, col) in self.lights.keys() {
            let neighbor_count = self.get_neighbor_count(*row, *col, is_part_2);
            let light = self.get(*row, *col, is_part_2);
            let new_light = match light {
                Light::On => {
                    if neighbor_count == 2 || neighbor_count == 3 {
                        Light::On
                    } else {
                        Light::Off
                    }
                }
                Light::Off => {
                    if neighbor_count == 3 {
                        Light::On
                    } else {
                        Light::Off
                    }
                }
            };
            new_lights.insert((*row, *col), new_light);
        }
        self.lights = new_lights;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

impl Light {
    fn parse(c: char) -> Self {
        match c {
            '#' => Self::On,
            '.' => Self::Off,
            _ => panic!("Invalid light char: {}", c),
        }
    }

    fn flip(&mut self) {
        match self {
            Self::On => *self = Self::Off,
            Self::Off => *self = Self::On,
        }
    }
}
