use fxhash::FxHashMap;

pub fn solution(part: usize) -> usize {
    let lines = include_str!("../../../problem_inputs_2023/day_16_test.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut floor = parse_floor(lines);
    let (xmax, ymax) = find_bounds(&floor);
    let mut beams = vec![LightBeam::new()];
    let mut deleted_beams = 0;
    loop {
        dbg!(beams.len());
        if beams.len() == deleted_beams {
            break;
        }
        let mut new_beams = Vec::new();
        for beam in &mut beams {
            // dbg!(beam.position);
            let beam_status = beam.step(&mut floor, (xmax, ymax));
            match beam_status {
                BeamStatus::Live => (),
                BeamStatus::Split => new_beams.push(LightBeam {
                    status: BeamStatus::Live,
                    position: beam.position,
                    direction: beam.direction.invert(),
                }),
                BeamStatus::Dead => beam.status = BeamStatus::Dead,
                BeamStatus::JustDied => {
                    dbg! {beam.position};
                    deleted_beams += 1;
                    beam.status = BeamStatus::Dead;
                }
            }
        }
        beams.append(&mut new_beams);
    }
    dbg!(floor.values().filter(|t| t.is_energized()).count());
    0
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum TileContents {
    Empty,
    Mirror(MirrorDirection),
    Spliiter(SplitterDirection),
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum MirrorDirection {
    /*
    / or \
    */
    FrontSlash,
    BackSlash,
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum SplitterDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Tile {
    contents: TileContents,
    energized: bool,
}

impl Tile {
    fn is_energized(&self) -> bool {
        self.energized
    }

    fn energize(&mut self) {
        self.energized = true;
    }
}

type Position = (isize, isize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct LightBeam {
    status: BeamStatus,
    position: Position,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum BeamStatus {
    Live,
    Split,
    Dead,
    JustDied,
}

impl LightBeam {
    fn new() -> LightBeam {
        LightBeam {
            status: BeamStatus::Live,
            position: (0, 0),
            direction: Direction::Right,
        }
    }

    fn hit_frontslash_mirror(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Down,
            Direction::Right => self.direction = Direction::Up,
        }
    }

    fn hit_backslash_mirror(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Left,
            Direction::Down => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }
    fn hit_horizontal_splitter(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            _ => (),
        }
    }
    fn hit_vertical_splitter(&mut self) {
        match self.direction {
            Direction::Left => self.direction = Direction::Down,
            Direction::Right => self.direction = Direction::Right,
            _ => (),
        }
    }

    fn step(
        &mut self,
        floor: &mut FxHashMap<Position, Tile>,
        bounds: (isize, isize),
    ) -> BeamStatus {
        let mut return_status = BeamStatus::Live;
        if self.status == BeamStatus::Dead {
            return self.status;
        }
        if self.position.0 < 0
            || self.position.0 > bounds.0
            || self.position.1 < 0
            || self.position.1 > bounds.1
        {
            return BeamStatus::JustDied;
        }
        let tile_at_pos = floor.get_mut(&self.position).unwrap();
        tile_at_pos.energize();
        match tile_at_pos.contents {
            TileContents::Empty => {}
            TileContents::Mirror(mir_dir) => match mir_dir {
                MirrorDirection::FrontSlash => self.hit_frontslash_mirror(),
                MirrorDirection::BackSlash => self.hit_backslash_mirror(),
            },
            TileContents::Spliiter(spl_dir) => match spl_dir {
                SplitterDirection::Horizontal => {
                    self.hit_horizontal_splitter();
                    return_status = BeamStatus::Split;
                }
                SplitterDirection::Vertical => {
                    self.hit_vertical_splitter();
                    return_status = BeamStatus::Split;
                }
            },
        }

        match self.direction {
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
            Direction::Up => self.position.1 += 1,
            Direction::Down => self.position.1 -= 1,
        }
        return return_status;
    }
}

fn parse_floor(input: &str) -> FxHashMap<Position, Tile> {
    let mut floor = FxHashMap::default();
    for (row_num, line) in input.lines().enumerate() {
        for (col_num, space) in line.chars().enumerate() {
            floor.insert(
                (row_num as isize, col_num as isize),
                Tile {
                    contents: match space {
                        '.' => TileContents::Empty,
                        '/' => TileContents::Mirror(MirrorDirection::FrontSlash),
                        '\\' => TileContents::Mirror(MirrorDirection::BackSlash),
                        '-' => TileContents::Spliiter(SplitterDirection::Horizontal),
                        '|' => TileContents::Spliiter(SplitterDirection::Vertical),
                        _ => panic!("Bad tile content: {space}"),
                    },
                    energized: false,
                },
            );
        }
    }
    floor
}

fn find_bounds(floor: &FxHashMap<Position, Tile>) -> (isize, isize) {
    let (xs, ys): (Vec<isize>, Vec<isize>) = floor.clone().into_keys().unzip();
    (*xs.iter().max().unwrap(), *ys.iter().max().unwrap())
}
