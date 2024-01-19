use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2019/day_22.txt");
    (
        solve(lines, 10007, 1),
        solve(lines, 119315717514047, 101741582076661),
    )
}

fn solve(lines: &str, deck_size: usize, shuffle_reps: usize) -> (usize, Duration) {
    let now = Instant::now();
    let mut deck = Deck::new(deck_size);
    for _ in 0..shuffle_reps {
        for line in lines.lines() {
            if line.starts_with("deal into") {
                // println!("deal into");
                deck.deal_new_stack();
            } else if line.starts_with("cut") {
                let n = line.split(' ').last().unwrap().parse::<isize>().unwrap();
                // println!("cut {}", n);
                deck.cut_n(n);
            } else {
                let inc = line.split(' ').last().unwrap().parse::<isize>().unwrap();
                // println!("deal with increment {}", inc);
                deck.deal_with_inc(inc);
            }
            // println!("{:?}", &deck.cards);
        }
    }
    // dbg!(deck.cards);
    (
        deck.cards.iter().position(|&c| c == 2019).unwrap(),
        now.elapsed(),
    )
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Deck {
    cards: VecDeque<usize>,
}

impl Deck {
    fn new(size: usize) -> Self {
        Self {
            cards: (0..size).collect(),
        }
    }

    fn deal_new_stack(&mut self) {
        self.cards.make_contiguous().reverse();
    }

    fn cut_n(&mut self, n: isize) {
        if n.is_positive() {
            self.cards.rotate_left(n as usize);
        } else {
            self.cards.rotate_right(n.abs() as usize);
        }
    }

    fn deal_with_inc(&mut self, inc: isize) {
        let mut new_cards = VecDeque::from_iter(0..self.cards.len());
        for (card_num, card) in self.cards.iter().enumerate() {
            new_cards[(card_num as isize * inc).rem_euclid(self.cards.len() as isize) as usize] =
                *card;
        }
        self.cards = new_cards;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deal_new_stack() {
        let mut deck = Deck::new(10);
        deck.deal_new_stack();
        assert_eq!(deck.cards, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_cut_n() {
        let mut deck = Deck::new(10);
        deck.cut_n(3);
        assert_eq!(deck.cards, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test2_cut_n() {
        let mut deck = Deck::new(10);
        deck.cut_n(-4);
        assert_eq!(deck.cards, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_cut_with_inc() {
        let mut deck = Deck::new(10);
        deck.deal_with_inc(3);
        assert_eq!(deck.cards, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}
