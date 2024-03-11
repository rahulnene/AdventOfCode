use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2022/day_17_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let wind_iter = Box::new(LINES.chars().map(WindDirection::from_char).cycle());
    let mut tetris = Tetris {
        grid: Vec::new(),
        current_block_type: BlockType::Horizontal,
        block_generator: BlockGenerator {
            current_block: BlockType::Horizontal,
        },
        current_block_position: (0, 0),
        wind_iter,
    };
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Lateral,
    Vertical,
}

struct BlockGenerator {
    current_block: BlockType,
}

impl BlockGenerator {
    fn next_block(&mut self) -> BlockType {
        self.current_block = match self.current_block {
            BlockType::Horizontal => BlockType::Plus,
            BlockType::Plus => BlockType::L,
            BlockType::L => BlockType::Vertical,
            BlockType::Vertical => BlockType::Square,
            BlockType::Square => BlockType::Horizontal,
        };
        return self.current_block;
    }
}

#[derive(Debug, Clone, Copy)]
enum WindDirection {
    Left,
    Right,
}

impl WindDirection {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum BlockType {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square,
}
struct Tetris {
    grid: Vec<[bool; 7]>,
    current_block_type: BlockType,
    current_block_position: (usize, usize),
    block_generator: BlockGenerator,
    wind_iter: Box<dyn Iterator<Item = WindDirection>>,
}
