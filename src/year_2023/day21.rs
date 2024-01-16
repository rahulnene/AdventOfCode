use fxhash::FxHashMap;
use itertools::Itertools;

use std::{collections::HashMap, fmt::Debug, ops::Add};

pub fn solution(part: u16) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_21_test.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let players = lines.split('\n').collect_vec();
    let mut player1 = Player::from_str(players[0]);
    let mut player2 = Player::from_str(players[1]);
    let mut die = DetDice::new();
    loop {
        player1.play(&mut die);
        if player1.has_won(1000) {
            return player2.score * die.rolls;
        }
        player2.play(&mut die);
        if player2.has_won(1000) {
            return player1.score * die.rolls;
        }
    }
}

#[derive(Clone, Copy)]
struct Player {
    id: u16,
    pos: u16,
    score: usize,
}

impl Player {
    fn from_str(line: &str) -> Self {
        {
            let id = line[7..8].parse().unwrap();
            let pos = line[9..]
                .trim_start_matches("starting position: ")
                .parse()
                .unwrap();
            Self { id, pos, score: 0 }
        }
    }

    fn move_pawn(&mut self, val: usize) {
        self.pos += val as u16;
        self.pos %= 10;
        if self.pos == 0 {
            self.pos = 10;
        }
        self.score += self.pos as usize;
    }

    fn has_won(&self, winning_score: usize) -> bool {
        self.score >= winning_score
    }

    fn play(&mut self, die: &mut DetDice) {
        let roll1 = die.roll();
        let roll2 = die.roll();
        let roll3 = die.roll();
        self.move_pawn(roll1 + roll2 + roll3);
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player {} has {} points and is at {}",
            self.id, self.score, self.pos
        )?;
        writeln!(f)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct DetDice {
    rolls: usize,
}
impl DetDice {
    fn new() -> Self {
        Self { rolls: 0 }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;
        self.rolls % 100
    }
}

fn solve02(lines: &str) -> usize {
    let players = lines.split('\n').collect_vec();
    let mut player1 = Player::from_str(players[0]);
    let mut player2 = Player::from_str(players[1]);
    let mut results: FxHashMap<GameState, WinTracker> = FxHashMap::default();
    let mut start = GameState {
        p1_pos: player1.pos as u8,
        p2_pos: player2.pos as u8,
        p1_score: 0,
        p2_score: 0,
        p1_turn: true,
    };
    // dbg!(count_wins(start));
    count_wins(start).0.max(count_wins(start).1)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GameState {
    p1_pos: u8,
    p2_pos: u8,
    p1_score: usize,
    p2_score: usize,
    p1_turn: bool,
}

impl GameState {
    fn future_by_moving(&self, amount_moved: u8) -> Self {
        let mut new_p1_pos = (self.p1_pos + amount_moved * self.p1_turn as u8) % 10;
        if new_p1_pos == 0 {
            new_p1_pos = 10;
        }
        let mut new_p2_pos = (self.p2_pos + amount_moved * (!self.p1_turn) as u8) % 10;
        if new_p2_pos == 0 {
            new_p2_pos = 10;
        }
        Self {
            p1_pos: new_p1_pos,
            p2_pos: new_p2_pos,
            p1_score: self.p1_score + new_p1_pos as usize * self.p1_turn as usize,
            p2_score: self.p2_score + new_p2_pos as usize * (!self.p1_turn) as usize,
            p1_turn: !self.p1_turn,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct WinTracker(usize, usize);

fn count_wins(state: GameState) -> WinTracker {
    let winning_score = 21;
    dbg!(state);
    if state.p1_score >= winning_score {
        WinTracker(1, 0)
    } else if state.p2_score >= winning_score {
        WinTracker(0, 1)
    } else {
        count_wins(state.future_by_moving(1))
            + count_wins(state.future_by_moving(2))
            + count_wins(state.future_by_moving(3))
    }
}

impl Add for WinTracker {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Debug for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player 1 has {} points and is at {}, Player 2 has {} points and is at {}, it is player {}'s turn",
            self.p1_score, self.p1_pos, self.p2_score, self.p2_pos, self.p1_turn as u8 + 1
        )?;
        writeln!(f)?;
        Ok(())
    }
}
