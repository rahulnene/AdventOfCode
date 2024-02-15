use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2020/day_22.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let (mut p1, mut p2) = LINES
        .split("\r\n\r\n")
        .map(Player::from_str)
        .collect_tuple()
        .unwrap();
    let ans = fight_p1(&mut p1, &mut p2);
    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let (mut p1, mut p2) = LINES
        .split("\r\n\r\n")
        .map(Player::from_str)
        .collect_tuple()
        .unwrap();

    let ans = fight_p2(&mut p1, &mut p2);
    println!("{:?} {:?}", p1.deck, p2.deck);
    (p1.score() + p2.score(), now.elapsed())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Player {
    deck: Vec<usize>,
}

impl Player {
    fn new(deck: Vec<usize>) -> Self {
        Self { deck }
    }

    fn from_str(input: &str) -> Self {
        let deck = input
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect();
        Self::new(deck)
    }

    fn score(&self) -> usize {
        self.deck
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &card)| (i + 1) * card)
            .sum()
    }

    fn is_empty(&self) -> bool {
        self.deck.is_empty()
    }
}

fn fight_p1(p1: &mut Player, p2: &mut Player) -> usize {
    let c1 = p1.deck.remove(0);
    let c2 = p2.deck.remove(0);

    if c1 > c2 {
        p1.deck.push(c1);
        p1.deck.push(c2);
    } else {
        p2.deck.push(c2);
        p2.deck.push(c1);
    }
    loop {
        if p1.is_empty() || p2.is_empty() {
            break;
        }
        fight_p1(p1, p2);
    }
    let ans = p1.score() + p2.score();
    ans
}

fn fight_p2(p1: &mut Player, p2: &mut Player) -> (usize, bool) {
    let mut seen_games = FxHashSet::default();
    let ans = fight_p2_inner(p1, p2, &mut seen_games);
    (p1.score() + p2.score(), ans)
}

fn fight_p2_inner(p1: &mut Player, p2: &mut Player, seen_games: &mut FxHashSet<String>) -> bool {
    if seen_games.contains(&game_config(p1, p2)) {
        return true;
    }
    seen_games.insert(game_config(p1, p2));
    let c1 = p1.deck.remove(0);
    let c2 = p2.deck.remove(0);
    let mut result_of_sub_game = (0, true);
    if p1.deck.len() >= c1 && p2.deck.len() >= c2 {
        println!("PLAYING SUB GAME");
        let mut p1_inner = p1.clone();
        p1_inner.deck = p1_inner.deck.iter().cloned().take(c1).collect_vec();
        let mut p2_inner = p2.clone();
        p2_inner.deck = p2_inner.deck.iter().cloned().take(c2).collect_vec();
        result_of_sub_game = fight_p2(&mut p1_inner, &mut p2_inner);
    } else {
        if c1 > c2 {
            p1.deck.push(c1);
            p1.deck.push(c2);
        } else {
            p2.deck.push(c2);
            p2.deck.push(c1);
        }
    }
    if result_of_sub_game.0 != 0 {
        if result_of_sub_game.1 {
            p1.deck.push(c1);
            p1.deck.push(c2);
        } else {
            p2.deck.push(c2);
            p2.deck.push(c1);
        }
    }
    if p1.is_empty() || p2.is_empty() {
        return p2.is_empty();
    }
    fight_p2_inner(p1, p2, seen_games)
}

fn game_config(p1: &Player, p2: &Player) -> String {
    let p1_cards = p1
        .deck
        .iter()
        .map(|c| c.to_string() + ",")
        .collect::<String>();
    let p2_cards = p2
        .deck
        .iter()
        .map(|c| c.to_string() + ",")
        .collect::<String>();
    p1_cards + "|" + &p2_cards
}
