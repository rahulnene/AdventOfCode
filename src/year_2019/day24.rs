use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::time::{Duration, Instant};
use your_game_of_life::{Cell, CellNeighbors, Life};

const ALIVE: Cell = Cell::alive();
const DEAD: Cell = Cell::dead();
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let init = include_str!("../../problem_inputs_2019/day_24.txt");
    (solve01(init), solve02(init))
}

fn solve01(init: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut life = from_str(init);
    let mut seen = FxHashSet::default();
    seen.insert(life);
    loop {
        life.play(|same, others, _, _| {
            let alive = others.top().is_alive() as usize
                + others.bottom().is_alive() as usize
                + others.left().is_alive() as usize
                + others.right().is_alive() as usize;
            match same {
                ALIVE => {
                    if alive == 1 {
                        same
                    } else {
                        DEAD
                    }
                }
                DEAD => {
                    if alive == 1 || alive == 2 {
                        ALIVE
                    } else {
                        same
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        });
        if seen.contains(&life) {
            break;
        }
        seen.insert(life.clone());
    }
    // for line in life.cells.iter() {
    //     for cell in line {
    //         if cell == &ALIVE {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    (score(&life), now.elapsed())
}

fn solve02(init: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn from_str(s: &str) -> Life<5, 5> {
    let cells = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => ALIVE,
                    '.' => DEAD,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let mut life: Life<5, 5> = Life::from(cells);
    life.out_of_bounds = DEAD;
    life
}

fn score(life: &Life<5, 5>) -> usize {
    let mut score = 0;
    for (y, line) in life.cells.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if cell == &ALIVE {
                score += 2usize.pow((y * 5 + x) as u32);
            }
        }
    }
    score
}
