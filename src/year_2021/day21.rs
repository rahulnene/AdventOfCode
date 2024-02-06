use itertools::Itertools;
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

use std::fmt::Debug;

use std::time::{Duration, Instant};
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = FxHashMap::default();
         $( map.insert($key, $val); )*
         map
    }}
}
const LINES: &str = include_str!("../../problem_inputs_2021/day_21.txt");
lazy_static! {
    static ref FREQS: FxHashMap<u16, u16> =
        hashmap![3 => 1, 4 => 3, 5 => 6, 6 => 7, 7 => 6, 8 => 3, 9 => 1];
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let players = LINES.split('\n').collect_vec();
    let player1 = Player::from_str(players[0]);
    let player2 = Player::from_str(players[1]);
    let mut player_1_turn = true;
    let mut round = 0;
    let mut game = Game {
        p1: player1,
        p2: player2,
    };
    let mut die = DetDice::new(100);
    loop {
        game = game.make_move(die.roll(), player_1_turn);
        round += 1;
        if game.high_score() >= 1000 {
            let ans = round * 3 * game.low_score();
            return (ans, now.elapsed());
        }
        player_1_turn = !player_1_turn;
    }
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let players = LINES.split('\n').collect_vec();
    let player1 = Player::from_str(players[0]);
    let player2 = Player::from_str(players[1]);
    let mut games = GameWinMap::new();
    games.freqs.insert(
        Game {
            p1: player1,
            p2: player2,
        },
        1,
    );
    let mut win_arr = vec![1, 1];
    let mut player_1_turn = true;
    while !games.freqs.is_empty() {
        let temp = games.make_move(player_1_turn);
        games = temp.0;
        let wins = temp.1;
        if player_1_turn {
            win_arr[0] += wins;
        }
        if !player_1_turn {
            win_arr[1] += wins;
        }
        player_1_turn = !player_1_turn;
    }
    (win_arr.into_iter().max().unwrap(), now.elapsed())
}

#[derive(Debug, Clone)]
struct GameWinMap {
    freqs: FxHashMap<Game, usize>,
}

impl GameWinMap {
    fn new() -> Self {
        Self {
            freqs: FxHashMap::default(),
        }
    }
    fn make_move(&self, player_1_turn: bool) -> (GameWinMap, usize) {
        let mut next_games = GameWinMap::new();
        let mut wins = 0;
        for (game, count) in self.freqs.iter() {
            for (roll, frequency) in FREQS.iter() {
                let new_game = game.make_move(*roll, player_1_turn);
                let new_count = count * *frequency as usize;
                if new_game.high_score() >= 21 {
                    wins += new_count;
                } else {
                    if !next_games.freqs.contains_key(&new_game) {
                        next_games.freqs.insert(new_game, 0);
                    }
                    *next_games.freqs.get_mut(&new_game).unwrap() += new_count;
                }
            }
        }
        return (next_games, wins);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    id: u16,
    pos: u16,
    score: usize,
}

impl Player {
    fn from_str(line: &str) -> Self {
        {
            let mut parts = line.split_whitespace();
            let id = parts.nth(1).unwrap().parse().unwrap();
            let pos = parts.nth(2).unwrap().parse().unwrap();
            Self { id, pos, score: 0 }
        }
    }

    fn update_score(&mut self, roll: u16) {
        self.pos = wrap_around_board(self.pos, roll);
        self.score = self.score + self.pos as usize;
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct DetDice {
    current_val: u16,
    max_val: u16,
}
impl DetDice {
    fn new(max_roll: u16) -> Self {
        Self {
            current_val: 0,
            max_val: max_roll,
        }
    }

    fn roll(&mut self) -> u16 {
        let mut out = 0;
        for _ in 0..3 {
            self.current_val += 1;
            self.current_val %= self.max_val;
            out += self.current_val;
        }
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Game {
    p1: Player,
    p2: Player,
}

impl Game {
    fn high_score(&self) -> usize {
        self.p1.score.max(self.p2.score)
    }
    fn low_score(&self) -> usize {
        self.p1.score.min(self.p2.score)
    }
    fn make_move(&self, roll: u16, player_1_turn: bool) -> Game {
        let mut game = self.clone();
        if player_1_turn {
            game.p1.update_score(roll);
        } else {
            game.p2.update_score(roll);
        }
        game
    }
}

fn wrap_around_board(pos: u16, roll: u16) -> u16 {
    let mut pos = pos + roll;
    pos %= 10;
    if pos == 0 {
        pos = 10;
    }
    pos
}
