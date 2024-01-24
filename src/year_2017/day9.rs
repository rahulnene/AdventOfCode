use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2017/day_9.txt");
    let mut lines: Vec<char> = lines.chars().rev().collect();
    let group = Group::parse(&mut lines, 0);
    (solve01(&group), solve02(&group))
}

fn solve01(group: &Group) -> (usize, Duration) {
    let now = Instant::now();
    (group.score(), now.elapsed())
}

fn solve02(group: &Group) -> (usize, Duration) {
    let now = Instant::now();
    (group.garbage_count(), now.elapsed())
}

#[derive(Debug, Clone)]
struct Group {
    level: usize,
    canceled_chars: usize,
    children: Vec<Group>,
}

impl Group {
    fn new(level: usize) -> Self {
        Self {
            level,
            canceled_chars: 0,
            children: Vec::new(),
        }
    }

    fn score(&self) -> usize {
        self.children.iter().fold(self.level, |p, n| p + n.score())
    }

    fn garbage_count(&self) -> usize {
        self.children
            .iter()
            .fold(self.canceled_chars, |p, n| p + n.garbage_count())
    }

    fn parse(s: &mut Vec<char>, level: usize) -> Self {
        let mut group = Self::new(level);
        let mut garbage = false;
        let mut ignore = false;
        while let Some(c) = s.pop() {
            if garbage && ignore {
                ignore = false;
            } else if garbage {
                match c {
                    '!' => ignore = true,
                    '>' => garbage = false,
                    _ => group.canceled_chars += 1,
                }
            } else {
                match c {
                    '{' => group.children.push(Self::parse(s, level + 1)),
                    '}' => return group,
                    ',' => {},
                    '<' => garbage = true,
                    _ => (),
                }
            }
        }
        group
    }
}
