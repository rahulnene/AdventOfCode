use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2018/day_15_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let game = Game::new(lines);
    dbg!(game.get_turn_order());
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum MapContent {
    Wall,
    Open,
    Elf(usize),
    Goblin(usize),
}

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct Game {
    map: FxHashMap<Position, MapContent>,
}

impl Game {
    fn new(lines: &str) -> Self {
        let mut map = FxHashMap::default();
        for (y, line) in lines.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let content = match c {
                    '#' => MapContent::Wall,
                    '.' => MapContent::Open,
                    'E' => MapContent::Elf(200),
                    'G' => MapContent::Goblin(200),
                    _ => panic!("Invalid map content"),
                };
                map.insert((x, y), content);
            }
        }
        Game { map }
    }

    fn get_hit(&mut self, position: Position) {
        match self.map.get(&position) {
            Some(MapContent::Elf(hp)) => {
                if *hp <= 3 {
                    self.map.insert(position, MapContent::Open);
                } else {
                    self.map.insert(position, MapContent::Elf(hp - 3));
                }
            }
            Some(MapContent::Goblin(hp)) => {
                if *hp <= 3 {
                    self.map.insert(position, MapContent::Open);
                } else {
                    self.map.insert(position, MapContent::Goblin(hp - 3));
                }
            }
            _ => panic!("Invalid map content"),
        };
    }

    fn get_turn_order(&self) -> Vec<Position> {
        let mut positions = self
            .map
            .iter()
            .filter(|(_, v)| **v != MapContent::Open && **v != MapContent::Wall)
            .map(|(p, _)| p)
            .cloned()
            .collect::<Vec<_>>();
        positions.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        positions
    }

    fn get_in_range(&self, actor_pos: Position) -> Vec<Position> {
        let enemy_type = match self.map.get(&actor_pos) {
            Some(MapContent::Elf(_)) => MapContent::Elf(0),
            Some(MapContent::Goblin(_)) => MapContent::Goblin(0),
            _ => panic!("Invalid map content"),
        };
        let mut in_range = vec![];
        let enemies = self
            .map
            .iter()
            .filter(|(_, v)| **v == enemy_type)
            .map(|(p, _)| p)
            .collect_vec();
        for enemy in enemies {
            let mut enemy_neighbours = vec![];
            enemy_neighbours.push((enemy.0, enemy.1 - 1));
            enemy_neighbours.push((enemy.0 - 1, enemy.1));
            enemy_neighbours.push((enemy.0 + 1, enemy.1));
            enemy_neighbours.push((enemy.0, enemy.1 + 1));
            
        }
        in_range
    }
}
