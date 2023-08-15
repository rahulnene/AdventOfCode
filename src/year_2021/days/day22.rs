use std::fmt::{Debug, Formatter};

use fxhash::FxHashMap;
use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_22_test.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut reactor = Reactor::new();
    for line in lines.lines() {
        let now = std::time::Instant::now();
        let (command, coords) = line.split(' ').collect_tuple().unwrap();
        let (x_coords, y_coords, z_coords) = coords.split(',').collect_tuple().unwrap();
        let x_coords = get_coords(x_coords);
        let y_coords = get_coords(y_coords);
        let z_coords = get_coords(z_coords);
        if x_coords.0 < -50
            || x_coords.1 > 50
            || y_coords.0 < -50
            || y_coords.1 > 50
            || z_coords.0 < -50
            || z_coords.1 > 50
        {
            continue;
        }
        match command {
            "on" => reactor.turn_on(x_coords, y_coords, z_coords),
            "off" => reactor.turn_off(x_coords, y_coords, z_coords),
            _ => panic!("Invalid command"),
        }
        println!("Time: {:?}", now.elapsed());
    }
    let ans = reactor.cells.values().filter(|&&v| v).count();
    ans
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Reactor {
    cells: FxHashMap<(isize, isize, isize), bool>,
}

impl Reactor {
    fn new() -> Self {
        Self {
            cells: FxHashMap::default(),
        }
    }

    fn turn_on(
        &mut self,
        x_range: (isize, isize),
        y_range: (isize, isize),
        z_range: (isize, isize),
    ) {
        for x in x_range.0..=x_range.1 {
            for y in y_range.0..=y_range.1 {
                for z in z_range.0..=z_range.1 {
                    self.cells.insert((x, y, z), true);
                }
            }
        }
    }
    fn turn_off(
        &mut self,
        x_range: (isize, isize),
        y_range: (isize, isize),
        z_range: (isize, isize),
    ) {
        for x in x_range.0..=x_range.1 {
            for y in y_range.0..=y_range.1 {
                for z in z_range.0..=z_range.1 {
                    self.cells.insert((x, y, z), false);
                }
            }
        }
    }
}

fn get_coords(raw_coords: &str) -> (isize, isize) {
    let (x, y) = raw_coords.split("..").collect_tuple().unwrap();
    (x[2..].parse().unwrap(), y.parse().unwrap())
}

fn solve02(lines: &str) -> usize {
    // dbg!(
    //     Cuboid::new((10, 11), (10, 11), (10, 11), false,).overlap_volume(Cuboid::new(
    //         (10, 10),
    //         (10, 10),
    //         (10, 10),
    //         false,
    //     ))
    // );
    let mut cuboid_list: Vec<Cuboid> = Vec::default();
    for line in lines.lines() {
        let (command, coords) = line.split(' ').collect_tuple().unwrap();
        let (x_coords, y_coords, z_coords) = coords.split(',').collect_tuple().unwrap();
        let x_coords = get_coords(x_coords);
        let y_coords = get_coords(y_coords);
        let z_coords = get_coords(z_coords);
        let new_cuboid = Cuboid::new(x_coords, y_coords, z_coords, command == "on");

        let mut a: Vec<Cuboid> = Vec::default();
        if cuboid_list.len() == 0 {
            a.push(new_cuboid);
        }
        for cub in &cuboid_list {
            if cub.overlap_volume(new_cuboid) > 0 {
                match cub.overlap_cuboid(new_cuboid) {
                    Some(mut c) => {
                        c.set_sign(!cub.pos_cube);
                        a.push(c);
                    }
                    _ => (),
                }
                if new_cuboid.pos_cube {
                    a.push(new_cuboid);
                }
            }
        }

        cuboid_list.extend(a);
        dbg!(
            &cuboid_list,
            cuboid_list.iter().map(|c| c.volume()).collect_vec(),
            cuboid_list.iter().fold(0, |acc, c| acc + c.volume()) as usize
        );
        println!("-------------------------");
    }

    cuboid_list.iter().fold(0, |acc, c| acc + c.volume()) as usize
}
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Cuboid {
    x_range: (isize, isize),
    y_range: (isize, isize),
    z_range: (isize, isize),
    pos_cube: bool,
}

impl Cuboid {
    fn new(
        x_range: (isize, isize),
        y_range: (isize, isize),
        z_range: (isize, isize),
        pos_cube: bool,
    ) -> Self {
        Self {
            x_range,
            y_range,
            z_range,
            pos_cube,
        }
    }
    fn volume(&self) -> isize {
        if self.x_range == (0, 0) && self.y_range == (0, 0) && self.z_range == (0, 0) {
            return 0;
        }
        let x = (self.x_range.1 - self.x_range.0) + 1;
        let y = (self.y_range.1 - self.y_range.0) + 1;
        let z = (self.z_range.1 - self.z_range.0) + 1;
        (x * y * z) * (2 * (self.pos_cube as isize) - 1)
    }

    fn overlap_cuboid(&self, other: Cuboid) -> Option<Cuboid> {
        let x = (
            self.x_range.0.max(other.x_range.0),
            self.x_range.1.min(other.x_range.1),
        );
        let y = (
            self.y_range.0.max(other.y_range.0),
            self.y_range.1.min(other.y_range.1),
        );
        let z = (
            self.z_range.0.max(other.z_range.0),
            self.z_range.1.min(other.z_range.1),
        );
        if x.0 > x.1 || y.0 > y.1 || z.0 > z.1 {
            return None;
        }
        Some(Cuboid::new(x, y, z, true))
    }

    fn overlap_volume(&self, other: Cuboid) -> isize {
        if self.x_range.0 > self.x_range.1 {
            return 0;
        };
        match self.overlap_cuboid(other) {
            Some(c) => c.volume(),
            None => 0,
        }
    }

    fn set_sign(&mut self, sign: bool) {
        self.pos_cube = sign;
    }
}

impl Debug for Cuboid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {:?}, y: {:?}, z: {:?}, pos_cube: {:?}",
            self.x_range, self.y_range, self.z_range, self.pos_cube
        )
    }
}
