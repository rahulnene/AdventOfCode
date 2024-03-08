use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::str::FromStr;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2023/day_19.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let (workflow_strings, objects) = LINES.split("\n\n").collect_tuple().unwrap();

    let workflows: FxHashMap<&str, Vec<Rule>> =
        workflow_strings.lines().map(rules_parser).collect();
    (solve01(&workflows, objects), solve02(&workflows))
}

fn solve01(workflows: &FxHashMap<&'static str, Vec<Rule>>, objects: &str) -> (usize, Duration) {
    let now = Instant::now();

    let ans = objects
        .lines()
        .filter_map(|object_string| {
            process_object(&Object::from_str(object_string).unwrap(), &workflows)
        })
        .sum::<isize>() as usize;
    (ans, now.elapsed())
}
fn solve02(workflows: &FxHashMap<&'static str, Vec<Rule>>) -> (usize, Duration) {
    let now = Instant::now();
    let max_val = 40;
    let mut sum = 0;
    (sum, now.elapsed())
}
type Rule = Box<dyn Fn(&Object) -> RuleResult>;

fn process_object(obj: &Object, workflows: &FxHashMap<&'static str, Vec<Rule>>) -> Option<isize> {
    let mut rule: &Vec<Rule> = workflows.get("in").unwrap();
    let mut rule_ind = 0;
    loop {
        let res = rule.get(rule_ind).unwrap()(obj);
        match res {
            RuleResult::Accepted => {
                return Some(obj.sum());
            }
            RuleResult::Rejected => return None,
            RuleResult::NextRule => rule_ind += 1,
            RuleResult::NextWorkflow(a) => {
                rule = workflows.get(&a).unwrap();
                rule_ind = 0;
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RuleResult {
    NextRule,
    NextWorkflow(&'static str),
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug)]
struct Object {
    x: i16,
    m: i16,
    a: i16,
    s: i16,
}

impl Object {
    fn sum(self) -> isize {
        (self.x + self.m + self.a + self.s) as isize
    }
    fn get_property_from_str(&self, property: &str) -> i16 {
        match property {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("wrong property format!"),
        }
    }
}

impl FromStr for Object {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .trim_matches(|p| p == '{' || p == '}')
            .split(',')
            .collect();
        let x: i16 = parts[0].split('=').nth(1).unwrap().parse()?;
        let m: i16 = parts[1].split('=').nth(1).unwrap().parse()?;
        let a: i16 = parts[2].split('=').nth(1).unwrap().parse()?;
        let s: i16 = parts[3].split('=').nth(1).unwrap().parse()?;

        Ok(Object { x, m, a, s })
    }
}

//Converts a workflow into a name and vec of rules
fn rules_parser(workflow: &'static str) -> (&str, Vec<Rule>) {
    let mut individual_rule_strings: Vec<&'static str> = workflow
        .trim_matches(|p| p == '{' || p == '}')
        .split(',')
        .collect();
    let (workflow_name, first_rule) = individual_rule_strings[0]
        .split('{')
        .collect_tuple()
        .unwrap();
    individual_rule_strings = individual_rule_strings.split_first().unwrap().1.to_vec();
    individual_rule_strings.insert(0, first_rule);

    let rules_vec: Vec<Rule> = individual_rule_strings
        .iter()
        .map(|part| str_to_rule(part))
        .collect();

    (workflow_name, rules_vec)
}

fn str_to_rule(rule_str: &'static str) -> Rule {
    Box::new(|o: &Object| -> RuleResult {
        let parts: Vec<&str> = rule_str
            .split(|c| c == '<' || c == '>' || c == '=')
            .collect();
        if parts.len() == 1 {
            match *parts.first().expect("parts vector is empty") {
                "A" => return RuleResult::Accepted,
                "R" => return RuleResult::Rejected,
                _ => {
                    return RuleResult::NextWorkflow(parts.first().expect("parts vector is empty"))
                }
            }
        }
        let object_prop = o.get_property_from_str(parts[0]);

        let (value_string, return_string): (&str, &str) =
            parts[1].split(':').collect_tuple().unwrap();
        let value = value_string.parse::<i16>().unwrap();
        let comparator = rule_str
            .chars()
            .find(|&c| c == '<' || c == '>' || c == '=')
            .unwrap();
        let is_match = match comparator {
            '<' => object_prop < value,
            '>' => object_prop > value,
            '=' => object_prop == value,
            _ => unreachable!(),
        };

        if is_match {
            match return_string {
                "A" => RuleResult::Accepted,
                "R" => RuleResult::Rejected,
                _ => RuleResult::NextWorkflow(return_string),
            }
        } else {
            RuleResult::NextRule
        }
    }) as Box<dyn Fn(&Object) -> RuleResult>
}
