use std::time::{Duration, Instant};

use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2020/day_24_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let mut lobby = Lobby::new();
    for line in LINES.lines() {
        lobby.flip_str(line);
    }
    (solve01(&lobby), solve02(&mut lobby))
}

fn solve01(lobby: &Lobby) -> (usize, Duration) {
    let now = Instant::now();
    println!("{:?}", &lobby.tiles);
    let ans = lobby.count_black();
    (ans, now.elapsed())
}

fn solve02(lobby: &mut Lobby) -> (usize, Duration) {
    let now = Instant::now();
    lobby.cycle();
    (0, now.elapsed())
}

type Position = (i32, i32);

#[derive(Debug, Clone)]
struct Lobby {
    tiles: FxHashMap<Position, bool>,
}

impl Lobby {
    fn new() -> Self {
        Self {
            tiles: FxHashMap::default(),
        }
    }

    fn get(&self, pos: Position) -> bool {
        *self.tiles.get(&pos).unwrap_or(&false)
    }

    fn cycle(&mut self) {
        let old_lobby = self.clone();
        let min_q = old_lobby.tiles.keys().max_by_key(|j| j.0).unwrap().0;
        let max_q = old_lobby.tiles.keys().max_by_key(|j| j.0).unwrap().0;
        let min_r = old_lobby.tiles.keys().min_by_key(|t| t.1).unwrap().1;
        let max_r = old_lobby.tiles.keys().max_by_key(|t| t.1).unwrap().1;
        for q in min_q..=max_q {
            for r in min_r..max_r {
                let pos = (q, r);
                let black_neighbor_count = old_lobby.count_black_neighbors(pos);
                let is_tile_white = old_lobby.get(pos);
                if (is_tile_white && black_neighbor_count == 2)
                    || (!is_tile_white && black_neighbor_count != 1)
                {
                    self.tiles.insert(pos, !is_tile_white);
                }
            }
        }
        dbg!(self.count_black());
    }

    fn flip_str(&mut self, s: &str) {
        let mut pos = (0, 0);
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            let dir = match c {
                'e' => "e",
                'w' => "w",
                's' => {
                    let next = chars.next().unwrap();
                    match next {
                        'e' => "se",
                        'w' => "sw",
                        _ => "",
                    }
                }
                'n' => {
                    let next = chars.next().unwrap();
                    match next {
                        'e' => "ne",
                        'w' => "nw",
                        _ => "",
                    }
                }
                _ => "",
            };
            let (del_q, del_r) = convert_to_coord(dir);
            pos.0 += del_q;
            pos.1 += del_r;
        }
        if self.tiles.get(&pos).is_none() {
            self.tiles.insert(pos, false);
        } else {
            let tile = self.tiles.get(&pos).unwrap();
            self.tiles.insert(pos, !*tile);
        }
    }

    fn count_black(&self) -> usize {
        self.tiles.values().filter(|c| !**c).count()
    }

    fn count_black_neighbors(&self, (q, r): Position) -> usize {
        let mut sum = 0;
        for (del_q, del_r) in [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)] {
            let white = self.get((q + del_q, r + del_r));
            if !white {
                sum += 1;
            }
        }
        sum
    }
}

fn convert_to_coord(dir: &str) -> (i32, i32) {
    match dir {
        "e" => (1, 0),
        "se" => (0, 1),
        "sw" => (-1, 1),
        "w" => (-1, 0),
        "nw" => (0, -1),
        "ne" => (1, -1),
        _ => (0, 0),
    }
}
