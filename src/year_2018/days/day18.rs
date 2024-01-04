use std::time::Instant;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_18.txt");
    let mut area = Area::parse(lines);
    match part {
        1 => solve(10, &mut area),
        2 => solve(1000000000, &mut area),
        _ => 1,
    }
}

fn solve(mins: usize, area: &mut Area) -> usize {
    for _ in 0..mins {
        let now = Instant::now();
        area.update();
        println!("{:?}", now.elapsed());
    }
    area.calculate_value()
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Area {
    grid: Vec<Vec<AcreContents>>,
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
    fn get(&self, x: isize, y: isize) -> AcreContents {
        if x >= 0 && y >= 0 {
            let (x, y) = (x as usize, y as usize);
            if x < self.width && y < self.height {
                return self.grid[y][x];
            };
        }
        AcreContents::OutOfBounds
    }

    fn set(&mut self, x: usize, y: usize, contents: AcreContents) {
        self.grid[y][x] = contents;
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<AcreContents> {
        let mut neighbors = Vec::new();
        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                neighbors.push(self.get(x as isize + x_offset, y as isize + y_offset));
            }
        }
        neighbors
    }

    fn parse(lines: &str) -> Self {
        let mut grid = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in lines.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let contents = match c {
                    '.' => AcreContents::Open,
                    '|' => AcreContents::Trees,
                    '#' => AcreContents::Lumberyard,
                    _ => panic!("Invalid character in input"),
                };
                row.push(contents);
            }
            width = row.len();
            grid.push(row);
            height += 1;
        }
        Self {
            grid,
            width,
            height,
        }
    }

    fn print(&self) {
        for row in &self.grid {
            for acre in row {
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
                let neighbors = self.get_neighbors(x, y);
                let acre = self.get(x as isize, y as isize);
                let next_acre = match acre {
                    AcreContents::Open => {
                        if neighbors
                            .iter()
                            .filter(|&&a| a == AcreContents::Trees)
                            .count()
                            >= 3
                        {
                            AcreContents::Trees
                        } else {
                            AcreContents::Open
                        }
                    }
                    AcreContents::Trees => {
                        if neighbors
                            .iter()
                            .filter(|&&a| a == AcreContents::Lumberyard)
                            .count()
                            >= 3
                        {
                            AcreContents::Lumberyard
                        } else {
                            AcreContents::Trees
                        }
                    }
                    AcreContents::Lumberyard => {
                        if neighbors
                            .iter()
                            .filter(|&&a| a == AcreContents::Lumberyard)
                            .count()
                            >= 1
                            && neighbors
                                .iter()
                                .filter(|&&a| a == AcreContents::Trees)
                                .count()
                                >= 1
                        {
                            AcreContents::Lumberyard
                        } else {
                            AcreContents::Open
                        }
                    }
                    AcreContents::OutOfBounds => AcreContents::OutOfBounds,
                };
                next_grid[y][x] = next_acre;
            }
        }
        self.grid = next_grid;
    }

    fn calculate_value(&self) -> usize {
        let (t, l) = self
            .grid
            .iter()
            .flatten()
            .fold((0, 0), |(t, l), &a| match a {
                AcreContents::Trees => (t + 1, l),
                AcreContents::Lumberyard => (t, l + 1),
                _ => (t, l),
            });
        t * l
    }
}
