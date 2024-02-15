use itertools::Itertools;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2020/day_19_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let (rules, to_check) = LINES.split_once("\r\n\r\n").unwrap();
    let mut ind_to_rule = FxHashMap::default();
    let mut ind_to_rule_p2 = FxHashMap::default();
    for rule in rules.lines() {
        let id = rule.split(":").next().unwrap().parse().unwrap();
        ind_to_rule_p2.insert(
            id,
            parse_rule(if id == 8 {
                "8: 42 | 42 8"
            } else if id == 11 {
                "11: 42 31 | 42 11 31"
            } else {
                rule
            }),
        );
        let a = parse_rule(rule);
        ind_to_rule.insert(id, a);
    }
    dbg!(&ind_to_rule);
    (
        // solve(&ind_to_rule, to_check),
        (0, Duration::default()),
        solve(&ind_to_rule_p2, to_check),
    )
}

fn solve(ind_to_rule: &FxHashMap<usize, RuleType>, to_check: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = to_check
        .lines()
        .filter(|to_check| {
            let mut to_check_chars = to_check.chars().collect_vec();
            let check_str = to_check_chars.iter().collect::<String>();
            let a = apply_rule(
                ind_to_rule.get(&0).unwrap(),
                &ind_to_rule,
                &mut to_check_chars,
            );
            // dbg!(check_str, a);
            a
        })
        .count();
    (ans, now.elapsed())
}

#[derive(Parser)]
#[grammar = "year_2020/day19.pest"]
struct RuleParser;

#[derive(Debug, Clone)]
enum RuleType {
    Char(char),
    Or(Box<RuleType>, Box<RuleType>),
    And(Box<RuleType>, Box<RuleType>),
    Ref(usize),
}

fn parse_rule(rule_str: &str) -> RuleType {
    fn parse_rule_inner(pair: Pair<Rule>) -> RuleType {
        match pair.as_rule() {
            Rule::numbered_rule => {
                let inner = pair.into_inner().next().unwrap();
                parse_rule_inner(inner)
            }
            Rule::char_rule => {
                let inner = pair.into_inner().next().unwrap();
                let c = inner.as_str().chars().next().unwrap();
                // dbg!(c);
                RuleType::Char(c)
            }
            Rule::or_rule => {
                let mut inner = pair.into_inner();
                let left = parse_rule_inner(inner.next().unwrap());
                let right = parse_rule_inner(inner.next().unwrap());
                // dbg!(&left, &right);
                RuleType::Or(Box::new(left), Box::new(right))
            }
            Rule::and_rule => {
                let mut inner = pair.into_inner();
                let left = parse_rule_inner(inner.next().unwrap());
                let right = parse_rule_inner(inner.next().unwrap());
                // dbg!(&left, &right);
                RuleType::And(Box::new(left), Box::new(right))
            }
            Rule::rule_id => {
                let ans = RuleType::Ref(pair.as_str().parse().unwrap());
                // dbg!(&ans);
                ans
            }
            _ => unreachable!(),
        }
    }
    let pairs = RuleParser::parse(Rule::numbered_rule, rule_str)
        .unwrap()
        .next()
        .unwrap();

    parse_rule_inner(pairs)
}

//Returns whether the rule passed
fn apply_rule<'chars>(
    rule: &RuleType,
    rules: &FxHashMap<usize, RuleType>,
    chars: &'chars mut Vec<char>,
) -> bool {
    if chars.is_empty() {
        return false;
    }
    println!("{} {:?}", chars.iter().collect::<String>(), rule);
    let a = match rule {
        RuleType::Char(c) => {
            if chars.first().unwrap() == c {
                chars.remove(0);
                true
            } else {
                false
            }
        }
        RuleType::And(rule_1, rule_2) => {
            let chars_clone = chars.clone();
            if apply_rule(rule_1, rules, chars) {
                if apply_rule(rule_2, rules, chars) {
                    true
                } else {
                    *chars = chars_clone;
                    false
                }
            } else {
                *chars = chars_clone;
                false
            }
        }

        RuleType::Or(rule_1, rule_2) => {
            let chars_clone = chars.clone();
            if apply_rule(rule_1, rules, chars) {
                true
            } else {
                *chars = chars_clone;
                let chars_clone = chars.clone();
                if apply_rule(rule_2, rules, chars) {
                    true
                } else {
                    *chars = chars_clone;
                    false
                }
            }
        }
        // _ => unimplemented!(),
        RuleType::Ref(rule_num) => apply_rule(rules.get(rule_num).unwrap(), rules, chars),
    };
    a
}
