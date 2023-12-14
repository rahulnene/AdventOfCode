use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_22.txt");
    match part {
        1 => solve01(lines),
        // 2 => solve(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let (mut p1, mut p2) = lines
        .split("\n\n")
        .map(Player::from_str)
        .collect_tuple()
        .unwrap();
    while !p1.is_empty() && !p2.is_empty() {
        fight(&mut p1, &mut p2);
    }
    p1.score() + p2.score()
}

#[derive(Debug, Clone)]
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

fn fight(p1: &mut Player, p2: &mut Player) {
    let c1 = p1.deck.remove(0);
    let c2 = p2.deck.remove(0);

    if c1 > c2 {
        p1.deck.push(c1);
        p1.deck.push(c2);
    } else {
        p2.deck.push(c2);
        p2.deck.push(c1);
    }
}
