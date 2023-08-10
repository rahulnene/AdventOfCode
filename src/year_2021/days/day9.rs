use fxhash::FxHashMap;
use std::fmt::Debug;

pub fn solution(part: usize) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_9.txt");
    let now = std::time::Instant::now();
    let map = HeightMap::new(lines);
    let map_size = (lines.lines().count(), lines.lines().next().unwrap().len());
    println!("Map parsed in {:?}", now.elapsed());
    match part {
        1 => solve01(&map, map_size),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(map: &HeightMap, map_size: (usize, usize)) -> usize {
    let now = std::time::Instant::now();
    let sum = map
        .map
        .iter()
        .filter(|point| {
            point
                .loc
                .get_neighbors(map_size)
                .iter()
                .filter_map(|f| *f)
                .all(|f| map.get_height(&f) > point.height)
        })
        .map(|point| point.height + 1)
        .sum();

    println!("Part 1 finished in {:?}", now.elapsed());
    sum
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn get_neighbors(&self, (row_bounds, col_bounds): (usize, usize)) -> Vec<Option<Location>> {
        let north = self.get_up(row_bounds, col_bounds);
        let south = self.get_down(row_bounds, col_bounds);
        let east = self.get_right(row_bounds, col_bounds);
        let west = self.get_left(row_bounds, col_bounds);
        vec![north, south, east, west]
    }
    fn get_up(&self, _: usize, _: usize) -> Option<Location> {
        self.row
            .checked_sub(1)
            .map(|row| Location { row, col: self.col })
    }
    fn get_down(&self, row_bounds: usize, _: usize) -> Option<Location> {
        if self.row + 1 < row_bounds {
            Some(Location {
                row: self.row + 1,
                col: self.col,
            })
        } else {
            None
        }
    }
    fn get_right(&self, _: usize, col_bounds: usize) -> Option<Location> {
        if self.col + 1 < col_bounds {
            Some(Location {
                row: self.row,
                col: self.col + 1,
            })
        } else {
            None
        }
    }
    fn get_left(&self, _: usize, _: usize) -> Option<Location> {
        self.col
            .checked_sub(1)
            .map(|col| Location { row: self.row, col })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    loc: Location,
    height: usize,
}

#[derive(Clone)]
struct HeightMap {
    loc_to_height: FxHashMap<Location, usize>,
    map: Vec<Point>,
}

impl HeightMap {
    fn new(lines: &str) -> Self {
        let mut map: Vec<Point> = Vec::new();
        for (row, line) in lines.lines().enumerate() {
            for (col, height) in line.chars().enumerate() {
                let height = height.to_digit(10).unwrap() as usize;
                let loc = Location { row, col };
                let point = Point { loc, height };
                map.push(point);
            }
        }
        let loc_to_height = map.iter().map(|p| (p.loc, p.height)).collect();

        Self { map, loc_to_height }
    }

    fn get_height(&self, loc: &Location) -> usize {
        *self.loc_to_height.get(loc).unwrap()
    }
}

impl Debug for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = self.map.clone();
        map.sort();
        let mut s = String::new();
        for point in map {
            s.push_str(&format!("{:?}", point));
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
