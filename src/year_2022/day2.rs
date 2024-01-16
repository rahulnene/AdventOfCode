use crate::util::read_lines;

#[derive(PartialEq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    opponent: Play,
    player: Play,
}

impl Round {
    fn new_strat1(opponent: char, player: char) -> Round {
        Round {
            opponent: match opponent {
                'A' => Play::Rock,
                'B' => Play::Paper,
                'C' => Play::Scissors,
                _ => panic!("Invalid play"),
            },
            player: match player {
                'X' => Play::Rock,
                'Y' => Play::Paper,
                'Z' => Play::Scissors,
                _ => panic!("Invalid play"),
            },
        }
    }

    fn new_strat2(opponent_play: char, player_play: char) -> Round {
        let (opponent, player) = match (opponent_play, player_play) {
            ('A', 'X') => (Play::Rock, Play::Scissors),
            ('A', 'Y') => (Play::Rock, Play::Rock),
            ('A', 'Z') => (Play::Rock, Play::Paper),
            ('B', 'X') => (Play::Paper, Play::Rock),
            ('B', 'Y') => (Play::Paper, Play::Paper),
            ('B', 'Z') => (Play::Paper, Play::Scissors),
            ('C', 'X') => (Play::Scissors, Play::Paper),
            ('C', 'Y') => (Play::Scissors, Play::Scissors),
            ('C', 'Z') => (Play::Scissors, Play::Rock),
            _ => panic!("Invalid play"),
        };
        Round { opponent, player }
    }

    fn score(&self) -> u32 {
        match (self.opponent, self.player) {
            (Play::Rock, Play::Rock) => 1 + 3,
            (Play::Rock, Play::Paper) => 2 + 6,
            (Play::Rock, Play::Scissors) => 3 + 0,
            (Play::Paper, Play::Rock) => 1 + 0,
            (Play::Paper, Play::Paper) => 2 + 3,
            (Play::Paper, Play::Scissors) => 3 + 6,
            (Play::Scissors, Play::Rock) => 1 + 6,
            (Play::Scissors, Play::Paper) => 2 + 0,
            (Play::Scissors, Play::Scissors) => 3 + 3,
        }
    }
}

pub fn solution(part: u8) -> u32 {
    if let Ok(lines) = read_lines("./problem_inputs/day2.txt") {
        let (mut score1, mut score2) = (0, 0);
        for line in lines.flatten() {
            let (opponent_play, player_play) =
                (line.chars().next().unwrap(), line.chars().nth(2).unwrap());
            match part {
                1 => score1 += Round::new_strat1(opponent_play, player_play).score(),
                2 => score2 += Round::new_strat2(opponent_play, player_play).score(),
                _ => (),
            };
        }
        return score1 + score2;
    }
    0
}
