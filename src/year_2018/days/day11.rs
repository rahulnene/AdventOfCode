use std::time::Instant;

use fxhash::FxHashMap;
use itertools::Itertools;

pub fn solution(part: u8) -> Vec<usize> {
    let serial_number = 18;

    let grid = Grid::new(serial_number);
    match part {
        1 => {
            let ans = solve_search_size(&grid, 3).0;
            vec![ans.0, ans.1]
        }
        2 => {
            let ans = solve_generic(&grid);
            vec![ans.0 .0, ans.0 .1, ans.1]
        }
        _ => Vec::new(),
    }
}

fn solve_search_size(grid: &Grid, search_size: usize) -> ((usize, usize), isize) {
    let now = Instant::now();
    let mut max_pow = 0;
    let mut max_top_left = (0, 0);
    let mut top_left;
    for cx in 1..(300 - search_size + 2) {
        for cy in 1..(300 - search_size + 2) {
            top_left = (cx, cy);
            let power = grid.get_square_sum(top_left, search_size);
            if power > max_pow {
                max_pow = power;
                max_top_left = top_left;
            }
        }
    }
    println!("{:?}", Instant::now() - now);
    (max_top_left, max_pow)
}

fn solve_generic(grid: &Grid) -> ((usize, usize), usize) {
    // Vec of (((tlx, tly), power), searchsize)
    let a = (1..300)
        .map(|s| (solve_search_size(grid, s), s))
        .collect_vec();
    let (point, search_size) = *a.iter().max_by_key(|f| f.0 .1).unwrap();
    (point.0, search_size)
}

type Point = (usize, usize);

#[derive(Debug, Clone)]
struct Grid {
    cells: FxHashMap<Point, isize>,
}

impl Grid {
    fn new(serial: usize) -> Self {
        let mut cells = FxHashMap::default();
        for x in 1..=300 {
            for y in 1..=300 {
                let rack_id = x + 10;
                let mut power = (rack_id * y + serial) as isize;
                power *= rack_id as isize;
                power /= 100;
                power %= 10;
                power -= 5;
                cells.insert((x, y), power);
            }
        }
        Grid { cells }
    }

    fn get_square_sum(&self, top_left: Point, search_size: usize) -> isize {
        let (tl_x, tl_y) = top_left;
        let mut sum = 0;
        for x in 0..search_size {
            for y in 0..search_size {
                let coord = (tl_x + x, tl_y + y);
                sum += *self.cells.get(&coord).unwrap();
            }
        }
        sum
    }
}
