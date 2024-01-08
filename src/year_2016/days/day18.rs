pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2016/day_18.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> usize {
    let mut grid = Grid::new(lines);
    while grid.rows.len() < 40 {
        grid.add_next_row();
    }
    // grid.print();
    grid.count_safe()
}

fn solve02(lines: &str) -> usize {
    let mut grid = Grid::new(lines);
    while grid.rows.len() < 400_000 {
        grid.add_next_row();
    }
    // grid.print();
    grid.count_safe()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Safe,
            '^' => Self::Trap,
            _ => panic!("Invalid character in row: {c}"),
        }
    }
}

#[derive(Debug)]
struct Row {
    tiles: Vec<Tile>,
}

impl Row {
    fn new(str: &str) -> Self {
        let tiles = str.chars().map(Tile::new).collect::<Vec<_>>();
        Self { tiles }
    }

    fn read(&self, index: isize) -> Tile {
        if index < 0 || index as usize >= self.tiles.len() {
            Tile::Safe
        } else {
            self.tiles[index as usize]
        }
    }
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    fn new(str: &str) -> Self {
        let row = Row::new(str);
        Self { rows: vec![row] }
    }

    // fn print(&self) {
    //     for row in &self.rows {
    //         for tile in &row.tiles {
    //             match tile {
    //                 Tile::Safe => print!("."),
    //                 Tile::Trap => print!("^"),
    //             }
    //         }
    //         println!();
    //     }
    // }

    fn add_next_row(&mut self) {
        let mut next_row = Vec::with_capacity(self.rows[0].tiles.len());
        let last_row = self.rows.last().unwrap();
        for i in 0..last_row.tiles.len() as isize {
            let left = last_row.read(i - 1);
            let center = last_row.read(i);
            let right = last_row.read(i + 1);
            let tile = match (left, center, right) {
                (Tile::Trap, Tile::Trap | Tile::Safe, Tile::Safe)
                | (Tile::Safe, Tile::Trap | Tile::Safe, Tile::Trap) => Tile::Trap,
                _ => Tile::Safe,
            };
            next_row.push(tile);
        }
        self.rows.push(Row { tiles: next_row });
    }

    fn count_safe(&self) -> usize {
        self.rows
            .iter()
            .map(|f| f.tiles.iter().filter(|t| **t == Tile::Safe).count())
            .sum()
    }
}
