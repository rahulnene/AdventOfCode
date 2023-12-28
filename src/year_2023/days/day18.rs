use fxhash::FxHashSet;
use itertools::Itertools;
use std::{
    cmp,
    fmt::Debug,
    str::FromStr,
};

//UNFINISHED

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2023/day_18.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut trench = Trench::new();
    for line in lines.lines() {
        let direction = DigInstruction::from_str(line).unwrap();
        trench.dig(direction);
    }
    trench.outline = trench.cubes.clone();
    dbg!(trench.volume());
    trench.fill_interior();
    dbg!(trench.volume());
    // trench.print();
    0
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err("bad direction".to_owned()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DigInstruction {
    dir: Direction,
    distance: usize,
}

impl FromStr for DigInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, dist_str, color_str) = s.split(' ').collect_tuple().unwrap();
        let dir = Direction::from_str(dir_str)?;
        let distance = usize::from_str(dist_str).map_err(|_| "bad distance")?;
        Ok(DigInstruction { dir, distance })
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Cube {
    x: isize,
    y: isize,
}

impl Debug for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl Cube {
    fn from(x: isize, y: isize) -> Cube {
        Cube { x, y }
    }
}

#[derive(Debug, Clone)]
struct Trench {
    cubes: FxHashSet<Cube>,
    outline: FxHashSet<Cube>,
    current_pos: Cube,
}

impl Trench {
    fn new() -> Trench {
        let mut cubes: FxHashSet<Cube> = FxHashSet::default();
        cubes.insert(Cube::default());
        Trench {
            cubes: cubes.clone(),
            outline: cubes,
            current_pos: Cube::default(),
        }
    }

    fn print(&self) {
        let bounds = self.find_bounds();
        let mut repr: Vec<Vec<char>> = Vec::new();
        for y in bounds.1 .0..=bounds.1 .1 {
            let mut row = Vec::new();

            for x in bounds.0 .0..=bounds.0 .1 {
                let current_cube = Cube::from(x, y);

                if self.cubes.contains(&current_cube) {
                    row.push('#')
                } else {
                    row.push('.')
                }
            }
            repr.push(row);
        }
        repr.reverse();
        fancy_print(repr);
    }

    fn volume(&self) -> usize {
        self.cubes.len()
    }

    fn is_inside(&self, cube: Cube) -> bool {
        let bounds = self.find_bounds();
        let mut intersect_count = 0;
        for x in (cube.x + 1)..=(bounds.0.1) {
            let check_cube = Cube::from(x, cube.y);
            if self.outline.contains(&check_cube) {
                intersect_count += 1;
            }
        }
        intersect_count % 2 != 0
    }

    fn dig(&mut self, instr: DigInstruction) {
        for _ in 0..instr.distance {
            match instr.dir {
                Direction::Left => self.current_pos.x -= 1,
                Direction::Right => self.current_pos.x += 1,
                Direction::Up => self.current_pos.y += 1,
                Direction::Down => self.current_pos.y -= 1,
            }
            self.cubes.insert(self.current_pos);
        }
    }

    fn find_bounds(&self) -> ((isize, isize), (isize, isize)) {
        self.cubes
            .iter()
            .fold(None, |acc, cube| {
                Some(match acc {
                    None => ((cube.x, cube.y), (cube.x, cube.y)),
                    Some(((min_x, max_x), (min_y, max_y))) => (
                        (cmp::min(min_x, cube.x), cmp::max(max_x, cube.x)),
                        (cmp::min(min_y, cube.y), cmp::max(max_y, cube.y)),
                    ),
                })
            })
            .unwrap()
    }

    fn fill_interior(&mut self) {
        let bounds = self.find_bounds();
        let mut inside_trench;
        for y in bounds.1 .0..=bounds.1 .1 {
            inside_trench = false;
            for x in bounds.0 .0..=bounds.0 .1 {
                let current_cube = Cube::from(x, y);
                // Inside trench
                if self.is_inside(current_cube) {
                    self.cubes.insert(current_cube);
                }
            }
        }
    }
}

fn fancy_print(repr: Vec<Vec<char>>) {
    for row in repr {
        println!("{:?}", row)
    }
}
