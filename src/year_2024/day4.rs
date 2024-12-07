use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

pub fn solution(test: bool) -> ((usize, Duration), (usize, Duration)) {
    let lines;
    if test {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_4_test.txt");
    } else {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_4.txt");
    }
    let word_search = WordSearch::from_str(lines);
    (solve01(&word_search), solve02(&word_search))
}

fn solve01(word_search: &WordSearch) -> (usize, Duration) {
    let now = Instant::now();
    let ans = word_search.search("XMAS");
    (ans, now.elapsed())
}

fn solve02(word_search: &WordSearch) -> (usize, Duration) {
    let now = Instant::now();
    let ans = word_search.cross_mas_search();
    (ans, now.elapsed())
}

type RowIndex = isize;
type ColIndex = isize;

#[derive(Debug, Clone)]
struct WordSearch {
    letters: FxHashMap<(RowIndex, ColIndex), char>,
    bounds: (RowIndex, ColIndex),
}

impl WordSearch {
    fn from_str(input: &str) -> Self {
        let mut letters = FxHashMap::default();
        for (row, line) in input.lines().enumerate() {
            for (col, letter) in line.chars().enumerate() {
                letters.insert((row as RowIndex, col as ColIndex), letter);
            }
        }
        Self {
            letters,
            bounds: (
                input.lines().count() as isize,
                input.lines().next().unwrap().chars().count() as isize,
            ),
        }
    }

    fn search_at(&self, row_ind: RowIndex, col_ind: ColIndex, word: &str) -> usize {
        let directions = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ];

        directions
            .iter()
            .map(|dir| {
                dir.get_indices(row_ind, col_ind, word.len())
                    .iter()
                    .map(|&(row, col)| self.letters.get(&(row, col)))
                    .collect::<Vec<Option<&char>>>()
            })
            .filter(|letters| {
                letters.iter().all(Option::is_some)
                    && letters
                        .into_iter()
                        .flatten()
                        .map(|c| **c)
                        .collect::<String>()
                        == word
            })
            .count()
    }

    fn search(&self, word: &str) -> usize {
        (0..self.bounds.0)
            .flat_map(|row| (0..self.bounds.1).map(move |col| self.search_at(row, col, word)))
            .sum()
    }

    fn convert_indices_to_vec(
        &self,
        indices: impl Iterator<Item = (i32, i32)>,
    ) -> Vec<Option<&char>> {
        indices
            .map(|(row, col)| self.letters.get(&(row as isize, col as isize)))
            .collect::<Vec<Option<&char>>>()
    }

    fn cross_mas_search(&self) -> usize {
        let mut count = 0;
        for row in 0..self.bounds.0 {
            for col in 0..self.bounds.1 {
                if self.letters.get(&(row, col)) == Some(&'A') {
                    let cross_letters = vec![
                        self.letters.get(&(row - 1, col - 1)),
                        self.letters.get(&(row - 1, col + 1)),
                        self.letters.get(&(row + 1, col - 1)),
                        self.letters.get(&(row + 1, col + 1)),
                    ];
                    if cross_letters
                        .iter()
                        .all(|&c| c.is_some() && (c.unwrap() == &'S' || c.unwrap() == &'M'))
                    {
                        let cross_word = cross_letters
                            .iter()
                            .map(|&c| c.unwrap())
                            .collect::<String>();
                        if cross_word == "SMMS" || cross_word == "MSSM" {
                        } else {
                            let counts = cross_word.chars().counts();
                            if counts.get(&'S') == Some(&2) && counts.get(&'M') == Some(&2) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }
}

fn check_letters(letters: Vec<Option<&char>>, word: &str) -> bool {
    letters.iter().all(Option::is_some) && letters.into_iter().flatten().collect::<String>() == word
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    UpLeft,
    UpRight,
    Down,
    DownLeft,
    DownRight,
    Left,
    Right,
}

impl Direction {
    fn get_indices(&self, row: RowIndex, col: ColIndex, word_len: usize) -> Vec<(isize, isize)> {
        let word_len = word_len as isize;
        match self {
            Direction::Up => (0..word_len).map(|i| (row - i, col)).collect(),
            Direction::UpLeft => (0..word_len).map(|i| (row - i, col - i)).collect(),
            Direction::UpRight => (0..word_len).map(|i| (row - i, col + i)).collect(),
            Direction::Down => (0..word_len).map(|i| (row + i, col)).collect(),
            Direction::DownLeft => (0..word_len).map(|i| (row + i, col - i)).collect(),
            Direction::DownRight => (0..word_len).map(|i| (row + i, col + i)).collect(),
            Direction::Left => (0..word_len).map(|i| (row, col - i)).collect(),
            Direction::Right => (0..word_len).map(|i| (row, col + i)).collect(),
        }
    }
}
