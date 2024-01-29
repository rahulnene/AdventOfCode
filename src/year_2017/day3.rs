use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use rustc_hash::FxHashMap;
pub fn solution() -> ((usize, Duration), (isize, Duration)) {
    let input = 368078;
    (solve01(input), solve02(input))
}

fn solve01(n: isize) -> (usize, Duration) {
    let now = Instant::now();
    let ans = position(n as usize - 1);
    (ans, now.elapsed())
}

fn solve02(input: isize) -> (isize, Duration) {
    let now = Instant::now();
    let mut grid = FxHashMap::default();
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut dir = 0;
    let mut sum = 1;
    grid.insert((x, y), sum);
    loop {
        println!("{} {} {}", x, y, sum);
        let (forward_dx, forward_dy) = match dir {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };
        x += forward_dx;
        y += forward_dy;
        let (left_dx, left_dy) = match dir {
            0 => (0, 1),
            1 => (-1, 0),
            2 => (0, -1),
            3 => (1, 0),
            _ => unreachable!(),
        };
        if grid.get(&(x + left_dx, y + left_dy)).is_none() {
            dir = (dir + 1) % 4;
        }
        sum = grid.get(&(x - 1, y - 1)).unwrap_or(&0)
            + grid.get(&(x - 1, y)).unwrap_or(&0)
            + grid.get(&(x - 1, y + 1)).unwrap_or(&0)
            + grid.get(&(x, y - 1)).unwrap_or(&0)
            + grid.get(&(x, y + 1)).unwrap_or(&0)
            + grid.get(&(x + 1, y - 1)).unwrap_or(&0)
            + grid.get(&(x + 1, y)).unwrap_or(&0)
            + grid.get(&(x + 1, y + 1)).unwrap_or(&0);
        grid.insert((x, y), sum);
        if sum > input {
            break;
        }
    }
    (sum, now.elapsed())
}

const fn first(cycle: usize) -> usize {
    (2 * cycle - 1).pow(2)
}

fn cycle(index: usize) -> usize {
    let sqrt = (index as f64).sqrt() as usize + 1;
    sqrt / 2
}

fn sector(index: usize) -> usize {
    let cycle = cycle(index);
    let offset = index - first(cycle);
    let n = 8 * cycle;
    4 * offset / n
}

fn position(index: usize) -> usize {
    let cycle = cycle(index);
    let sector = sector(index);
    let offset = index - first(cycle) - sector * (2 * cycle);
    cycle + cycle.abs_diff(offset + 1)
}
