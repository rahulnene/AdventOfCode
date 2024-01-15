use std::time::{Duration, Instant};

use fxhash::FxHashMap;
use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2022/day_14.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut cavern = Cavern {
        contents: FxHashMap::default(),
    };
    for line in lines.lines() {
        let edge_ends = line
            .split("->")
            .map(str::trim)
            .map(|p| {
                p.split(',')
                    .map(|c| c.parse::<isize>().unwrap())
                    .collect_tuple::<(isize, isize)>()
                    .unwrap()
            })
            .tuple_windows::<(_, _)>()
            .sorted_unstable()
            .collect_vec();
        for (a, b) in edge_ends {
            if a.0 == b.0 {
                let (start, end) = (a.1.min(b.1), a.1.max(b.1));
                for y in start..=end {
                    cavern.contents.insert((a.0, y), Contents::Rock);
                }
            } else {
                let (start, end) = (a.0.min(b.0), a.0.max(b.0));
                for x in start..=end {
                    cavern.contents.insert((x, a.1), Contents::Rock);
                }
            }
        }
    }
    let bottom = cavern.contents.keys().max_by_key(|f| f.1).unwrap().1;
    cavern.contents.insert((500, 0), Contents::FallingSand);
    loop {
        // dbg!(cavern
        //     .contents
        //     .iter()
        //     .find(|(_, g)| g == &&Contents::FallingSand));
        let sand_position = *cavern
            .contents
            .iter()
            .find(|(_, g)| g == &&Contents::FallingSand)
            .unwrap()
            .0;
        let below = (sand_position.0, sand_position.1 + 1);
        let left = (sand_position.0 - 1, sand_position.1 + 1);
        let right = (sand_position.0 + 1, sand_position.1 + 1);

        cavern.contents.remove(&sand_position);
        if *cavern.contents.get(&below).unwrap_or(&Contents::default()) == Contents::Empty {
            cavern.contents.insert(below, Contents::FallingSand);
        } else if *cavern.contents.get(&left).unwrap_or(&Contents::default()) == Contents::Empty {
            cavern.contents.insert(left, Contents::FallingSand);
        } else if *cavern.contents.get(&right).unwrap_or(&Contents::default()) == Contents::Empty {
            cavern.contents.insert(right, Contents::FallingSand);
        } else {
            cavern.contents.insert(sand_position, Contents::AtRestSand);
            cavern.contents.insert((500, 0), Contents::FallingSand);
        }
        if cavern
            .contents
            .iter()
            .any(|(p, g)| g == &Contents::FallingSand && p.1 > bottom)
        {
            return (
                cavern
                    .contents
                    .values()
                    .filter(|c| **c == Contents::AtRestSand)
                    .count(),
                now.elapsed(),
            );
        }
    }
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut cavern = Cavern {
        contents: FxHashMap::default(),
    };
    for line in lines.lines() {
        let edge_ends = line
            .split("->")
            .map(str::trim)
            .map(|p| {
                p.split(',')
                    .map(|c| c.parse::<isize>().unwrap())
                    .collect_tuple::<(isize, isize)>()
                    .unwrap()
            })
            .tuple_windows::<(_, _)>()
            .sorted_unstable()
            .collect_vec();
        for (a, b) in edge_ends {
            if a.0 == b.0 {
                let (start, end) = (a.1.min(b.1), a.1.max(b.1));
                for y in start..=end {
                    cavern.contents.insert((a.0, y), Contents::Rock);
                }
            } else {
                let (start, end) = (a.0.min(b.0), a.0.max(b.0));
                for x in start..=end {
                    cavern.contents.insert((x, a.1), Contents::Rock);
                }
            }
        }
    }
    let floor = cavern.contents.keys().max_by_key(|f| f.1).unwrap().1 + 2;
    dbg!(floor);
    for x in 0..1000 {
        cavern.contents.insert((x, floor), Contents::Rock);
    }
    cavern.contents.insert((500, 0), Contents::FallingSand);
    loop {
        let sand_position = *cavern
            .contents
            .iter()
            .find(|(_, g)| g == &&Contents::FallingSand)
            .unwrap()
            .0;
        let below = (sand_position.0, sand_position.1 + 1);
        let left = (sand_position.0 - 1, sand_position.1 + 1);
        let right = (sand_position.0 + 1, sand_position.1 + 1);

        cavern.contents.remove(&sand_position);
        if *cavern.contents.get(&below).unwrap_or(&Contents::default()) == Contents::Empty {
            cavern.contents.insert(below, Contents::FallingSand);
        } else if *cavern.contents.get(&left).unwrap_or(&Contents::default()) == Contents::Empty {
            cavern.contents.insert(left, Contents::FallingSand);
        } else if *cavern.contents.get(&right).unwrap_or(&Contents::default()) == Contents::Empty {
            cavern.contents.insert(right, Contents::FallingSand);
        } else {
            cavern.contents.insert(sand_position, Contents::AtRestSand);
            if cavern.contents.contains_key(&(500, 0)) {
                return (
                    cavern
                        .contents
                        .values()
                        .filter(|c| **c == Contents::AtRestSand)
                        .count(),
                    now.elapsed(),
                );
            }
            cavern.contents.insert((500, 0), Contents::FallingSand);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cavern {
    contents: FxHashMap<(isize, isize), Contents>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
enum Contents {
    Rock,
    FallingSand,
    AtRestSand,
    #[default]
    Empty,
}
