use std::time::{Duration, Instant};

use fxhash::FxHashMap;
use itertools::{Itertools, Group};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2022/day_23_test.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut grid = FxHashMap::default();
    for (y, line) in lines.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                (x as isize, y as isize),
                match c {
                    '#' => Contains::Elf,
                    '.' => Contains::Empty,
                    _ => panic!("Unknown char {}", c),
                },
            );
        }
    }
    let mut proposals = FxHashMap::default();
    for elf in grid.iter().filter(|c| c.1 == &Contains::Elf) {
        let mut count = 0;
        let north_neighbor = grid
            .get(&(elf.0 .0, elf.0 .1 - 1))
            .unwrap_or(&Contains::Empty);
        let south_neighbor = grid
            .get(&(elf.0 .0, elf.0 .1 + 1))
            .unwrap_or(&Contains::Empty);
        let west_neighbor = grid
            .get(&(elf.0 .0 - 1, elf.0 .1))
            .unwrap_or(&Contains::Empty);
        let east_neighbor = grid
            .get(&(elf.0 .0 + 1, elf.0 .1))
            .unwrap_or(&Contains::Empty);
        let nw_neighbor = grid
            .get(&(elf.0 .0 - 1, elf.0 .1 - 1))
            .unwrap_or(&Contains::Empty);
        let ne_neighbor = grid
            .get(&(elf.0 .0 + 1, elf.0 .1 - 1))
            .unwrap_or(&Contains::Empty);
        let sw_neighbor = grid
            .get(&(elf.0 .0 - 1, elf.0 .1 + 1))
            .unwrap_or(&Contains::Empty);
        let se_neighbor = grid
            .get(&(elf.0 .0 + 1, elf.0 .1 + 1))
            .unwrap_or(&Contains::Empty);
        let north_neighbors = [north_neighbor, nw_neighbor, ne_neighbor];
        let south_neighbors = [south_neighbor, sw_neighbor, se_neighbor];
        let west_neighbors = [west_neighbor, nw_neighbor, sw_neighbor];
        let east_neighbors = [east_neighbor, ne_neighbor, se_neighbor];
        let neighbors = [
            north_neighbor,
            nw_neighbor,
            ne_neighbor,
            south_neighbor,
            sw_neighbor,
            se_neighbor,
            west_neighbor,
            east_neighbor,
        ];
        if neighbors.iter().filter(|n| n == &&&Contains::Elf).count() == 0 {
            {}
        } else {
            if north_neighbors
                .iter()
                .filter(|n| n == &&&Contains::Elf)
                .count()
                == 0
            {
                proposals.insert((elf.0 .0, elf.0 .1 - 1), elf.0);
            } else if south_neighbors
                .iter()
                .filter(|n| n == &&&Contains::Elf)
                .count()
                == 0
            {
                proposals.insert((elf.0 .0, elf.0 .1 + 1), elf.0);
            } else if west_neighbors
                .iter()
                .filter(|n| n == &&&Contains::Elf)
                .count()
                == 0
            {
                proposals.insert((elf.0 .0 - 1, elf.0 .1), elf.0);
            } else if east_neighbors
                .iter()
                .filter(|n| n == &&&Contains::Elf)
                .count()
                == 0
            {
                proposals.insert((elf.0 .0 + 1, elf.0 .1), elf.0);
            } else {
            }
        }
        let solo_proposals = proposals.keys().counts();
        let solo_proposals = solo_proposals
            .iter()
            .filter(|p| *p.1 == 1)
            .collect::<Vec<_>>();
        for solo_proposal in solo_proposals {
            let proposer = proposals.get(solo_proposal.0).unwrap();
            grid.insert(**solo_proposal.0, Contains::Elf);
            grid.insert(**proposer, Contains::Empty);
        }
    }
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Contains {
    Elf,
    #[default]
    Empty,
}
