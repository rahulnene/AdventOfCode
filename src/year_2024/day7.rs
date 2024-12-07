use rayon::prelude::*;
use std::time::{Duration, Instant};

pub fn solution(test: bool) -> ((usize, Duration), (usize, Duration)) {
    let lines;
    if test {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_7_test.txt");
    } else {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_7.txt");
    }

    (
        solve(&lines, Problem::is_possible_p1),
        solve(&lines, Problem::is_possible_p2),
    )
}

fn solve<Ch>(lines: &str, checker: Ch) -> (usize, Duration)
where
    Ch: Fn(&Problem) -> usize + Sync + Send,
{
    let now = Instant::now();
    let problems: Vec<Problem> = lines.lines().map(Problem::from_str).collect();
    let ans = problems.par_iter().map(|p| checker(p)).sum::<usize>();

    (ans, now.elapsed())
}

#[derive(Debug, Clone)]
struct Problem {
    target: usize,
    numbers: Vec<usize>,
}

impl Problem {
    fn from_str(input: &str) -> Self {
        let (target, numbers) = input.split_once(": ").unwrap();
        let target = target.parse().unwrap();
        let numbers = numbers
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
        Self { target, numbers }
    }

    fn check(&self, operators: Vec<Operation>) -> bool {
        let mut result = self.numbers[0];
        result =
            self.numbers
                .iter()
                .skip(1)
                .zip(operators.iter())
                .fold(result, |acc, (&num, &op)| match op {
                    Operation::Add => acc + num,
                    Operation::Multiply => acc * num,
                    Operation::Concantenate => acc * 10_usize.pow(get_num_digits(num)) + num,
                });
        result == self.target
    }

    fn is_possible_p1(&self) -> usize {
        self.solve_with_operations(vec![Operation::Add, Operation::Multiply])
    }

    fn is_possible_p2(&self) -> usize {
        if self.is_possible_p1() == self.target {
            return self.target;
        }
        self.solve_with_operations(vec![
            Operation::Add,
            Operation::Multiply,
            Operation::Concantenate,
        ])
    }

    fn solve_with_operations(&self, operations: Vec<Operation>) -> usize {
        let mut operators = vec![operations[0]; self.numbers.len() - 1];
        loop {
            if self.check(operators.clone()) {
                return self.target;
            }
            let mut i = operators.len() - 1;
            loop {
                let next_op = match operators[i] {
                    op if op == *operations.last().unwrap() => operations[0],
                    _ => {
                        operations[operations.iter().position(|&x| x == operators[i]).unwrap() + 1]
                    }
                };
                operators[i] = next_op;
                if next_op != operations[0] {
                    break;
                }
                if i == 0 {
                    return 0;
                }
                i -= 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
    Concantenate,
}

fn get_num_digits(mut num: usize) -> u32 {
    let mut digits: Vec<u8> = Vec::with_capacity(4);

    while num > 0 {
        let n = (num % 10) as u8;
        num /= 10;
        digits.push(n);
    }
    digits.len() as u32
}
