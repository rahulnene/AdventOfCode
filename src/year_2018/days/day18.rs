pub fn solution(part: usize) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_18.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    lines
        .lines()
        .map(|line| {
            evaluate(&transform01(
                &line.chars().filter(|c| *c != ' ').collect::<Vec<_>>(),
            ))
        })
        .sum()
}

fn solve02(lines: &str) -> usize {
    lines
        .lines()
        .map(|line| {
            evaluate(&transform02(
                &line.chars().filter(|c| *c != ' ').collect::<Vec<_>>(),
            ))
        })
        .sum()
}

//Transform infix string to postfix
fn transform01(expr: &[char]) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();
    let mut postfix: Vec<char> = Vec::new();
    for c in expr.iter() {
        match c {
            '(' => stack.push(*c),
            ')' => {
                while let Some(op) = stack.pop() {
                    if op == '(' {
                        break;
                    }
                    postfix.push(op);
                }
            }
            '+' | '*' => {
                while let Some(op) = stack.pop() {
                    if op == '(' {
                        stack.push(op);
                        break;
                    }
                    postfix.push(op);
                }
                stack.push(*c);
            }
            _ => postfix.push(*c),
        }
    }
    while let Some(op) = stack.pop() {
        postfix.push(op);
    }
    postfix
}

fn transform02(expr: &[char]) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();
    let mut postfix: Vec<char> = Vec::new();
    for c in expr.iter() {
        match c {
            '(' => stack.push(*c),
            ')' => {
                while let Some(op) = stack.pop() {
                    if op == '(' {
                        break;
                    }
                    postfix.push(op);
                }
            }
            '+' => {
                while let Some(op) = stack.pop() {
                    if op == '(' || op == '*' {
                        stack.push(op);
                        break;
                    }
                    postfix.push(op);
                }
                stack.push(*c);
            }
            '*' => {
                while let Some(op) = stack.pop() {
                    if op == '(' {
                        stack.push(op);
                        break;
                    }
                    postfix.push(op);
                }
                stack.push(*c);
            }
            _ => postfix.push(*c),
        }
    }
    while let Some(op) = stack.pop() {
        postfix.push(op);
    }
    postfix
}

fn evaluate(expr: &[char]) -> usize {
    let mut stack: Vec<usize> = Vec::new();
    for c in expr.iter() {
        match c {
            '+' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            '*' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            _ => stack.push(c.to_digit(10).unwrap() as usize),
        }
    }
    stack.pop().unwrap()
}
