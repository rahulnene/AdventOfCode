use itertools::Itertools;
use lazy_static::lazy_static;
use std::time::{Duration, Instant};
const LINES: &str = include_str!("../../problem_inputs_2015/day_13.txt");

lazy_static! {
    static ref HAPPINESS_RULES: Vec<HappinessRule> =
        LINES.lines().map(parse_happiness_rule).collect();
    static ref NAMES: Vec<&'static str> = HAPPINESS_RULES
        .iter()
        .map(|(name, _, _)| name.as_str())
        .unique()
        .collect();
}

pub fn solution() -> ((isize, Duration), (isize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (isize, Duration) {
    let now = Instant::now();
    let ans = find_optimal(NAMES.as_slice(), HAPPINESS_RULES.as_slice());
    (ans, now.elapsed())
}

fn solve02() -> (isize, Duration) {
    let now = Instant::now();
    let mut happiness_rules = HAPPINESS_RULES.clone();
    let me = "me";
    for name in NAMES.iter() {
        happiness_rules.push((me.to_owned(), name.to_owned().to_owned(), 0));
        happiness_rules.push((name.to_owned().to_owned(), me.to_owned(), 0));
    }
    let mut names = NAMES.clone();
    names.push(me);
    let ans = find_optimal(names.as_slice(), happiness_rules.as_slice());
    (ans, now.elapsed())
}

type HappinessRule = (String, String, isize);

fn parse_happiness_rule(s: &str) -> HappinessRule {
    let mut iter = s.split_whitespace();
    let name = iter.next().unwrap();
    let _ = iter.next().unwrap();
    let happiness_direction = iter.next().unwrap();
    let happiness_amount = iter.next().unwrap().parse::<isize>().unwrap();
    let happiness_change = match happiness_direction {
        "gain" => happiness_amount,
        "lose" => -happiness_amount,
        _ => panic!("Invalid happiness direction: {happiness_direction}"),
    };
    let other = iter.last().unwrap().strip_suffix('.').unwrap();
    (name.to_owned(), other.to_owned(), happiness_change)
}

fn calculate_happiness(happiness_rules: &[HappinessRule], seating_arrangement: &[&&str]) -> isize {
    let mut happiness = 0;
    for i in 0..seating_arrangement.len() {
        let left = seating_arrangement[i];
        let right = seating_arrangement[(i + 1) % seating_arrangement.len()];
        // dbg!(left, right);
        let rule1 = happiness_rules
            .iter()
            .find(|(name, other, _)| name == left && other == right)
            .unwrap();
        happiness += rule1.2;
        let rule2 = happiness_rules
            .iter()
            .find(|(name, other, _)| name == right && other == left)
            .unwrap();
        happiness += rule2.2;
    }
    happiness
}

fn find_optimal(names: &[&str], happiness_rules: &[HappinessRule]) -> isize {
    names
        .iter()
        .permutations(names.len())
        .map(|s| calculate_happiness(happiness_rules, s.as_slice()))
        .max()
        .unwrap()
}
