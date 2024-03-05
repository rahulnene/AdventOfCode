use std::time::{Duration, Instant};

use rustc_hash::FxHashSet;

const LINES: &str = include_str!("../../problem_inputs_2018/day_25.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut constellations: Vec<Constellation> = Vec::new();
    let mut lines = LINES.lines();
    while let Some(star_str) = lines.next() {
        let current = parse_star(star_str);
        let mut added = false;
        for constellation in &mut constellations {
            if constellation.should_be_added(&current) {
                constellation.stars.insert(current);
                added = true;
                break;
            }
        }
        if !added {
            constellations.push(Constellation {
                stars: [current].iter().cloned().collect(),
            });
        }
    }
    let mut old_count = constellations.len();
    loop {
        merge_constellations(&mut constellations);
        if constellations.len() == old_count {
            return (constellations.len(), now.elapsed());
        }
        old_count = constellations.len();
    }
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

type Star = [isize; 4];
#[derive(Clone, Debug)]
struct Constellation {
    stars: FxHashSet<Star>,
}

impl Constellation {
    fn should_be_added(&self, star: &Star) -> bool {
        self.stars.iter().any(|x| distance(x, star) <= 3)
    }
}

fn distance(a: &Star, b: &Star) -> isize {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum()
}

fn parse_star(s: &str) -> Star {
    s.split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<isize>>()
        .try_into()
        .unwrap()
}

fn merge_constellations(constellations: &mut Vec<Constellation>) {
    let mut i = 0;
    while i < constellations.len() {
        let mut j = i + 1;
        while j < constellations.len() {
            if constellations[i]
                .stars
                .iter()
                .any(|x| constellations[j].stars.iter().any(|y| distance(x, y) <= 3))
            {
                let temp = constellations[j].stars.clone();
                constellations[i].stars.extend(temp.iter().cloned());
                constellations.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}
