use std::time::{Duration, Instant};

use itertools::Itertools;
use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2021/day_4.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let a = LINES.split("\r\n\r\n").collect_vec();
    let called_nums = a[0]
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let mut boards = Vec::with_capacity(a.len() - 1);
    for (board_num, board) in a[1..].iter().enumerate() {
        let mut b = BingoBoard::new(board_num);
        for (i, line) in board.trim().split("\r\n").enumerate() {
            for (j, num) in line.split_whitespace().enumerate() {
                b.numbers
                    .insert((i, j), Cell::new(num.parse::<usize>().unwrap()));
            }
        }
        boards.push(b);
    }
    (
        solve01(&boards, &called_nums),
        solve02(&boards, &called_nums),
    )
}

fn solve01(boards: &[BingoBoard], called_nums: &[usize]) -> (usize, Duration) {
    let mut boards = boards.to_owned();
    let now = Instant::now();
    for num in called_nums {
        for board in &mut boards {
            board.mark(*num);
        }
        for board in boards.clone() {
            if board.check_win() {
                let score = board
                    .numbers
                    .iter()
                    .filter(|c| !c.1.is_marked)
                    .map(|c| c.1.number)
                    .sum::<usize>()
                    * num;
                return (score, now.elapsed());
            }
        }
    }
    unreachable!("No winner found")
}

fn solve02(boards: &[BingoBoard], called_nums: &[usize]) -> (usize, Duration) {
    let mut boards = boards.to_owned();
    let now = Instant::now();
    let mut removed_boards = Vec::new();
    for num in called_nums {
        for board in &mut boards {
            board.mark(*num);
        }
        for board in boards.clone() {
            if removed_boards.contains(&board.id) {
                continue;
            }
            if board.check_win() {
                removed_boards.push(board.id);
            }
        }
        if removed_boards.len() == boards.len() {
            let last_removed = boards
                .iter()
                .find(|b| b.id == *removed_boards.last().unwrap())
                .unwrap();
            let score = last_removed
                .numbers
                .iter()
                .filter(|c| !c.1.is_marked)
                .map(|c| c.1.number)
                .sum::<usize>();
            return (score * num, now.elapsed());
        }
    }

    (0, now.elapsed())
}

#[derive(Debug, Clone)]
struct BingoBoard {
    id: usize,
    numbers: FxHashMap<(usize, usize), Cell>,
}

impl BingoBoard {
    fn new(id: usize) -> Self {
        BingoBoard {
            id,
            numbers: FxHashMap::default(),
        }
    }

    fn mark(&mut self, number: usize) {
        for cell in &mut self.numbers.values_mut() {
            if cell.number == number {
                cell.is_marked = true;
            }
        }
    }

    fn check_win(&self) -> bool {
        for row in 0..5 {
            if (0..5).all(|c| self.numbers.get(&(row, c)).unwrap().is_marked) {
                return true;
            }
        }
        for col in 0..5 {
            if (0..5).all(|r| self.numbers.get(&(r, col)).unwrap().is_marked) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
struct Cell {
    number: usize,
    is_marked: bool,
}

impl Cell {
    fn new(number: usize) -> Self {
        Self {
            number,
            is_marked: false,
        }
    }
}
