use std::{
    cmp::Ordering,
    env::current_exe,
    time::{Duration, Instant},
};

use fxhash::FxHashSet;
use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2019/day_12.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut a = lines.lines().map(Planet::from_str).collect::<Vec<_>>();
    let planet_count = a.len();
    for _ in 0..1000 {
        for (i, j) in (0..planet_count).tuple_combinations() {
            let other = a[j];
            a[i].apply_gravity(other);
        }
        for (j, i) in (0..planet_count).tuple_combinations() {
            let other = a[j];
            a[i].apply_gravity(other);
        }
        a.iter_mut().for_each(|p| p.apply_velocity());
    }
    let energy = a.iter().map(|p| p.energy()).collect_vec();
    (energy.iter().sum(), now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut a = lines.lines().map(Planet::from_str).collect::<Vec<_>>();
    let mut states = FxHashSet::default();
    let planet_count = a.len();
    for step in 0.. {
        for (i, j) in (0..planet_count).tuple_combinations() {
            let (p1, p2) = (a[i], a[j]);
            a[i].apply_gravity(p2);
            a[j].apply_gravity(p1);
        }
        a.iter_mut().for_each(|p| p.apply_velocity());
        let state = a.iter().map(|p| p.to_string()).join("\n");
        if !states.insert(state) {
            return (step, now.elapsed());
        }
    }
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Planet {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

impl Planet {
    fn new(position: (isize, isize, isize)) -> Self {
        Self {
            position,
            velocity: (0, 0, 0),
        }
    }

    fn from_str(s: &str) -> Self {
        let mut s = s.trim_matches(|c| c == '<' || c == '>');
        s = s.trim_matches(|c| c == 'x' || c == 'y' || c == 'z' || c == '=');
        let mut s = s.split(',');
        let x = s
            .next()
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let y = s
            .next()
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let z = s
            .next()
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        Self::new((x, y, z))
    }

    fn apply_gravity(&mut self, other: Planet) {
        let mut velocity = self.velocity;
        velocity.0 += match self.position.0.cmp(&other.position.0) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        velocity.1 += match self.position.1.cmp(&other.position.1) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        velocity.2 += match self.position.2.cmp(&other.position.2) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        self.velocity = velocity;
    }

    fn apply_velocity(&mut self) {
        let position = self.position;
        let velocity = self.velocity;
        self.position = (
            position.0 + velocity.0,
            position.1 + velocity.1,
            position.2 + velocity.2,
        );
    }

    fn energy(&self) -> usize {
        let potential = self.position.0.abs() + self.position.1.abs() + self.position.2.abs();
        let kinetic = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();
        (potential * kinetic) as usize
    }

    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            self.position.0,
            self.position.1,
            self.position.2,
            self.velocity.0,
            self.velocity.1,
            self.velocity.2
        )
    }
}
