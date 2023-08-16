use regex::Regex;
pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_17.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let now = std::time::Instant::now();
    let target = Target::from_str(lines);
    let mut largest_y = usize::MIN;
    for vx in 0..100 {
        for vy in 0..100 {
            let mut probe = Probe::new(vx, vy);
            if probe.simulate(&target) {
                largest_y = largest_y.max(probe.max_y as usize);
            }
        }
    }

    println!("Time: {:?}", now.elapsed());
    largest_y
}

fn solve02(lines: &str) -> usize {
    let now = std::time::Instant::now();
    let target = Target::from_str(lines);
    let mut good_launches = 0;
    for vx in 0..400 {
        for vy in -100..1000 {
            let mut probe = Probe::new(vx, vy);
            if probe.simulate(&target) {
                good_launches += 1;
            }
        }
    }
    println!("Time: {:?}", now.elapsed());
    good_launches
}

#[derive(Debug, Clone, Copy)]
struct Coords {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Probe {
    loc: Coords,
    vx: isize,
    vy: isize,
    max_y: isize,
}

impl Probe {
    fn new(vx: isize, vy: isize) -> Self {
        Self {
            loc: Coords { x: 0, y: 0 },
            vx,
            vy,
            max_y: 0,
        }
    }

    fn update(&mut self) {
        self.loc.x += self.vx;
        self.loc.y += self.vy;
        self.vy -= 1;
        if self.vx > 0 {
            self.vx -= 1;
        } else if self.vx < 0 {
            self.vx += 1;
        }
    }

    fn on_target(&self, target: &Target) -> bool {
        target.is_in_bounds(self)
    }

    fn simulate(&mut self, target: &Target) -> bool {
        for _ in 0.. {
            self.update();
            if self.on_target(&target) {
                return true;
            }
            if self.loc.y < target.min_y && self.vy < 0 {
                return false;
            }
            self.max_y = self.max_y.max(self.loc.y);
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Target {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Target {
    fn new(bounds: &[isize]) -> Self {
        Self {
            min_x: bounds[0],
            max_x: bounds[1],
            min_y: bounds[2],
            max_y: bounds[3],
        }
    }

    fn from_str(lines: &str) -> Self {
        let bounds: Vec<isize> = Regex::new(
            r"target area: x=(?P<x_min>\d+)..(?P<x_max>\d+), y=(?P<y_min>-?\d+)..(?P<y_max>-?\d+)",
        )
        .unwrap()
        .captures(lines)
        .unwrap()
        .iter()
        .skip(1)
        .map(|f| isize::from_str_radix(f.unwrap().as_str(), 10).unwrap())
        .collect();
        Target::new(&bounds)
    }

    fn is_in_bounds(&self, probe: &Probe) -> bool {
        probe.loc.x >= self.min_x
            && probe.loc.x <= self.max_x
            && probe.loc.y >= self.min_y
            && probe.loc.y <= self.max_y
    }
}
