use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2023/day_4_test.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = lines.lines().map(Card::parse).map(Card::score).sum();
    (ans, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let cards = lines.lines().map(Card::parse).collect_vec();
    let mut all_cards: Vec<usize> = cards.iter().map(|c| c.id).collect_vec();
    let mut cards_in_hand = Vec::new();
    for card in cards.iter().filter(|c| c.winning.len() == 0) {
        all_cards.push(card.id);
        cards_in_hand.push(card);
    }
    while cards_in_hand.len() > 0 {
        let mut new_cards = Vec::new();
        for card in cards_in_hand {
            let winning_amount = card.count_winning();
            for id_counter in card.id + 1..=card.id + winning_amount {
                all_cards.push(id_counter);
                new_cards.push(cards.iter().filter(|c| c.id == id_counter).next().unwrap());
            }
        }
        cards_in_hand = new_cards;
        dbg!(&all_cards.iter().counts());
        println!("-------------------");
    }
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    having: Vec<usize>,
}

impl Card {
    fn new(id: usize, winning: Vec<usize>, having: Vec<usize>) -> Self {
        Self {
            id,
            winning,
            having,
        }
    }

    fn parse(s: &str) -> Self {
        let (card_id_str, card_nums_str) = s.split(':').collect_tuple().unwrap();
        let id: usize = card_id_str
            .trim()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let (winner_str, having_str) = card_nums_str.split('|').collect_tuple().unwrap();
        // dbg!(&winner_str, &having_str);
        let winning = winner_str
            .trim()
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect_vec();
        let having = having_str
            .trim()
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect_vec();
        Self::new(id, winning, having)
    }

    fn count_winning(&self) -> usize {
        self.having
            .iter()
            .filter(|c| self.winning.contains(c))
            .count()
    }
    fn score(self) -> usize {
        let ans = self.count_winning();
        if ans == 0 {
            0
        } else {
            2_usize.pow((ans - 1) as u32)
        }
    }
}
