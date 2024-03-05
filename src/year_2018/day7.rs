use std::time::{Duration, Instant};

use itertools::Itertools;
use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2018/day_7_test.txt");

pub fn solution() -> ((String, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (String, Duration) {
    let now = Instant::now();
    let mut steps = FxHashMap::default();
    for line in LINES.lines() {
        let (_, prereq, _, _, _, _, _, name, _, _) =
            line.split_whitespace().collect_tuple().unwrap();
        let prereq = prereq.chars().next().unwrap();
        let name = name.chars().next().unwrap();
        steps
            .entry(name)
            .and_modify(|e: &mut Step| {
                if e.prereqs.is_some() {
                    e.prereqs.as_mut().unwrap().push(prereq)
                } else {
                    e.prereqs = Some(vec![prereq])
                }
            })
            .or_insert(Step {
                prereqs: Some(vec![prereq]),
            });
        if !steps.contains_key(&prereq) {
            steps.insert(prereq, Step { prereqs: None });
        }
    }
    let mut order = String::new();
    while steps.len() > 0 {
        let next = steps
            .iter()
            .filter(|(_, step)| step.prereqs.is_none() || step.prereqs.as_ref().unwrap().is_empty())
            .map(|(k, _)| *k)
            .min()
            .unwrap();
        order.push(next);
        steps.remove(&next);
        for step in steps.values_mut() {
            if let Some(prereqs) = step.prereqs.as_mut() {
                prereqs.retain(|&p| p != next);
            }
        }
    }
    (order, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut steps = FxHashMap::default();
    for line in LINES.lines() {
        let (_, prereq, _, _, _, _, _, name, _, _) =
            line.split_whitespace().collect_tuple().unwrap();
        let prereq = prereq.chars().next().unwrap();
        let name = name.chars().next().unwrap();
        steps
            .entry(name)
            .and_modify(|e: &mut Step| {
                if e.prereqs.is_some() {
                    e.prereqs.as_mut().unwrap().push(prereq)
                } else {
                    e.prereqs = Some(vec![prereq])
                }
            })
            .or_insert(Step {
                prereqs: Some(vec![prereq]),
            });
        if !steps.contains_key(&prereq) {
            steps.insert(prereq, Step { prereqs: None });
        }
    }
    let mut steps_backup = steps.clone();
    let mut order = String::new();
    while steps.len() > 0 {
        let next = steps
            .iter()
            .filter(|(_, step)| step.prereqs.is_none() || step.prereqs.as_ref().unwrap().is_empty())
            .map(|(k, _)| *k)
            .min()
            .unwrap();
        order.push(next);
        steps.remove(&next);
        for step in steps.values_mut() {
            if let Some(prereqs) = step.prereqs.as_mut() {
                prereqs.retain(|&p| p != next);
            }
        }
    }

    let mut workers = vec![
        Worker {
            step: None,
            time_left: 0,
        };
        5
    ];
    (0, now.elapsed())
}

#[derive(Debug, Clone)]
struct Step {
    prereqs: Option<Vec<char>>,
}

#[derive(Debug, Clone, Copy)]
struct Worker {
    step: Option<char>,
    time_left: usize,
}
