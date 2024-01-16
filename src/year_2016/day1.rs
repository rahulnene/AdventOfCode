use fxhash::FxHashSet;

pub fn solution() -> (usize, usize) {
    let line = include_str!("../../../problem_inputs_2016/day_1.txt");
    (solve01(line), solve02(line))
}

fn solve01(line: &str) -> usize {
    let mut human = Human::new();
    line.split(", ")
        .map(Instruction::parse)
        .for_each(|instr| human.follow(instr));

    (human.pos.0.abs() + human.pos.1.abs()) as usize
}

fn solve02(line: &str) -> usize {
    let mut human = Human::new();
    for instr in line.split(", ").map(Instruction::parse) {
        dbg!(&human.pos);
        let new_pos = human.visited_pos.insert(human.pos);
        if !new_pos {
            break;
        } else {
            human.follow(instr);
        }
    }
    (human.pos.0.abs() + human.pos.1.abs()) as usize
}

#[derive(Debug, Clone, Copy)]

enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    turn: Turn,
    steps: usize,
}
impl Instruction {
    fn parse(s: &str) -> Self {
        let turn = match s.chars().nth(0).unwrap() {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => panic!("Invalid turn"),
        };
        let steps = s[1..].parse::<usize>().unwrap();
        Self { turn, steps }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]

struct Human {
    pos: (isize, isize),
    dir: Direction,
    visited_pos: FxHashSet<(isize, isize)>,
}

impl Human {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            dir: Direction::North,
            visited_pos: FxHashSet::default(),
        }
    }

    fn follow(&mut self, instr: Instruction) {
        match instr.turn {
            Turn::Left => match self.dir {
                Direction::North => self.dir = Direction::West,
                Direction::East => self.dir = Direction::North,
                Direction::South => self.dir = Direction::East,
                Direction::West => self.dir = Direction::South,
            },
            Turn::Right => match self.dir {
                Direction::North => self.dir = Direction::East,
                Direction::East => self.dir = Direction::South,
                Direction::South => self.dir = Direction::West,
                Direction::West => self.dir = Direction::North,
            },
        }
        match self.dir {
            Direction::North => self.pos.1 += instr.steps as isize,
            Direction::East => self.pos.0 += instr.steps as isize,
            Direction::South => self.pos.1 -= instr.steps as isize,
            Direction::West => self.pos.0 -= instr.steps as isize,
        }
    }
}
