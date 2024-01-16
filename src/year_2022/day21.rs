use std::collections::{HashMap, VecDeque};

pub fn solution(part: u8) -> isize {
    let lines = include_str!("../../../problem_inputs/day21.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => -1,
    }
}

fn solve01(lines: &str) -> isize {
    let now = std::time::Instant::now();
    let (monkeys, root_index, _) = parse(lines);
    println!("Time taken: {:?}", now.elapsed());
    monkeys[root_index].get_value(&monkeys)
}

fn solve02(lines: &str) -> isize {
    let now = std::time::Instant::now();
    let (monkeys, root_index, humn_index) = parse(lines);
    let mut queue: VecDeque<(usize, isize)> = VecDeque::new();
    if let Monkey::Expression(i1, i2, _) = monkeys[root_index] {
        queue.push_back((i2, monkeys[i1].get_value(&monkeys)));
        queue.push_back((i1, monkeys[i2].get_value(&monkeys)));
    }

    while let Some((i, expected)) = queue.pop_front() {
        if i == humn_index {
            println!("Time taken: {:?}", now.elapsed());
            return expected;
        }

        if let Monkey::Expression(i1, i2, _) = monkeys[i] {
            queue.push_back((i1, monkeys[i].get_expected1(expected, &monkeys)));
            queue.push_back((i2, monkeys[i].get_expected2(expected, &monkeys)));
        }
    }
    println!("Time Taken: {:?}", now.elapsed());
    -1
}

fn parse(lines: &str) -> (Vec<Monkey>, usize, usize) {
    let id_to_index: HashMap<&str, usize> = HashMap::from_iter(
        lines
            .lines()
            .enumerate()
            .map(|(i, line)| (line.split_once(':').map(|(id, _)| id).unwrap(), i)),
    );

    (
        Vec::from_iter(lines.lines().map(|line| {
            let (_, definition) = line.split_once(": ").unwrap();
            match definition.parse::<isize>() {
                Ok(c) => Monkey::Const(c),
                Err(_) => {
                    let mut parts = definition.split(' ');
                    let i1 = id_to_index.get(parts.next().unwrap()).unwrap();
                    let operator = match parts.next().unwrap() {
                        "+" => Operator::Add,
                        "-" => Operator::Sub,
                        "*" => Operator::Mul,
                        "/" => Operator::Div,
                        _ => panic!("bad input"),
                    };
                    let i2 = id_to_index.get(parts.next().unwrap()).unwrap();
                    Monkey::Expression(*i1, *i2, operator)
                }
            }
        })),
        *id_to_index.get("root").unwrap(),
        *id_to_index.get("humn").unwrap(),
    )
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn calculate(&self, v1: isize, v2: isize) -> isize {
        use Operator::*;
        match self {
            Add => v1 + v2,
            Sub => v1 - v2,
            Mul => v1 * v2,
            Div => v1.checked_div(v2).unwrap_or(0),
        }
    }

    fn reverse_calculate(&self) -> Operator {
        use Operator::*;
        match self {
            Add => Sub,
            Sub => Add,
            Mul => Div,
            Div => Mul,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Monkey {
    Const(isize),
    Expression(usize, usize, Operator),
}

impl Monkey {
    fn get_value(&self, monkeys: &[Monkey]) -> isize {
        match self {
            Monkey::Const(c) => *c,
            Monkey::Expression(i1, i2, op) => op.calculate(
                monkeys[*i1].get_value(monkeys),
                monkeys[*i2].get_value(monkeys),
            ),
        }
    }

    fn get_expected1(&self, expected_result: isize, monkeys: &[Monkey]) -> isize {
        match self {
            Monkey::Const(_) => panic!(),
            Monkey::Expression(_, i2, op) => op
                .reverse_calculate()
                .calculate(expected_result, monkeys[*i2].get_value(monkeys)),
        }
    }

    fn get_expected2(&self, expected_result: isize, monkeys: &[Monkey]) -> isize {
        match self {
            Monkey::Const(_) => panic!(),
            Monkey::Expression(i1, _, op) => {
                let v1 = monkeys[*i1].get_value(monkeys);
                match op {
                    Operator::Add | Operator::Mul => {
                        op.reverse_calculate().calculate(expected_result, v1)
                    }
                    Operator::Sub | Operator::Div => op.calculate(v1, expected_result),
                }
            }
        }
    }
}
