use crate::util::read_lines;
use itertools::Itertools;

pub fn solution(part: u8) -> u32 {
    let lines = read_lines("./problem_inputs/day4.txt").unwrap();
    match part {
        1 => calculate(lines, &contains),
        2 => calculate(lines, &overlaps),
        _ => 0,
    }
}

#[derive(Copy, Clone)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn new(given: &str) -> Self {
        let (start, end) = given
            .split('-')
            .map(|f| f.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Assignment { start, end }
    }
}

fn contains(a1: Assignment, a2: Assignment) -> bool {
    (a1.start <= a2.start && a1.end >= a2.end) || (a1.start >= a2.start && a1.end <= a2.end)
}

fn overlaps(a1: Assignment, a2: Assignment) -> bool {
    !(a1.end < a2.start || a1.start > a2.end)
}

fn calculate(
    lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    f: &dyn Fn(Assignment, Assignment) -> bool,
) -> u32 {
    lines
        .flatten()
        .filter(|line| {
            let (assignment1, assignment2) = line.split(',').collect_tuple().unwrap();
            f(Assignment::new(assignment1), Assignment::new(assignment2))
        })
        .count() as u32
}
