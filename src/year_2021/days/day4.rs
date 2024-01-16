use std::time::{Duration, Instant};

use fxhash::FxHashMap;
use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2021/day_4.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let a = lines.split("\r\n\r\n").collect_vec();
    let called_nums = a[0]
        .trim()
        .split(",")
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
    for num in called_nums {
        for board in boards.iter_mut() {
            board.mark(num);
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

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let a = lines.split("\r\n\r\n").collect_vec();
    let called_nums = a[0]
        .trim()
        .split(",")
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
    for num in &called_nums {
        for i in 0..boards.len() {
            boards[i].mark(*num);
            if boards[i].check_win() {
                boards[i].won = true;
            }
        }
        if boards.iter().filter(|b: &&BingoBoard| !b.won).count() == 1 {
            let num_pos = called_nums.iter().position(|&n| n == *num).unwrap();
            let num = called_nums[num_pos + 1];
            dbg!(num);
            let score = boards
                .iter()
                .find(|b: &&BingoBoard| !b.won)
                .unwrap()
                .numbers
                .iter()
                .filter(|c| !c.1.is_marked)
                .map(|c| c.1.number)
                .sum::<usize>();
            dbg!(score);
            dbg!(score*num);
            return ((score - num) * num, now.elapsed());
        }
    }
    unreachable!("No winner found")
}

#[derive(Debug, Clone)]
struct BingoBoard {
    id: usize,
    won: bool,
    numbers: FxHashMap<(usize, usize), Cell>,
}

impl BingoBoard {
    fn new(id: usize) -> Self {
        BingoBoard {
            id,
            won: false,
            numbers: FxHashMap::default(),
        }
    }

    fn mark(&mut self, number: usize) {
        for (_, cell) in self.numbers.iter_mut() {
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
