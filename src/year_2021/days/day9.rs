use std::fmt::Debug;
pub fn solution(part: usize) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_9.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let now = std::time::Instant::now();
    let map = HeightMap::new(lines);
    let mut sum = 0;
    for point in map.map.iter() {
        if point.height == 9 {
            continue;
        }
        if map
            .get_neighbors(point.loc)
            .iter()
            .filter_map(|f| *f)
            .all(|f| map.get_height(&f) >= point.height)
        {
            sum += point.height + 1;
        }
    }
    println!("Time: {:?}", now.elapsed());
    sum
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    loc: Location,
    height: usize,
}

#[derive(Clone)]
struct HeightMap {
    height: usize,
    width: usize,
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
        HeightMap {
            height: lines.lines().count(),
            width: lines.lines().next().unwrap().len(),
            map,
        }
    }

    fn get_height(&self, loc: &Location) -> usize {
        self.map
            .iter()
            .find(|p| p.loc == *loc)
            .map(|p| p.height)
            .unwrap()
    }

    fn get_neighbors(&self, center: Location) -> Vec<Option<Location>> {
        vec![
            center.get_up(self.height, self.width),
            center.get_down(self.height, self.width),
            center.get_left(self.height, self.width),
            center.get_right(self.height, self.width),
        ]
    }
}

impl Debug for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = String::new();
        map.push('\n');
        for row in 0..self.height {
            for col in 0..self.width {
                let point = self.map.iter().find(|p| p.loc == Location { row, col });
                match point {
                    Some(p) => map.push_str(&p.height.to_string()),
                    None => map.push(' '),
                }
            }
            map.push('\n');
        }
        write!(f, "{}", map)
    }
}
