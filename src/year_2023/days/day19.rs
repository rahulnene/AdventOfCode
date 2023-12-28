use fxhash::FxHashMap;
use itertools::Itertools;
use std::str::FromStr;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2023/day_19.txt");
    match part {
        1 => solve01(lines),
        // 2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &'static str) -> usize {
    let (workflow_strings, objects) = lines.split("\n\n").collect_tuple().unwrap();

    let workflows: FxHashMap<&str, Vec<Rule>> =
        workflow_strings.lines().map(rules_parser).collect();

    objects
        .lines()
        .map(|object_string| process_object(&Object::from_str(object_string).unwrap(), &workflows))
        .sum::<isize>() as usize
}

type Rule = Box<dyn Fn(Object) -> RuleResult>;

fn process_object(obj: &Object, workflows: &FxHashMap<&str, Vec<Rule>>) -> isize {
    let mut rule: &Vec<Rule> = workflows.get("in").unwrap();
    let mut rule_ind = 0;
    loop {
        let res = rule[rule_ind](*obj);
        match res {
            RuleResult::Accepted => {
                return obj.sum();
            }
            RuleResult::Rejected => return 0,
            RuleResult::NextRule => rule_ind += 1,
            RuleResult::NextWorkflow(a) => {
                rule = workflows.get(&a).unwrap();
                rule_ind = 0;
            }
        }
    }
}

// fn solve02(lines: &'static str) -> usize {
//     let (workflow_strings, objects) = lines.split("\n\n").collect_tuple().unwrap();
//     let mut workflows: FxHashMap<String, Vec<Rule>> = FxHashMap::default();
//     for workflow_string in workflow_strings.lines() {
//         let (name, rules) = rules_parser(workflow_string);
//         workflows.insert(name, rules);
//     }
//     let mut accepted = Vec::new();
//     for x in 1..4000_isize {
//         let mut obj = Object::from_str("{x=787,m=0,a=0,s=0}").unwrap();
//         obj.x = x;
//         let start = Instant::now();
//         let mut finalized = false;
//         let mut rule: &WorkChain = workflows.get("in").unwrap();
//         let mut rule_ind = 0;
//         while !finalized {
//             let res = rule[rule_ind](obj);
//             match res {
//                 RuleResult::Accepted => {
//                     finalized = true;
//                     accepted.push(true);
//                 }
//                 RuleResult::Rejected => {
//                     finalized = true;
//                     accepted.push(false);
//                 }
//                 RuleResult::NextRule => rule_ind += 1,
//                 RuleResult::NextWorkflow(a) => {
//                     rule = workflows.get(&a).unwrap();
//                     rule_ind = 0
//                 }
//             }
//         }
//     }
//     let (mut last_i, mut last_val) = accepted.iter().enumerate().f
//     for (i, val) in accepted.iter().enumerate().skip(1) {
//         if val != last_val {
//             println!("{val}");
//         }
//         (last_i, last_val) = (i,val);
//     }
//     0
// }

#[derive(Clone, Debug, PartialEq, Eq)]
enum RuleResult {
    NextRule,
    NextWorkflow(&'static str),
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
    fn get_property_from_str(&self, property: &str) -> isize {
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
        let x: isize = parts[0].split('=').nth(1).unwrap().parse()?;
        let m: isize = parts[1].split('=').nth(1).unwrap().parse()?;
        let a: isize = parts[2].split('=').nth(1).unwrap().parse()?;
        let s: isize = parts[3].split('=').nth(1).unwrap().parse()?;

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
    Box::new(|o: Object| -> RuleResult {
        let parts: Vec<&str> = rule_str
            .split(|c| c == '<' || c == '>' || c == '=')
            .collect();
        if parts.len() == 1 {
            match *parts.first().expect("parts vector is empty") {
                "A" => return RuleResult::Accepted,
                "R" => return RuleResult::Rejected,
                _ => return RuleResult::NextWorkflow(parts.first().expect("parts vector is empty")),
            }
        }
        let object_prop = o.get_property_from_str(parts[0]);

        let (value_string, return_string): (&str, &str) =
            parts[1].split(':').collect_tuple().unwrap();
        let value = value_string.parse::<isize>().unwrap();
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
    }) as Box<dyn Fn(Object) -> RuleResult>
}
