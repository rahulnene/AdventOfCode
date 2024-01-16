use std::time::Instant;

use itertools::Itertools;

pub fn solution(part: u8) -> Vec<usize> {
    let serial_number = 2866;

    let grid = Grid::new(serial_number);
    match part {
        1 => {
            let ans = solve_search_size(&grid, 3).0;
            vec![ans.0, ans.1]
        }
        2 => {
            let ans = max_submatrix(&grid.cells);
            vec![ans.0 .0, ans.0 .1, ans.2]
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
    // println!("{:?}", Instant::now() - now);
    (max_top_left, max_pow)
}

fn solve_generic(grid: &Grid) -> ((usize, usize), usize) {
    // Vec of (((tlx, tly), power), searchsize)
    let a = (0..100)
        .map(|s| (solve_search_size(grid, s), s))
        .collect_vec();
    let (point, search_size) = *a.iter().max_by_key(|f| f.0 .1).unwrap();
    dbg!(point.1);
    (point.0, search_size)
}

type Point = (usize, usize);

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<isize>>,
}

impl Grid {
    fn new(serial: usize) -> Self {
        let mut cells: Vec<Vec<isize>> = vec![vec![0; 301]; 301];
        for x in 1..=300 {
            for y in 1..=300 {
                let rack_id = x + 10;
                let mut power = (rack_id * y + serial) as isize;
                power *= rack_id as isize;
                power /= 100;
                power %= 10;
                power -= 5;
                cells[x][y] = power;
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
                sum += self.cells[coord.0][coord.1];
            }
        }
        sum
    }
}

fn max_submatrix(matrix: &Vec<Vec<isize>>) -> ((usize, usize), isize, usize) {
    let n = matrix.len();
    let mut max_sum = std::isize::MIN;
    let mut top_left = (0, 0);
    let mut size = 0;

    let mut sum_matrix = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            sum_matrix[i][j] = matrix[i - 1][j - 1] + sum_matrix[i - 1][j] + sum_matrix[i][j - 1]
                - sum_matrix[i - 1][j - 1];
        }
    }

    for len in 1..=n {
        for i in len..=n {
            for j in len..=n {
                let sum = sum_matrix[i][j] - sum_matrix[i - len][j] - sum_matrix[i][j - len]
                    + sum_matrix[i - len][j - len];
                if sum > max_sum {
                    max_sum = sum;
                    top_left = (i - len, j - len);
                    size = len;
                }
            }
        }
    }

    (top_left, max_sum, size)
}
