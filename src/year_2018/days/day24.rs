pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_24_test.txt");
    match part {
        1 => solve01(lines),
        // 2 => solve(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut lobby = Lobby::new();
    for line in lines.lines() {
        lobby.flip_str(line);
    }
    lobby.count_black()
}

fn solve02(lines: &str) -> usize {
    let mut lobby = Lobby::new();
    for line in lines.lines() {
        lobby.flip_str(line);
    }

    0
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    q: i32,
    r: i32,
    white: bool,
    to_be_flipped: bool,
}

impl Tile {
    fn new(q: i32, r: i32, white: bool) -> Self {
        Self {
            q,
            r,
            white,
            to_be_flipped: false,
        }
    }

    fn flip(&mut self) {
        self.white = !self.white;
    }
}

#[derive(Debug, Clone)]
struct Lobby {
    tiles: Vec<Tile>,
}

impl Lobby {
    fn new() -> Self {
        Self { tiles: Vec::new() }
    }

    fn get_tile_at_mut(&mut self, (q, r): (i32, i32)) -> Option<&mut Tile> {
        self.tiles.iter_mut().find(|t| t.q == q && t.r == r)
    }

    fn get_tile_at(&self, (q, r): (i32, i32)) -> Option<&Tile> {
        self.tiles.iter().find(|t| t.q == q && t.r == r)
    }

    fn flip_str(&mut self, s: &str) {
        let (mut q, mut r) = (0, 0);
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            let dir = match c {
                'e' => "e",
                'w' => "w",
                's' => {
                    let next = chars.next().unwrap();
                    match next {
                        'e' => "se",
                        'w' => "sw",
                        _ => "",
                    }
                }
                'n' => {
                    let next = chars.next().unwrap();
                    match next {
                        'e' => "ne",
                        'w' => "nw",
                        _ => "",
                    }
                }
                _ => "",
            };
            let (del_q, del_r) = convert_to_coord(dir);
            q += del_q;
            r += del_r;
        }
        if let Some(tile) = self.get_tile_at_mut((q, r)) {
            tile.flip();
        } else {
            self.tiles.push(Tile::new(q, r, false));
        }
    }

    fn count_black(&self) -> usize {
        self.tiles.iter().filter(|t| !t.white).count()
    }

    fn count_black_neighbors(&self, (q, r): (i32, i32)) -> usize {
        let mut sum = 0;
        for (del_q, del_r) in [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)] {
            if let Some(tile) = self.get_tile_at((q + del_q, r + del_r)) {
                if !tile.white {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn count_white_neighbors(&self, (q, r): (i32, i32)) -> usize {
        6 - self.count_black_neighbors((q, r))
    }

    fn cycle(&mut self) {}
}

fn convert_to_coord(dir: &str) -> (i32, i32) {
    match dir {
        "e" => (1, 0),
        "se" => (0, 1),
        "sw" => (-1, 1),
        "w" => (-1, 0),
        "nw" => (0, -1),
        "ne" => (1, -1),
        _ => (0, 0),
    }
}
