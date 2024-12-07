use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use rustc_hash::{FxHashMap, FxHashSet};

pub fn solution(test: bool) -> ((usize, Duration), (usize, Duration)) {
    let lines;
    if test {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_5_test.txt");
    } else {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_5.txt");
    }
    let (rules_str, update_str) = lines.split_once("\n\n").unwrap();
    (
        solve01(rules_str, update_str),
        solve02(rules_str, update_str),
    )
}

fn solve01(rules_str: &str, update_str: &str) -> (usize, Duration) {
    let now = Instant::now();
    let raw_rules: Vec<RawRule> = rules_str.lines().map(RawRule::from_str).collect();
    let rules = raw_rules.iter().fold(RuleSet::default(), |mut acc, x| {
        acc.add_raw_rule(*x);
        acc
    });
    let updates: Vec<UpdateSet> = update_str.lines().map(UpdateSet::from_str).collect();
    let ans = updates
        .iter()
        .map(|update| update.check(&rules).unwrap_or(0))
        .sum::<usize>();
    (ans, now.elapsed())
}

fn solve02(rules_str: &str, update_str: &str) -> (usize, Duration) {
    let now = Instant::now();
    let raw_rules: Vec<RawRule> = rules_str.lines().map(RawRule::from_str).collect();
    let rules = raw_rules.iter().fold(RuleSet::default(), |mut acc, x| {
        acc.add_raw_rule(*x);
        acc
    });
    let updates: Vec<UpdateSet> = update_str.lines().map(UpdateSet::from_str).collect();
    let ans = updates
        .iter()
        .filter(|update| update.check(&rules).is_none())
        .map(|u| u.correct_invalid_pages(&rules))
        .sum();

    (ans, now.elapsed())
}

#[derive(Debug, Copy, Clone)]
struct RawRule {
    before: usize,
    after: usize,
}

impl RawRule {
    fn from_str(s: &str) -> Self {
        let (before, after) = s.split_once("|").unwrap();
        let before = before.parse().unwrap();
        let after = after.parse().unwrap();
        Self { before, after }
    }
}

#[derive(Debug, Default, Clone)]
struct RuleSet {
    rules: FxHashMap<usize, Vec<usize>>,
}

impl RuleSet {
    fn add_raw_rule(&mut self, raw_rule: RawRule) {
        let key = raw_rule.before;
        let value = raw_rule.after;
        self.rules.entry(key).or_insert_with(Vec::new).push(value);
    }

    fn get_rule(&self, key: usize) -> Option<&Vec<usize>> {
        self.rules.get(&key)
    }
}

#[derive(Debug, Clone)]
struct UpdateSet {
    pages: VecDeque<usize>,
}

impl UpdateSet {
    fn from_str(s: &str) -> Self {
        let pages = s.split(',').map(|x| x.parse().unwrap()).collect();
        Self { pages }
    }

    fn check(&self, rules_list: &RuleSet) -> Option<usize> {
        let mid_page = *self.pages.get(self.pages.len() / 2).unwrap();
        let mut seen = FxHashSet::default();

        for page in self.pages.iter() {
            if let Some(allowed) = rules_list.get_rule(*page) {
                for &allowed_num in allowed {
                    if seen.contains(&allowed_num) {
                        return None;
                    }
                }
            }
            seen.insert(*page);
        }

        Some(mid_page)
    }

    fn correct_invalid_pages(&self, rules_list: &RuleSet) -> usize {
        let sequence: Vec<usize> = self.pages.iter().copied().collect();
        // dbg!(&sequence);
        let sorted = sort_with_ruleset(&rules_list.rules, &sequence).unwrap();
        // dbg!(&sorted);
        let mid_page = sorted.get(self.pages.len() / 2).unwrap();
        // dbg!(mid_page);
        *mid_page
    }
}

fn sort_with_ruleset(
    ruleset: &FxHashMap<usize, Vec<usize>>,
    numbers: &[usize],
) -> Option<Vec<usize>> {
    // Build graph and in-degree count
    let mut graph: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
    let mut in_degree: FxHashMap<usize, usize> = FxHashMap::default();

    for &num in numbers {
        graph.entry(num).or_default();
        in_degree.entry(num).or_insert(0);
    }

    for (&key, values) in ruleset {
        for &value in values {
            if graph.contains_key(&key) && graph.contains_key(&value) {
                graph.entry(key).or_default().push(value);
                *in_degree.entry(value).or_insert(0) += 1;
            }
        }
    }

    // Perform topological sort
    let mut sorted = Vec::new();
    let mut queue: VecDeque<usize> = in_degree
        .iter()
        .filter_map(|(&node, &deg)| if deg == 0 { Some(node) } else { None })
        .collect();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Check if topological sort is valid (i.e., no cycles)
    if sorted.len() == numbers.len() {
        Some(sorted)
    } else {
        None // Cycle detected or invalid ruleset
    }
}
