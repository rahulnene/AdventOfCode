use std::{
    time::{Duration, Instant},
    usize,
};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2023/day_6.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let (record_times, race_lengths) = lines.lines().collect_tuple().unwrap();
    let record_times = record_times
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<usize>().unwrap());
    let race_lengths = race_lengths
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<usize>().unwrap());
    let records = record_times
        .zip(race_lengths)
        .map(ways_to_win)
        .fold(1, |prod, num| prod * num);
    (records, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    // return (0, Duration::from_secs(0));
    let now = Instant::now();
    let (race_length, record_distance) = lines
        .lines()
        .map(|s| {
            s.chars()
                .filter(|t| t.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    dbg!(race_length, record_distance);
    (ways_to_win((race_length, record_distance)), now.elapsed())
}

fn ways_to_win((race_length, record_distance): (usize, usize)) -> usize {
    // (1..race_length)
    //     .map(|t| calculate_distance(race_length, t) > record_distance)
    //     .filter(|t| *t)
    //     .count()
    let mut d = ((race_length * race_length - (4 * record_distance)) as f64).sqrt();
    if d.fract() == 0.0 {
        d -= 1.0;
        if (race_length - d as usize) % 2 == 0 {
            d -= 1.0;
        }
    }
    return d.round() as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ways_to_win_1() {
        let ans = ways_to_win((7, 9));
        assert_eq!(ans, 4);
    }
    #[test]
    fn test_ways_to_win_2() {
        let ans = ways_to_win((15, 40));
        assert_eq!(ans, 8);
    }
    #[test]
    fn test_ways_to_win_3() {
        let ans = ways_to_win((30, 200));
        assert_eq!(ans, 9);
    }
}
