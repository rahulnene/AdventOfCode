use std::fmt::{self, Debug, Display};
use std::time::Instant;

pub fn solution(part: u8) -> isize {
    let now = Instant::now();
    let lines = include_str!("../../problem_inputs/day21.txt");
    let monkeys = parse_input(lines);
    let root = monkeys.iter().find(|m| m.name == "root").unwrap();
    // let left_monkey = &root.value;
    // dbg!(left_monkey);
    let part_1 = resolve(root, &Box::new(&monkeys));
    println!("Time: {}ms", now.elapsed().as_millis());
    for monkey in monkeys {
        if monkey.name == "humn" {}
    }

    match part {
        1 => part_1,
        2 => 2,
        _ => 0,
    }
}

fn parse_input(lines: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for line in lines.split("\n") {
        let name = line[0..4].to_string();
        let rest = &line[6..];
        if !rest.contains(" ") {
            monkeys.push(Monkey {
                name,
                value: Box::new(Value::Val(rest.parse::<isize>().unwrap())),
            });
        } else {
            let mut split = rest.split(" ");
            let left = split.next().unwrap().to_string();
            let op = match split.next().unwrap() {
                "+" => Box::new(|a: isize, b: isize| a + b),
                "-" => Box::new(|a: isize, b: isize| a - b) as Box<dyn Fn(isize, isize) -> isize>,
                "*" => Box::new(|a: isize, b: isize| a * b),
                "/" => Box::new(|a: isize, b: isize| a / b),
                _ => panic!("Invalid operation"),
            };
            let right = split.next().unwrap().to_string();
            monkeys.push(Monkey {
                name,
                value: Box::new(Value::Operation((
                    Monkey {
                        name: left,
                        value: Box::new(Value::Val(0)),
                    },
                    op,
                    Monkey {
                        name: right,
                        value: Box::new(Value::Val(0)),
                    },
                ))),
            });
        }
    }
    let monkeys_ptr = Box::new(monkeys);
    *monkeys_ptr
}

fn resolve(current_monkey: &Monkey, monkeys: &Box<&Vec<Monkey>>) -> isize {
    match current_monkey.value.as_ref() {
        Value::Val(val) => *val,
        Value::Operation((left_monkey, op, right_monkey)) => {
            let left_monkey = monkeys
                .iter()
                .find(|m| m.name == *left_monkey.name)
                .unwrap();
            let right_monkey = monkeys
                .iter()
                .find(|m| m.name == *right_monkey.name)
                .unwrap();
            let left_val = resolve(left_monkey, &monkeys);
            let right_val = resolve(right_monkey, &monkeys);
            op(left_val, right_val)
        }
    }
}
enum Value {
    Val(isize),
    Operation((Monkey, Box<dyn Fn(isize, isize) -> isize>, Monkey)),
}

struct Monkey {
    name: String,
    value: Box<Value>,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Monkey {{ name: {}, value: {:?} }}",
            self.name, self.value
        )
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Val(val) => write!(f, "Value::Val({})", val),
            Value::Operation((l, _, r)) => write!(f, "{} {}", l, r),
        }
    }
}
