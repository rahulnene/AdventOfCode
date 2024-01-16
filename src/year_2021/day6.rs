use std::fmt::Debug;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_6.txt");
    match part {
        1 => solve(lines, 80),
        2 => solve(lines, 256),
        _ => 1,
    }
}

fn solve(lines: &str, days: u16) -> usize {
    let now = std::time::Instant::now();
    let mut school: School = School::from_str(lines);
    for _ in 1..=days {
        school.update();
    }
    println!("Time: {:?}", now.elapsed());
    school.per_day_count.iter().sum()
}

#[derive(Clone, PartialEq, Eq)]
struct School {
    per_day_count: Vec<usize>,
}

impl School {
    fn new() -> Self {
        Self {
            per_day_count: vec![0; 9],
        }
    }

    fn from_str(input: &str) -> Self {
        let mut school = School::new();
        input
            .split(',')
            .map(|f| f.parse().unwrap())
            .for_each(|day_val: usize| school.per_day_count[day_val as usize] += 1);
        school
    }

    fn update(&mut self) {
        let baby_count = self.per_day_count[0];
        for day in 0..=7 {
            self.per_day_count[day] = self.per_day_count[day + 1];
        }
        self.per_day_count[6] += baby_count;
        self.per_day_count[8] = baby_count;
    }
}

impl Debug for School {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for day in self.per_day_count.iter() {
            s.push_str(&format!("{:?}, ", day));
        }
        write!(f, "{}", s)
    }
}
