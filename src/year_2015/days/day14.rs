use std::time::{Duration, Instant};

use fxhash::FxHashMap;
use itertools::Itertools;

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2015/day_14.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (
        lines
            .lines()
            .map(|l| Reindeer::parse(l))
            .map(|r| r.calculate_distance_at(2503))
            .max()
            .unwrap(),
        now.elapsed(),
    )
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut reindeer_score = FxHashMap::default();
    for l in lines.lines() {
        let reindeer = Reindeer::parse(l);
        reindeer_score.insert(reindeer.name, 0);
    }
    let reindeer = lines
        .lines()
        .map(|l| Reindeer::parse(l))
        .collect::<Vec<_>>();
    for t in 0..1000 {
        let leader_reindeers = reindeer
            .iter()
            .map(|r| (r, r.calculate_distance_at(t)))
            .max_set_by_key(|s| s.1)
            .iter()
            .map(|f| f.0)
            .collect_vec();
        for r in leader_reindeers {
            *reindeer_score.get_mut(&r.name).unwrap() += 1;
        }
    }
    dbg!(&reindeer_score);
    let ans = reindeer_score.values().max().unwrap();
    (*ans, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Reindeer {
    name: String,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

impl Reindeer {
    fn parse(s: &str) -> Self {
        let mut words = s.split_whitespace();
        let name = words.next().unwrap().to_owned();
        let speed = words.nth(2).unwrap().parse::<usize>().unwrap();
        let fly_time = words.nth(2).unwrap().parse::<usize>().unwrap();
        let rest_time = words.nth(6).unwrap().parse::<usize>().unwrap();
        Self {
            name,
            speed,
            fly_time,
            rest_time,
        }
    }

    fn calculate_distance_at(&self, time: usize) -> usize {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = time / cycle_time;
        let remainder = time % cycle_time;
        let mut distance = cycles * self.speed * self.fly_time;
        if remainder > self.fly_time {
            distance += self.speed * self.fly_time;
        } else {
            distance += self.speed * remainder;
        }
        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reindeer_parse() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        let expected = Reindeer {
            name: "Comet".to_owned(),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };
        assert_eq!(Reindeer::parse(input), expected);
    }

    #[test]
    fn test_dancer_calculate_distance_at() {
        let input = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let reindeer = Reindeer::parse(input);
        assert_eq!(reindeer.calculate_distance_at(1), 16);
        assert_eq!(reindeer.calculate_distance_at(10), 160);
        assert_eq!(reindeer.calculate_distance_at(11), 176);
        assert_eq!(reindeer.calculate_distance_at(1000), 1056);
    }

    #[test]
    fn test_comet_calculate_distance_at() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        let reindeer = Reindeer::parse(input);
        assert_eq!(reindeer.calculate_distance_at(1), 14);
        assert_eq!(reindeer.calculate_distance_at(10), 140);
        assert_eq!(reindeer.calculate_distance_at(11), 140);
        assert_eq!(reindeer.calculate_distance_at(1000), 1120);
    }
}