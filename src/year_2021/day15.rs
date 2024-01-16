use std::fmt::Debug;

pub fn solution(part: usize) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_15.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let map = Map::new(lines);
    map.shortest_route()
}

fn solve02(lines: &str) -> usize {
    let map = Map::new_2(lines);
    map.shortest_route()
}

#[derive(Eq, PartialEq, Clone)]
struct Map {
    map: Vec<Vec<usize>>,
}

impl Map {
    fn new(lines: &str) -> Self {
        let map: Vec<Vec<usize>> = lines
            .lines()
            .map(|l| {
                l.chars()
                    .map(|f| (f.to_digit(10).unwrap() % 10_u32) as usize)
                    .collect()
            })
            .collect();
        Map { map }
    }

    fn new_2(lines: &str) -> Self {
        let map: Vec<Vec<usize>> = lines
            .lines()
            .map(|l| {
                l.chars()
                    .map(|f| (f.to_digit(10).unwrap() % 10_u32) as usize)
                    .collect()
            })
            .collect();
        let (dim_h, dim_w) = (map.len(), map[0].len());
        let mut large_map = vec![vec![0; dim_w * 5]; dim_h * 5];
        for h in 0..5 * dim_h {
            for w in 0..5 * dim_w {
                let new_num = h / dim_h + w / dim_w + map[h % dim_h][w % dim_w];
                large_map[h][w] = if new_num > 9 { new_num - 9 } else { new_num };
            }
        }
        Map { map: large_map }
    }

    fn shortest_route(&self) -> usize {
        let now = std::time::Instant::now();
        let mut distances = Map {
            map: vec![vec![usize::MAX; self.map[0].len()]; self.map.len()],
        };
        let dim = distances.map.len() - 1;
        distances.map[dim][dim] = self.get(
            (self.map.len() - 1) as isize,
            (self.map[0].len() - 1) as isize,
        );
        for _ in 0..dim / 25 {
            for col in (0..self.map.len()).rev() {
                for row in (0..self.map[0].len()).rev() {
                    let (row, col) = (row as isize, col as isize);
                    if row != (self.map.len() - 1) as isize
                        || col != (self.map[0].len() - 1) as isize
                    {
                        distances.map[row as usize][col as usize] = self.get(row, col)
                            + distances
                                .get(row + 1, col)
                                .min(distances.get(row, col + 1))
                                .min(distances.get(row - 1, col))
                                .min(distances.get(row, col - 1));
                    }
                }
            }
        }
        println!("Time: {:?}", now.elapsed());
        distances.get(0, 0) - self.get(0, 0)
    }

    fn get(&self, row: isize, col: isize) -> usize {
        let (row, col) = (row as usize, col as usize);
        if row >= self.map.len() || col >= self.map[0].len() {
            return usize::MAX;
        }
        self.map[row][col]
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for line in &self.map {
            for c in line {
                write!(f, "{c} ")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
