use std::{
    iter::Cycle,
    slice::Iter,
    time::{Duration, Instant},
};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const LINES: &str = include_str!("../../problem_inputs_2022/day_23_test.txt");

pub fn solution() -> ((isize, Duration), (isize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (isize, Duration) {
    let now = Instant::now();
    let direction_to_check = [Direction::N, Direction::S, Direction::W, Direction::E]
        .iter()
        .cycle();
    let mut grid = FxHashSet::default();
    for (y, line) in LINES.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert((x as isize, y as isize));
            }
        }
    }
    let mut grid = Grid {
        elves: grid,
        consideration: direction_to_check,
    };
    grid.pprint();
    for _ in 0..10 {
        grid.update();
        grid.pprint();
    }
    println!("{}", grid.get_empty_cells());
    (0, now.elapsed())
}

fn solve02() -> (isize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

type Position = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Elf {
    pos: Position,
}

impl Elf {
    fn new(pos: &Position) -> Self {
        Self { pos: *pos }
    }
    fn move_in(&self, dir: Direction) -> Self {
        match dir {
            Direction::N => Self::new(&(self.pos.0, self.pos.1 - 1)),
            Direction::S => Self::new(&(self.pos.0, self.pos.1 + 1)),
            Direction::E => Self::new(&(self.pos.0 + 1, self.pos.1)),
            Direction::W => Self::new(&(self.pos.0 - 1, self.pos.1)),
        }
    }
}
#[derive(Debug, Clone)]
struct Grid<'a> {
    elves: FxHashSet<Position>,
    consideration: Cycle<Iter<'a, Direction>>,
}

impl<'a> Grid<'a> {
    fn is_elf_at(&self, pos: &Position) -> bool {
        self.elves.contains(pos)
    }

    fn get_bounds(&self) -> (isize, isize, isize, isize) {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for pos in &self.elves {
            min_x = min_x.min(pos.0);
            max_x = max_x.max(pos.0);
            min_y = min_y.min(pos.1);
            max_y = max_y.max(pos.1);
        }
        (min_x, max_x, min_y, max_y)
    }

    fn get_empty_cells(&self) -> usize {
        let (min_x, max_x, min_y, max_y) = self.get_bounds();
        let mut empty_cells = 0;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if !self.is_elf_at(&(x, y)) {
                    empty_cells += 1;
                }
            }
        }
        empty_cells
    }
    // fn has_neighbors(&self, pos: &Elf) -> bool {
    //     self.has_north_neighbors(pos)
    //         || self.has_south_neighbors(pos)
    //         || self.has_east_neighbors(pos)
    //         || self.has_west_neighbors(pos)
    // }
    fn has_neighbors_in(&self, pos: &Elf, direction: Direction) -> bool {
        match direction {
            Direction::N => self.has_north_neighbors(pos),
            Direction::S => self.has_south_neighbors(pos),
            Direction::E => self.has_east_neighbors(pos),
            Direction::W => self.has_west_neighbors(pos),
        }
    }
    fn has_north_neighbors(&self, elf: &Elf) -> bool {
        self.is_elf_at(&(elf.pos.0, elf.pos.1 - 1))
            || self.is_elf_at(&(elf.pos.0 - 1, elf.pos.1 - 1))
            || self.is_elf_at(&(elf.pos.0 + 1, elf.pos.1 - 1))
    }
    fn has_south_neighbors(&self, elf: &Elf) -> bool {
        self.is_elf_at(&(elf.pos.0, elf.pos.1 + 1))
            || self.is_elf_at(&(elf.pos.0 - 1, elf.pos.1 + 1))
            || self.is_elf_at(&(elf.pos.0 + 1, elf.pos.1 + 1))
    }
    fn has_east_neighbors(&self, elf: &Elf) -> bool {
        self.is_elf_at(&(elf.pos.0 + 1, elf.pos.1))
            || self.is_elf_at(&(elf.pos.0 + 1, elf.pos.1 - 1))
            || self.is_elf_at(&(elf.pos.0 + 1, elf.pos.1 + 1))
    }
    fn has_west_neighbors(&self, elf: &Elf) -> bool {
        self.is_elf_at(&(elf.pos.0 - 1, elf.pos.1))
            || self.is_elf_at(&(elf.pos.0 - 1, elf.pos.1 - 1))
            || self.is_elf_at(&(elf.pos.0 - 1, elf.pos.1 + 1))
    }

    fn update(&mut self) {
        let mut next_move_to_elves: FxHashMap<Position, Vec<Position>> = FxHashMap::default();
        let mut next_grid = FxHashSet::default();
        for pos in &self.elves {
            let mut moved = false;
            let elf = Elf::new(&pos);
            for dir in &self.consideration.by_ref().take(4).collect_vec() {
                if self.has_neighbors_in(&elf, **dir) || moved {
                    continue;
                }
                let next_pos = elf.move_in(**dir);
                next_move_to_elves
                    .entry(next_pos.pos)
                    .and_modify(|e: &mut Vec<Position>| e.push(elf.pos))
                    .or_insert_with(|| vec![elf.pos]);
                moved = true;
            }
            if !moved {
                next_move_to_elves
                    .entry(elf.pos)
                    .and_modify(|e: &mut Vec<Position>| e.push(elf.pos))
                    .or_insert_with(|| vec![elf.pos]);
            }
        }
        for (new_pos, old_pos_list) in next_move_to_elves {
            if old_pos_list.len() == 1 {
                next_grid.insert(new_pos);
            } else {
                for old_pos in old_pos_list {
                    next_grid.insert(old_pos);
                }
            }
        }
        assert_eq!(self.elves.len(), next_grid.len());
        self.consideration.next();
        self.elves = next_grid;
    }

    fn pprint(&self) {
        let (min_x, max_x, min_y, max_y) = self.get_bounds();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.is_elf_at(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("\n");
    }
}
