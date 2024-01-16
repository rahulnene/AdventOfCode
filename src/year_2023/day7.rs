use std::time::{Duration, Instant};

use fxhash::FxHashMap;
use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2023/day_7.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut hand_to_bid = FxHashMap::default();
    let mut hands = Vec::new();
    for line in lines.lines() {
        let (hand_str, bid_str) = line.split_once(" ").unwrap();
        let hand = Hand::from_str(hand_str);
        let bid = bid_str.parse::<usize>().unwrap();
        hand_to_bid.insert(hand, bid);
        hands.push(hand);
    }
    hands.sort_unstable();

    let ans = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand_to_bid[hand])
        .sum::<usize>();
    (ans, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Self::Ace),
            'K' => Some(Self::King),
            'Q' => Some(Self::Queen),
            'J' => Some(Self::Jack),
            'T' => Some(Self::Ten),
            '9' => Some(Self::Nine),
            '8' => Some(Self::Eight),
            '7' => Some(Self::Seven),
            '6' => Some(Self::Six),
            '5' => Some(Self::Five),
            '4' => Some(Self::Four),
            '3' => Some(Self::Three),
            '2' => Some(Self::Two),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Hand {
    fn from_str(s: &str) -> Self {
        let mut cards = Vec::new();
        for c in s.chars() {
            cards.push(Card::from_char(c).unwrap());
        }
        Self {
            cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
            hand_type: Self::type_of(&cards),
        }
    }

    fn type_of(cards: &[Card]) -> HandType {
        let counts = cards.iter().counts();
        match counts.values().len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if counts.values().any(|&x| x == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if counts.values().any(|&x| x == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_type() {
        let hand = Hand::from_str("AA4AA");
        assert_eq!(hand.hand_type, HandType::FourOfAKind);
    }

    #[test]
    fn test_hand_type_order() {
        let hand1 = Hand::from_str("AA4AA");
        let hand2 = Hand::from_str("AA4KK");
        assert!(hand1.hand_type > hand2.hand_type);
    }
}
