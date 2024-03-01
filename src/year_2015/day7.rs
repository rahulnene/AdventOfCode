use std::time::{Duration, Instant};

use itertools::Itertools;
use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2015/day_7.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    {
        let now = Instant::now();
        let mut wire_values: FxHashMap<String, u16> = FxHashMap::default();
        let mut rule_to_def_map = FxHashMap::default();
        for rule in LINES.lines() {
            let (input, output) = rule.split(" -> ").collect_tuple().unwrap();
            rule_to_def_map.insert(output.to_string(), input.to_string());
        }
        let ans = calculate_wire_value("a", &mut wire_values, &rule_to_def_map);
        let p1_ans = (ans as usize, now.elapsed());
        rule_to_def_map.insert("b".to_string(), ans.to_string());
        let mut wire_values: FxHashMap<String, u16> = FxHashMap::default();
        let ans = calculate_wire_value("a", &mut wire_values, &rule_to_def_map);
        let p2_ans = (ans as usize, now.elapsed());
        (p1_ans, p2_ans)
    }
}

fn calculate_wire_value(
    rule_str: &str,
    wires: &mut FxHashMap<String, u16>,
    rule_to_def: &FxHashMap<String, String>,
) -> u16 {
    if wires.contains_key(rule_str) {
        return *wires.get(rule_str).unwrap();
    }
    if let Ok(value) = rule_str.parse::<u16>() {
        return value;
    }
    let rule = rule_to_def.get(rule_str).unwrap();
    let value = if rule.contains("AND") {
        let (a, b) = rule.split(" AND ").collect_tuple().unwrap();
        calculate_wire_value(a, wires, rule_to_def) & calculate_wire_value(b, wires, rule_to_def)
    } else if rule.contains("OR") {
        let (a, b) = rule.split(" OR ").collect_tuple().unwrap();
        calculate_wire_value(a, wires, rule_to_def) | calculate_wire_value(b, wires, rule_to_def)
    } else if rule.contains("LSHIFT") {
        let (a, b) = rule.split(" LSHIFT ").collect_tuple().unwrap();
        calculate_wire_value(a, wires, rule_to_def) << b.parse::<u16>().unwrap()
    } else if rule.contains("RSHIFT") {
        let (a, b) = rule.split(" RSHIFT ").collect_tuple().unwrap();
        calculate_wire_value(a, wires, rule_to_def) >> b.parse::<u16>().unwrap()
    } else if rule.contains("NOT") {
        let a = rule.split("NOT ").collect_tuple::<(_, _)>().unwrap().1;
        !calculate_wire_value(a, wires, rule_to_def)
    } else if rule.chars().all(|c| c.is_ascii_digit()) {
        rule.parse::<u16>().unwrap()
    } else {
        calculate_wire_value(rule, wires, rule_to_def)
    };
    wires.insert(rule_str.to_string(), value);
    value
}
