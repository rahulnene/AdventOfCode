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
    let mut problems = Vec::new();
    for line in lines.lines() {
        problems.push(Problem::from_str(line));
    }
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
        for (i, &num) in self.numbers.iter().enumerate().skip(1) {
            match operators[i - 1] {
                Operation::Add => result += num,
                Operation::Multiply => result *= num,
                Operation::Concantenate => {
                    let num_str = num.to_string();
                    let result_str = result.to_string();
                    result = format!("{}{}", result_str, num_str).parse().unwrap();
                }
            }
        }
        result == self.target
    }

    fn is_possible_p1(&self) -> usize {
        let mut operators = vec![Operation::Add; self.numbers.len() - 1];
        loop {
            if self.check(operators.clone()) {
                return self.target;
            }
            let mut i = operators.len() - 1;
            loop {
                match operators[i] {
                    Operation::Add => {
                        operators[i] = Operation::Multiply;
                        break;
                    }
                    Operation::Multiply => {
                        operators[i] = Operation::Add;
                        if i == 0 {
                            return 0;
                        }
                        i -= 1;
                    }
                    Operation::Concantenate => {}
                }
            }
        }
    }

    fn is_possible_p2(&self) -> usize {
        let mut operators = vec![Operation::Add; self.numbers.len() - 1];
        loop {
            if self.check(operators.clone()) {
                return self.target;
            }
            let mut i = operators.len() - 1;
            loop {
                match operators[i] {
                    Operation::Add => {
                        operators[i] = Operation::Multiply;
                        break;
                    }
                    Operation::Multiply => {
                        operators[i] = Operation::Concantenate;
                        break;
                    }
                    Operation::Concantenate => {
                        operators[i] = Operation::Add;
                        if i == 0 {
                            return 0;
                        }
                        i -= 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concantenate,
}
