use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2022/day_15.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut sensors = Vec::new();
    for line in lines.lines() {
        let parts = line.split_whitespace().map(str::trim).collect::<Vec<_>>();
        let sensor_x_coord = parts[2]
            .chars()
            .filter(|c| c.is_numeric() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        let sensor_y_coord = parts[3]
            .chars()
            .filter(|c| c.is_numeric() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        let beacon_x_coord = parts[8]
            .chars()
            .filter(|c| c.is_numeric() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        let beacon_y_coord = parts[9]
            .chars()
            .filter(|c| c.is_numeric() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        let range = ((sensor_x_coord - beacon_x_coord).abs()
            + (sensor_y_coord - beacon_y_coord).abs()) as usize;
        sensors.push(Sensor::new(sensor_x_coord, sensor_y_coord, range))
    }
    let ans = (-4000000..4000000)
        .filter(|x| sensors.iter().any(|s| s.distance(*x, 2000000) <= s.range))
        .count()
        - 1;
    (ans, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut sensors = Vec::new();
    for line in lines.lines() {
        let parts = line.split_whitespace().map(str::trim).collect::<Vec<_>>();
        let sensor_x_coord = parts[2]
            .chars()
            .filter(|c| c.is_numeric() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        let sensor_y_coord = parts[3]
            .chars()
            .filter(|c| c.is_numeric() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        let beacon_x_coord = parts[8]
            .chars()
            .filter(|c| c.is_numeric() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        let beacon_y_coord = parts[9]
            .chars()
            .filter(|c| c.is_numeric() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        let range = ((sensor_x_coord - beacon_x_coord).abs()
            + (sensor_y_coord - beacon_y_coord).abs()) as usize;
        sensors.push(Sensor::new(sensor_x_coord, sensor_y_coord, range))
    }
    let ans = (0..4000000)
        .combinations(2)
        .map(|x| (x[0], x[1]))
        .filter(|x| sensors.iter().any(|s| s.distance(x.0, x.1) <= s.range))
        .count()
        - 1;
    (ans, now.elapsed())
}

#[derive(Debug)]
struct Sensor {
    x_coord: isize,
    y_coord: isize,
    range: usize,
}

impl Sensor {
    fn new(x_coord: isize, y_coord: isize, range: usize) -> Self {
        Self {
            x_coord,
            y_coord,
            range,
        }
    }

    fn distance(&self, x_coord: isize, y_coord: isize) -> usize {
        ((self.x_coord - x_coord).abs() + (self.y_coord - y_coord).abs()) as usize
    }
}
