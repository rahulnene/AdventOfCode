use itertools::Itertools;
use lazy_static::lazy_static;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
lazy_static! {
    static ref INSTRUCTIONS: Vec<Instruction> =
        include_str!("../../problem_inputs_2016/day_21.txt")
            .lines()
            .map(Instruction::from_str)
            .collect::<Vec<_>>();
    static ref SEED: String = "abcdefgh".to_string();
    static ref SCRAMBLED: String = "fbgdceah".to_string();
}

pub fn solution() -> ((String, Duration), (String, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (String, Duration) {
    let now = Instant::now();
    let mut password = SEED.to_string();
    for instr in INSTRUCTIONS.iter() {
        password = apply_instr(&instr, &password);
    }
    (password, now.elapsed())
}

fn solve02() -> (String, Duration) {
    let now = Instant::now();
    let mut password = SCRAMBLED.to_string();
    for instr in INSTRUCTIONS.iter().rev() {
        password = invert_instr(&instr, &password);
    }
    (password, now.elapsed())
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let words: Vec<_> = s.split_whitespace().collect();
        match words[0] {
            "swap" => match words[1] {
                "position" => {
                    let x = words[2].parse().unwrap();
                    let y = words[5].parse().unwrap();
                    Self::SwapPosition(x, y)
                }
                "letter" => {
                    let x = words[2].chars().next().unwrap();
                    let y = words[5].chars().next().unwrap();
                    Self::SwapLetter(x, y)
                }
                _ => panic!("Invalid swap instruction"),
            },
            "rotate" => match words[1] {
                "left" => {
                    let x = words[2].parse().unwrap();
                    Self::RotateLeft(x)
                }
                "right" => {
                    let x = words[2].parse().unwrap();
                    Self::RotateRight(x)
                }
                "based" => {
                    let x = words[6].chars().next().unwrap();
                    Self::RotateLetter(x)
                }
                _ => panic!("Invalid rotate instruction"),
            },
            "reverse" => {
                let x = words[2].parse().unwrap();
                let y = words[4].parse().unwrap();
                Self::Reverse(x, y)
            }
            "move" => {
                let x = words[2].parse().unwrap();
                let y = words[5].parse().unwrap();
                Self::Move(x, y)
            }
            _ => panic!("Invalid instruction"),
        }
    }
}

fn apply_instr(instr: &Instruction, seed: &str) -> String {
    let seed = seed.chars().collect::<VecDeque<_>>();
    match instr {
        Instruction::SwapPosition(pos1, pos2) => {
            let mut new_seed = seed.clone();
            let pos1 = *pos1 % new_seed.len();
            let pos2 = *pos2 % new_seed.len();
            new_seed[pos1] = seed[pos2];
            new_seed[pos2] = seed[pos1];
            new_seed.iter().collect()
        }
        Instruction::SwapLetter(c1, c2) => {
            let mut new_seed = seed.clone();
            let pos1 = seed.iter().position(|c| *c == *c1).unwrap();
            let pos2 = seed.iter().position(|c| *c == *c2).unwrap();
            new_seed[pos1] = seed[pos2];
            new_seed[pos2] = seed[pos1];
            new_seed.iter().collect()
        }
        Instruction::RotateLeft(amount) => {
            let mut new_seed = seed.clone();
            new_seed.rotate_left(*amount % new_seed.len());
            new_seed.iter().collect()
        }
        Instruction::RotateRight(amount) => {
            let mut new_seed = seed.clone();
            new_seed.rotate_right(*amount % new_seed.len());
            new_seed.iter().collect()
        }
        Instruction::RotateLetter(letter) => {
            let pos = seed.iter().position(|c| *c == *letter).unwrap();
            let amount = if pos >= 4 { pos + 2 } else { pos + 1 };
            let amount = amount % seed.len();
            let mut new_seed = seed.clone();
            new_seed.rotate_right(amount);
            new_seed.iter().collect()
        }
        Instruction::Reverse(l1, l2) => {
            let mut new_seed = seed.clone();
            let to_reverse = new_seed.drain(*l1..=*l2).rev().collect_vec();
            for (i, c) in to_reverse.into_iter().enumerate() {
                new_seed.insert(*l1 + i, c);
            }
            new_seed.iter().collect()
        }
        Instruction::Move(pos1, pos2) => {
            let mut new_seed = seed.clone();
            let c = new_seed.remove(*pos1).unwrap();
            new_seed.insert(*pos2, c);
            new_seed.iter().collect()
        }
    }
}

fn invert_instr(instr: &Instruction, seed: &str) -> String {
    let seed = seed.chars().collect::<VecDeque<_>>();
    match instr {
        Instruction::SwapPosition(pos1, pos2) => {
            let mut new_seed = seed.clone();
            let pos1 = *pos1 % new_seed.len();
            let pos2 = *pos2 % new_seed.len();
            new_seed[pos1] = seed[pos2];
            new_seed[pos2] = seed[pos1];
            new_seed.iter().collect()
        }
        Instruction::SwapLetter(c1, c2) => {
            let mut new_seed = seed.clone();
            let pos1 = seed.iter().position(|c| *c == *c1).unwrap();
            let pos2 = seed.iter().position(|c| *c == *c2).unwrap();
            new_seed[pos1] = seed[pos2];
            new_seed[pos2] = seed[pos1];
            new_seed.iter().collect()
        }
        Instruction::RotateLeft(amount) => {
            let mut new_seed = seed.clone();
            new_seed.rotate_right(*amount % new_seed.len());
            new_seed.iter().collect()
        }
        Instruction::RotateRight(amount) => {
            let mut new_seed = seed.clone();
            new_seed.rotate_left(*amount % new_seed.len());
            new_seed.iter().collect()
        }
        Instruction::RotateLetter(letter) => {
            let current_pos = seed.iter().position(|c| *c == *letter).unwrap();
            let seed_len = seed.len();
            let mut original_pos = 0;

            for pos in 0..seed_len {
                let rotation = if pos >= 4 { pos + 2 } else { pos + 1 };
                if (pos + rotation) % seed_len == current_pos {
                    original_pos = pos;
                    break;
                }
            }

            let amount = (current_pos as isize - original_pos as isize)
                .rem_euclid(seed_len as isize) as usize;
            let mut new_seed = seed.clone();
            new_seed.rotate_left(amount);
            new_seed.iter().collect()
        }
        Instruction::Reverse(l1, l2) => {
            let mut new_seed = seed.clone();
            let to_reverse = new_seed.drain(*l1..=*l2).rev().collect_vec();
            for (i, c) in to_reverse.into_iter().enumerate() {
                new_seed.insert(*l1 + i, c);
            }
            new_seed.iter().collect()
        }
        Instruction::Move(pos1, pos2) => {
            let mut new_seed = seed.clone();
            let c = new_seed.remove(*pos2).unwrap();
            new_seed.insert(*pos1, c);
            new_seed.iter().collect()
        }
    }
}
