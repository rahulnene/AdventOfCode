use std::time::Instant;

use fxhash::FxHashMap;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_18.txt");
    let mut area = Area::parse(lines);
    match part {
        1 => solve(10, &mut area),
        2 => solve(1000000000, &mut area),
        _ => 1,
    }
}

//still very slow
fn solve(mins: usize, area: &mut Area) -> usize {
    for _ in 0..mins {
        let now = Instant::now();
        area.update();
        println!("{:?}", now.elapsed());
    }
    area.calculate_value()
}

type Coordinates = (usize, usize);

#[derive(Debug, PartialEq, Clone, Default)]
struct Area {
    grid: FxHashMap<Coordinates, AcreContents>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum AcreContents {
    Open,
    Trees,
    Lumberyard,
    OutOfBounds,
}

impl Area {
    fn get(&self, (x, y): (isize, isize)) -> AcreContents {
        if x >= 0 && y >= 0 {
            let (x, y) = (x as usize, y as usize);
            if x < self.width && y < self.height {
                return *self.grid.get(&(x, y)).unwrap();
            };
        }
        AcreContents::OutOfBounds
    }

    fn set(&mut self, (x, y): Coordinates, contents: AcreContents) {
        self.grid.insert((x, y), contents);
    }

    fn get_neighbors(&self, (x, y): Coordinates) -> Vec<AcreContents> {
        let mut neighbors = Vec::new();
        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                neighbors.push(self.get((x as isize + x_offset, y as isize + y_offset)));
            }
        }
        neighbors
    }

    fn parse(lines: &str) -> Self {
        let mut area = Area::default();
        for (y, line) in lines.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let contents = match c {
                    '.' => AcreContents::Open,
                    '|' => AcreContents::Trees,
                    '#' => AcreContents::Lumberyard,
                    _ => panic!("Invalid character"),
                };
                area.set((x, y), contents);
            }
        }
        area.width = lines.lines().next().unwrap().len();
        area.height = lines.lines().count();
        area
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let acre = self.get((x as isize, y as isize));
                let c = match acre {
                    AcreContents::Open => '.',
                    AcreContents::Trees => '|',
                    AcreContents::Lumberyard => '#',
                    AcreContents::OutOfBounds => ' ',
                };
                print!("{}", c);
            }
            println!();
        }
    }

    fn update(&mut self) {
        let mut next_grid = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let acre = self.get((x as isize, y as isize));
                let neighbors = self.get_neighbors((x, y));
                let mut next_acre = acre;
                match acre {
                    AcreContents::Open => {
                        if neighbors
                            .iter()
                            .filter(|&&a| a == AcreContents::Trees)
                            .count()
                            >= 3
                        {
                            next_acre = AcreContents::Trees;
                        }
                    }
                    AcreContents::Trees => {
                        if neighbors
                            .iter()
                            .filter(|&&a| a == AcreContents::Lumberyard)
                            .count()
                            >= 3
                        {
                            next_acre = AcreContents::Lumberyard;
                        }
                    }
                    AcreContents::Lumberyard => {
                        if neighbors
                            .iter()
                            .filter(|&&a| a == AcreContents::Lumberyard)
                            .count()
                            == 0
                            || neighbors
                                .iter()
                                .filter(|&&a| a == AcreContents::Trees)
                                .count()
                                == 0
                        {
                            next_acre = AcreContents::Open;
                        }
                    }
                    AcreContents::OutOfBounds => {}
                }
                next_grid.insert((x, y), next_acre);
            }
        }
        self.grid = next_grid;
    }

    fn calculate_value(&self) -> usize {
        let mut trees = 0;
        let mut lumberyards = 0;
        for acre in self.grid.values() {
            match acre {
                AcreContents::Trees => trees += 1,
                AcreContents::Lumberyard => lumberyards += 1,
                _ => {}
            }
        }
        trees * lumberyards
    }
}
