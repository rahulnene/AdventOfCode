pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_22.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut cave = Cave::init_from_str(lines);
    cave.update_geological_indices();
    cave.update_region_types();
    cave.risk_level()
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Cave {
    depth: usize,
    target: (usize, usize),
    region_types: Vec<Vec<RegionType>>,
    geological_indices: Vec<Vec<usize>>,
}
impl Cave {
    fn init_from_str(lines: &str) -> Self {
        let cave = Cave::default();
        let (depth_str, target_str) = lines.split_once('\n').unwrap();
        let depth = depth_str.split_once(' ').unwrap().1.parse().unwrap();
        let (x, y) = target_str
            .split_once(' ')
            .unwrap()
            .1
            .split_once(',')
            .unwrap();
        let target = (x.parse().unwrap(), y.parse().unwrap());
        Cave {
            depth,
            target,
            ..cave
        }
    }

    fn update_geological_indices(&mut self) {
        let mut indices = vec![vec![0; self.target.0 + 1]; self.target.1 + 1];
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                let index = if (x, y) == (0, 0) || (x, y) == self.target {
                    0
                } else if y == 0 {
                    x * 16807
                } else if x == 0 {
                    y * 48271
                } else {
                    indices[y][x - 1] * indices[y - 1][x]
                };
                indices[y][x] = (index + self.depth) % 20183;
            }
        }
        self.geological_indices = indices;
    }

    fn update_region_types(&mut self) {
        let mut types = vec![vec![RegionType::Rocky; self.target.0 + 1]; self.target.1 + 1];
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                types[y][x] = match self.geological_indices[y][x] % 3 {
                    0 => RegionType::Rocky,
                    1 => RegionType::Wet,
                    2 => RegionType::Narrow,
                    _ => unreachable!(),
                }
            }
        }
        self.region_types = types;
    }

    fn risk_level(&self) -> usize {
        let mut risk = 0;
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                risk += self.region_types[y][x] as usize;
            }
        }
        risk
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RegionType {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}
