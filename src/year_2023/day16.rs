use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2023/day_16.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let contraption = Contraption::from_str(LINES);
    (solve01(&contraption), solve02(&contraption))
}

fn solve01(contraption: &Contraption) -> (usize, Duration) {
    let now = Instant::now();
    let energy_count = run_sim(Beam::default(), contraption);
    (energy_count, now.elapsed())
}

fn solve02(contraption: &Contraption) -> (usize, Duration) {
    let now = Instant::now();
    let bounds = (contraption.bounds.0, contraption.bounds.1);

    let energy_counts = (0..bounds.0)
        .into_par_iter()
        .flat_map(|x| {
            (0..bounds.1).into_par_iter().filter_map(move |y| {
                let mut local_counts = Vec::new();
                if x == 0 {
                    let beam = Beam::new((x, y), Direction::Right);
                    let energy_count = run_sim(beam, contraption);
                    if energy_count > 0 {
                        local_counts.push(energy_count);
                    }
                }
                if y == 0 {
                    let beam = Beam::new((x, y), Direction::Down);
                    let energy_count = run_sim(beam, contraption);
                    if energy_count > 0 {
                        local_counts.push(energy_count);
                    }
                }
                if x == bounds.0 - 1 {
                    let beam = Beam::new((x, y), Direction::Left);
                    let energy_count = run_sim(beam, contraption);
                    if energy_count > 0 {
                        local_counts.push(energy_count);
                    }
                }
                if y == bounds.1 - 1 {
                    let beam = Beam::new((x, y), Direction::Up);
                    let energy_count = run_sim(beam, contraption);
                    if energy_count > 0 {
                        local_counts.push(energy_count);
                    }
                }
                Some(local_counts)
            })
        })
        .collect::<Vec<Vec<usize>>>();
    let ans = *energy_counts.iter().flatten().max().unwrap();
    (ans, now.elapsed())
}

type Position = (isize, isize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Content {
    Empty,
    MirrorFrontSlash,
    MirrorBackSlash,
    HorizontalSplitter,
    VerticalSplitter,
}

impl Content {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::MirrorFrontSlash,
            '\\' => Self::MirrorBackSlash,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => panic!("Invalid character: {}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Contraption {
    layout: FxHashMap<Position, Content>,
    bounds: (isize, isize),
}

impl Contraption {
    fn from_str(s: &str) -> Self {
        let mut layout = FxHashMap::default();
        let mut bounds = (0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = (x as isize, y as isize);
                layout.insert(position, Content::from_char(c));
                bounds.0 = bounds.0.max(x as isize);
                bounds.1 = bounds.1.max(y as isize);
            }
        }
        let mut beams = FxHashSet::default();
        beams.insert(Beam::default());
        Self { layout, bounds }
    }

    fn update(&self, beams: &mut FxHashSet<Beam>, energized: &mut FxHashSet<Position>) {
        let mut new_beams = FxHashSet::default();
        for beam in beams.iter() {
            energized.insert(beam.position);
            new_beams.extend(beam.update(self));
        }
        // println!("{:?}", new_beams);
        *beams = new_beams;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn default() -> Self {
        Self {
            position: (0, 0),
            direction: Direction::Right,
        }
    }

    fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    fn update_direction(&mut self, content: Content, first_split: bool) {
        match content {
            Content::Empty => {}
            Content::MirrorFrontSlash => match self.direction {
                Direction::Up => self.direction = Direction::Right,
                Direction::Down => self.direction = Direction::Left,
                Direction::Left => self.direction = Direction::Down,
                Direction::Right => self.direction = Direction::Up,
            },
            Content::MirrorBackSlash => match self.direction {
                Direction::Up => self.direction = Direction::Left,
                Direction::Down => self.direction = Direction::Right,
                Direction::Left => self.direction = Direction::Up,
                Direction::Right => self.direction = Direction::Down,
            },
            Content::HorizontalSplitter => match self.direction {
                Direction::Up => {
                    if first_split {
                        self.direction = Direction::Left
                    } else {
                        self.direction = Direction::Right
                    }
                }
                Direction::Down => {
                    if first_split {
                        self.direction = Direction::Right
                    } else {
                        self.direction = Direction::Left
                    }
                }
                _ => {}
            },
            Content::VerticalSplitter => match self.direction {
                Direction::Left => {
                    if first_split {
                        self.direction = Direction::Down
                    } else {
                        self.direction = Direction::Up
                    }
                }
                Direction::Right => {
                    if first_split {
                        self.direction = Direction::Up
                    } else {
                        self.direction = Direction::Down
                    }
                }
                _ => {}
            },
        }
    }

    fn update(&self, contraption: &Contraption) -> Vec<Self> {
        let content = contraption.layout.get(&self.position).unwrap();
        let new_beams = match content {
            Content::Empty => vec![Self::new(self.position, self.direction)],
            Content::MirrorFrontSlash | Content::MirrorBackSlash => {
                let mut new_beam = *self;
                new_beam.position = self.position;
                new_beam.update_direction(*content, false);
                vec![new_beam]
            }
            Content::HorizontalSplitter | Content::VerticalSplitter => {
                let mut new_beam1 = *self;
                let mut new_beam2 = *self;
                new_beam1.position = self.position;
                new_beam2.position = self.position;
                new_beam1.update_direction(*content, true);
                new_beam2.update_direction(*content, false);
                vec![new_beam1, new_beam2]
            }
        };
        // dbg!(&new_beams);
        let ans = new_beams
            .iter()
            .map(|b| {
                let (x, y) = b.position;
                let (dx, dy) = match b.direction {
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                };
                let new_position = (x + dx, y + dy);
                if new_position.0 < 0
                    || new_position.0 > contraption.bounds.0
                    || new_position.1 < 0
                    || new_position.1 > contraption.bounds.1
                {
                    None
                } else {
                    Some(Self::new(new_position, b.direction))
                }
            })
            .filter_map(|f| f)
            .collect_vec();
        // dbg!(&ans);
        ans
    }
}

fn run_sim(beam: Beam, contraption: &Contraption) -> usize {
    let mut energized = FxHashSet::default();
    let mut lag = 0;
    let mut beams = FxHashSet::default();
    beams.insert(beam);
    let mut energy_count = 0;
    while beams.len() > 0 {
        let old_energy_count = energy_count;
        contraption.update(&mut beams, &mut energized);
        energy_count = energized.len();
        if old_energy_count == energy_count {
            lag += 1;
            if lag > 10 {
                break;
            }
        } else {
            lag = 0;
        }
    }
    energy_count
}
