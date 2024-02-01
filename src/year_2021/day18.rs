use std::time::{Duration, Instant};

use itertools::Itertools;

const LINES: &str = include_str!("../../problem_inputs_2021/day_18.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let to_add = LINES.lines().map(tokenize).collect_vec();
    (solve01(&to_add), solve02(&to_add))
}

fn solve01(to_add: &[Vec<Token>]) -> (usize, Duration) {
    let now = Instant::now();
    let mut to_add = to_add.to_owned();
    let mut to_add = to_add.iter_mut();
    let snail_num = to_add.next().unwrap();
    for num in to_add {
        *snail_num = add_and_reduce(snail_num, num);
    }
    let ans = magnitude(snail_num);
    (ans, now.elapsed())
}

fn solve02(to_add: &[Vec<Token>]) -> (usize, Duration) {
    let now = Instant::now();
    let to_add = to_add.to_owned();
    let combos = to_add.iter().permutations(2);
    let ans = combos
        .map(|c| {
            let mut s1 = c[0].to_owned();
            let mut s2 = c[1].to_owned();
            let ans = add_and_reduce(&mut s1, &mut s2);
            magnitude(&ans)
        })
        .max()
        .unwrap();

    (ans, now.elapsed())
}

fn add_and_reduce(snail_num1: &mut Vec<Token>, snail_num2: &mut Vec<Token>) -> Vec<Token> {
    let mut ans = add(snail_num1, snail_num2);
    while reduce(&mut ans) {}
    ans
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    LBr,
    RBr,
    Comma,
    Num(usize),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut num_str = String::new();
    while let Some(c) = chars.next() {
        match c {
            '[' => tokens.push(Token::LBr),
            ']' => tokens.push(Token::RBr),
            ',' => tokens.push(Token::Comma),
            ' ' => {}
            _ => {
                num_str.push(c);
                if let Some(&next) = chars.peek() {
                    if !next.is_numeric() {
                        tokens.push(Token::Num(num_str.parse().unwrap()));
                        num_str.clear();
                    }
                }
            }
        }
    }
    tokens
}

fn reduce(tokens: &mut Vec<Token>) -> bool {
    if !explode(tokens) {
        return split(tokens);
    }
    true
}

fn explode(tokens: &mut Vec<Token>) -> bool {
    let mut pointer = 0;
    let mut stack = Vec::new();
    let mut depth = 0;
    let mut last_seen = (None, None);
    loop {
        if pointer > tokens.len() - 1 {
            break;
        }
        match tokens[pointer] {
            Token::LBr => {
                depth += 1;
            }
            Token::RBr => {
                depth -= 1;
            }
            Token::Num(x) => {
                if depth == 5 {
                    stack.push(x);
                } else {
                    last_seen = (Some(pointer), Some(x));
                }
                if stack.len() >= 2 {
                    if last_seen.0.is_some() {
                        tokens[last_seen.0.unwrap()] = Token::Num(last_seen.1.unwrap() + stack[0]);
                    }

                    let second_num = tokens
                        .iter()
                        .skip(pointer + 1)
                        .find_position(|t| matches!(t, Token::Num(_)));
                    if let Some((pos, num)) = second_num {
                        let Token::Num(num) = num else {
                            panic!("Expected num")
                        };
                        tokens[pos + pointer + 1] = Token::Num(num + stack[1]);
                    }

                    tokens.drain(pointer - 3..=pointer + 1);
                    tokens.insert(pointer - 3, Token::Num(0));
                    return true;
                }
            }
            _ => {}
        }
        pointer += 1;
    }
    false
}
fn split(tokens: &mut Vec<Token>) -> bool {
    let position = tokens.iter().find_position(|t| {
        if let Token::Num(x) = t {
            *x >= 10
        } else {
            false
        }
    });
    if let Some((pos, num)) = position {
        let Token::Num(num) = num else {
            panic!("Expected num")
        };
        let left_split = num / 2;
        let right_split = num - left_split;
        let to_insert = vec![
            Token::LBr,
            Token::Num(left_split),
            Token::Comma,
            Token::Num(right_split),
            Token::RBr,
        ];
        tokens.splice(pos..=pos, to_insert);
        true
    } else {
        false
    }
}
fn add(snail_num1: &mut Vec<Token>, snail_num2: &mut Vec<Token>) -> Vec<Token> {
    let mut ans = Vec::new();
    ans.push(Token::LBr);
    ans.append(snail_num1);
    ans.push(Token::Comma);
    ans.append(snail_num2);
    ans.push(Token::RBr);
    ans
}

fn magnitude(snail_num: &[Token]) -> usize {
    let mut snail_num = snail_num.to_owned();
    loop {
        let mut changed = false;
        if snail_num.len() == 1 {
            if let Token::Num(x) = snail_num[0] {
                return x;
            }
        }
        let mut pointer = 0;
        loop {
            let view = snail_num[pointer..pointer + 5].to_vec();
            if let [Token::LBr, Token::Num(x), Token::Comma, Token::Num(y), Token::RBr] =
                view.as_slice()
            {
                changed = true;
                snail_num.drain(pointer..pointer + 5);
                snail_num.insert(pointer, Token::Num(3 * x + 2 * y));
            }
            pointer += 1;
            if snail_num.len() == 1 {
                if let Token::Num(x) = snail_num[0] {
                    return x;
                }
            }
            if pointer > snail_num.len() - 5 {
                break;
            }
        }
        if !changed {
            break;
        }
    }
    0
}
