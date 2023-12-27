use fxhash::FxHashMap;
use itertools::Itertools;
use std::str::FromStr;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2023/day_19.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &'static str) -> usize {
    let (workflow_strings, objects) = lines.split("\n\n").collect_tuple().unwrap();
    let mut workflows: FxHashMap<String, Vec<Rule>> = FxHashMap::default();
    for workflow_string in workflow_strings.lines() {
        let (name, rules) = rules_parser(workflow_string);
        workflows.insert(name, rules);
    }
    let mut object_vec = Vec::new();
    for object_string in objects.lines() {
        object_vec.push(Object::from_str(object_string).unwrap());
    }
    let mut sum = 0;
    for obj in object_vec.iter() {
        let mut finalized = false;
        let mut rule: &WorkChain = workflows.get("in").unwrap();
        let mut rule_ind = 0;
        while !finalized {
            let res = rule[rule_ind](*obj);
            match res {
                RuleResult::Accepted => {
                    finalized = true;
                    sum += obj.sum();
                }
                RuleResult::Rejected => finalized = true,
                RuleResult::NextRule => rule_ind += 1,
                RuleResult::NextWorkflow(a) => {
                    rule = workflows.get(&a).unwrap();
                    rule_ind = 0
                }
            }
        }
    }

    sum as usize
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RuleResult {
    NextRule,
    NextWorkflow(String),
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug)]
struct Object {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

impl Object {
    fn sum(self) -> isize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Object {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .trim_matches(|p| p == '{' || p == '}')
            .split(',')
            .collect();
        let x: isize = parts[0].split('=').nth(1).unwrap().parse()?;
        let m: isize = parts[1].split('=').nth(1).unwrap().parse()?;
        let a: isize = parts[2].split('=').nth(1).unwrap().parse()?;
        let s: isize = parts[3].split('=').nth(1).unwrap().parse()?;

        Ok(Object { x, m, a, s })
    }
}

type Rule = Box<dyn Fn(Object) -> RuleResult>;
type WorkChain = Vec<Rule>;

//Converts a workflow into a name and vec of rules
fn rules_parser(workflow: &'static str) -> (String, Vec<Rule>) {
    let mut individual_rule_strings: Vec<&'static str> = workflow
        .trim_matches(|p| p == '{' || p == '}')
        .split(',')
        .collect();
    let (workflow_name, first_rule) = individual_rule_strings[0]
        .split("{")
        .collect_tuple()
        .unwrap();
    let mut rules_vec = Vec::new();
    individual_rule_strings = individual_rule_strings.drain(1..).collect_vec();
    individual_rule_strings.insert(0, first_rule);
    for part in individual_rule_strings.iter() {
        rules_vec.push(str_to_rule(*part));
    }
    let workflow_name = workflow_name.to_owned();
    return (workflow_name, rules_vec);
}

fn str_to_rule(rule_str: &'static str) -> Rule {
    Box::new(|o: Object| -> RuleResult {
        let parts: Vec<&str> = rule_str
            .split(|c| c == '<' || c == '>' || c == '=')
            .collect();
        if parts.len() == 1 {
            match parts[0] {
                "A" => return RuleResult::Accepted,
                "R" => return RuleResult::Rejected,
                _ => return RuleResult::NextWorkflow(parts[0].to_owned()),
            }
        }
        let property = parts[0];
        let object_prop = match property {
            "x" => o.x,
            "m" => o.m,
            "a" => o.a,
            "s" => o.s,
            _ => panic!("wrong property format!"),
        };
        let comparator = rule_str
            .chars()
            .find(|&c| c == '<' || c == '>' || c == '=')
            .unwrap();
        let value_and_return: Vec<&str> = parts[1].split(":").collect();
        let value = value_and_return[0].parse::<isize>().unwrap();
        let return_string = value_and_return[1];

        return match comparator {
            '<' => {
                if object_prop < value {
                    match return_string {
                        "A" => RuleResult::Accepted,
                        "R" => RuleResult::Rejected,
                        _ => RuleResult::NextWorkflow(return_string.to_string()),
                    }
                } else {
                    RuleResult::NextRule
                }
            }
            '>' => {
                if object_prop > value {
                    match return_string {
                        "A" => RuleResult::Accepted,
                        "R" => RuleResult::Rejected,
                        _ => RuleResult::NextWorkflow(return_string.to_string()),
                    }
                } else {
                    RuleResult::NextRule
                }
            }
            '=' => {
                if object_prop == value {
                    match return_string {
                        "A" => RuleResult::Accepted,
                        "R" => RuleResult::Rejected,
                        _ => RuleResult::NextWorkflow(return_string.to_string()),
                    }
                } else {
                    RuleResult::NextRule
                }
            }
            _ => panic!(),
        };
    }) as Box<dyn Fn(Object) -> RuleResult>
}
