use std::time::{Duration, Instant};

use rustc_hash::FxHashSet;

const LINES: &str = include_str!("../../problem_inputs_2020/day_16.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let mut lines = LINES.split("\r\n\r\n");
    let rules = lines.next().unwrap();
    let my_ticket = Ticket::from_str(lines.next().unwrap().lines().skip(1).next().unwrap());
    let nearby_tickets = lines
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(Ticket::from_str)
        .collect::<Vec<_>>();
    let mut rules_vec = Vec::new();
    for rule in rules.lines() {
        let mut parts = rule.split(": ");
        let name = parts.next().unwrap();
        let ranges = parts
            .next()
            .unwrap()
            .split(" or ")
            .map(|range| {
                let mut range_parts = range.split('-');
                let min = range_parts.next().unwrap().parse().unwrap();
                let max = range_parts.next().unwrap().parse().unwrap();
                (min, max)
            })
            .collect();
        rules_vec.push(Rule::new(name, ranges));
    }
    (
        solve01(&nearby_tickets, &rules_vec),
        solve02(&nearby_tickets, &rules_vec, &my_ticket),
    )
}

fn solve01(nearby_tickets: &[Ticket], rules_vec: &[Rule]) -> (usize, Duration) {
    let now = Instant::now();
    let ans = nearby_tickets
        .iter()
        .map(|ticket| ticket.invalid_value_sum(rules_vec))
        .sum();
    (ans, now.elapsed())
}

fn solve02(nearby_tickets: &[Ticket], rules_vec: &[Rule], my_ticket: &Ticket) -> (usize, Duration) {
    let now = Instant::now();
    let valid_tickets = nearby_tickets
        .iter()
        .filter(|ticket| ticket.invalid_value_sum(&rules_vec) == 0)
        .collect::<Vec<_>>();
    let mut confirmed_rules: Vec<Option<String>> = vec![None; rules_vec.len()];
    let field_count = rules_vec.len();

    while confirmed_rules.iter().any(Option::is_none) {
        if confirmed_rules.iter().filter(|r| r.is_none()).count() == 1 {
            break;
        }
        for rule in rules_vec {
            let possible_field_ind_to_rule_name: Vec<_> = (0..field_count)
                .filter(|&field_ind| {
                    confirmed_rules[field_ind].is_none()
                        && valid_tickets
                            .iter()
                            .all(|ticket| rule.passes(ticket.values[field_ind]))
                })
                .map(|field_ind| (field_ind, rule.name.clone()))
                .collect();
            if possible_field_ind_to_rule_name.len() == 1 {
                let (field_ind, rule_name) = &possible_field_ind_to_rule_name[0];
                confirmed_rules[*field_ind] = Some(rule_name.clone());
            }
        }
    }

    if let Some(unseen_name_ind) = confirmed_rules.iter().position(Option::is_none) {
        let seen_names = FxHashSet::from_iter(confirmed_rules.iter().filter_map(Option::as_ref));
        let unseen_name = rules_vec
            .iter()
            .find(|rule| !seen_names.contains(&rule.name))
            .unwrap();
        confirmed_rules[unseen_name_ind] = Some(unseen_name.name.clone());
    }

    let ans = confirmed_rules
        .iter()
        .map(|a| a.as_ref().unwrap())
        .enumerate()
        .filter_map(|(ind, name)| {
            if name.starts_with("departure") {
                Some(my_ticket.values[ind])
            } else {
                None
            }
        })
        .product();

    (ans, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    name: String,
    ranges: Vec<(usize, usize)>,
}

impl Rule {
    fn new(name: &str, ranges: Vec<(usize, usize)>) -> Self {
        Self {
            name: name.to_string(),
            ranges,
        }
    }

    fn passes(&self, value: usize) -> bool {
        self.ranges
            .iter()
            .any(|(min, max)| value >= *min && value <= *max)
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<usize>,
}

impl Ticket {
    fn invalid_value_sum(&self, rules: &[Rule]) -> usize {
        let ans = self
            .values
            .iter()
            .filter(|value| !rules.iter().any(|rule| rule.passes(**value)))
            .sum();
        ans
    }

    fn from_str(s: &str) -> Self {
        Self {
            values: s.split(',').map(|v| v.parse().unwrap()).collect(),
        }
    }
}
