use crate::util::read_lines;
use regex::Regex;
use rug::Integer;

pub fn solution(part: u8) -> Integer {
    let lines = read_lines("./problem_inputs/day11.txt").unwrap();
    match part {
        1 => part1(lines),
        2 => part2(lines),
        _ => Integer::from(0),
    }
}

fn part1(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> Integer {
    let re = Regex::new(r"Monkey (?P<id>\d+):\n\s+Starting items: (?P<items>(?:\d+, )*\d+)\n\s+Operation: new = old (?P<operation>.) (?P<operand>.+?)\n\s+Test: divisible by (?P<test>.+?)\n\s+If true: throw to monkey (?P<destination1>\d+)\n\s+If false: throw to monkey (?P<destination2>\d+)").unwrap();
    let given: String = lines
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .join("\n");
    let mut monkeys: Vec<Monkey> = Vec::new();
    for (_, [id, items, operation, operand, test, destination1, destination2]) in
        re.captures_iter(&given).map(|c| c.extract())
    {
        monkeys.push(Monkey::new(
            id,
            items,
            operation,
            operand,
            test,
            destination1,
            destination2,
        ));
    }
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            compute(monkey, &mut monkeys);
        }
    }
    let mut max_inspections: Vec<Integer> = monkeys.iter().map(|m| m.inspections.clone()).collect();
    max_inspections.sort_unstable();
    max_inspections[max_inspections.len() - 1].clone()
        * max_inspections[max_inspections.len() - 2].clone()
}

fn part2(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> Integer {
    let re = Regex::new(r"Monkey (?P<id>\d+):\n\s+Starting items: (?P<items>(?:\d+, )*\d+)\n\s+Operation: new = old (?P<operation>.) (?P<operand>.+?)\n\s+Test: divisible by (?P<test>.+?)\n\s+If true: throw to monkey (?P<destination1>\d+)\n\s+If false: throw to monkey (?P<destination2>\d+)").unwrap();
    let given: String = lines
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .join("\n");
    let mut monkeys: Vec<Monkey> = Vec::new();
    for (_, [id, items, operation, operand, test, destination1, destination2]) in
        re.captures_iter(&given).map(|c| c.extract())
    {
        monkeys.push(Monkey::new(
            id,
            items,
            operation,
            operand,
            test,
            destination1,
            destination2,
        ));
    }
    for _ in 0..1000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            compute2(monkey, &mut monkeys);
        }
    }
    let mut max_inspections: Vec<Integer> = monkeys.iter().map(|m| m.inspections.clone()).collect();
    max_inspections.sort_unstable();
    max_inspections[max_inspections.len() - 1].clone()
        * max_inspections[max_inspections.len() - 2].clone()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Monkey {
    id: usize,
    items: Vec<Integer>,
    test: Integer,
    destination: (Integer, Integer),
    operation: (char, String),
    inspections: Integer,
}

impl Monkey {
    fn new(
        id: &str,
        items: &str,
        operation: &str,
        operand: &str,
        test: &str,
        destination1: &str,
        destination2: &str,
    ) -> Monkey {
        Monkey {
            id: id.parse().unwrap(),
            items: items
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<Integer>>(),
            operation: (operation.chars().next().unwrap(), operand.to_string()),
            test: test.parse().unwrap(),
            destination: (destination1.parse().unwrap(), destination2.parse().unwrap()),
            inspections: Integer::ZERO,
        }
    }
}

fn compute(mut monkey: Monkey, monkeys: &mut [Monkey]) {
    for item in &mut monkey.items {
        let operand = match monkey.operation.1.as_str() {
            "old" => item.clone(),
            _ => monkey.operation.1.parse::<Integer>().unwrap(),
        };
        match monkey.operation.0 {
            '+' => *item += operand,
            '-' => *item -= operand,
            '*' => *item *= operand,
            '/' => *item /= operand,
            _ => panic!("Unknown operation"),
        }
        *item /= 3;
        if item.clone() % monkey.test.clone() == 0 {
            monkeys[monkey.destination.0.to_usize().unwrap()]
                .items
                .push(item.clone());
        } else {
            monkeys[monkey.destination.1.to_usize().unwrap()]
                .items
                .push(item.clone());
        }
        monkeys[monkey.id].inspections += 1;
    }
    monkeys[monkey.id].items.clear();
}

fn compute2(mut monkey: Monkey, monkeys: &mut [Monkey]) {
    for item in &mut monkey.items {
        let operand = match monkey.operation.1.as_str() {
            "old" => item.clone(),
            _ => monkey.operation.1.parse::<Integer>().unwrap(),
        };
        match monkey.operation.0 {
            '+' => *item += operand,
            '-' => *item -= operand,
            '*' => *item *= operand,
            '/' => *item /= operand,
            _ => panic!("Unknown operation"),
        }
        if item.clone() % monkey.test.clone() == 0 {
            monkeys[monkey.destination.0.to_usize().unwrap()]
                .items
                .push(item.clone());
        } else {
            monkeys[monkey.destination.1.to_usize().unwrap()]
                .items
                .push(item.clone());
        }
        monkeys[monkey.id].inspections += 1;
    }
    monkeys[monkey.id].items.clear();
}
