pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2016/day_2.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> usize {
    let mut keypad = KeyPad::new();
    let mut ans = String::new();
    for line in lines.lines() {
        for dir in line.chars().map(|c| parse_direction(c)) {
            keypad.process_direction(dir);
        }
        ans.push(keypad.read_current_position())
    }
    ans.parse().unwrap()
}

fn solve02(lines: &str) -> usize {
    let mut keypad = KeyPad2::new();
    let mut ans = String::new();
    for line in lines.lines() {
        for dir in line.chars().map(|c| parse_direction(c)) {
            keypad.process_direction(dir);
        }
        ans.push(keypad.read_current_position())
    }
    println!("{}", ans);
    0
}

#[derive(Debug, Clone, Copy)]
struct KeyPad {
    current_position: (isize, isize),
}

impl KeyPad {
    fn new() -> Self {
        Self {
            current_position: (0, 0),
        }
    }

    fn read_current_position(&self) -> char {
        match self.current_position {
            (-1, 1) => '1',
            (0, 1) => '2',
            (1, 1) => '3',
            (-1, 0) => '4',
            (0, 0) => '5',
            (1, 0) => '6',
            (-1, -1) => '7',
            (0, -1) => '8',
            (1, -1) => '9',
            _ => panic!("Invalid position"),
        }
    }

    fn process_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.current_position.1 < 1 {
                    self.current_position.1 += 1;
                }
            }
            Direction::Down => {
                if self.current_position.1 > -1 {
                    self.current_position.1 -= 1;
                }
            }
            Direction::Left => {
                if self.current_position.0 > -1 {
                    self.current_position.0 -= 1;
                }
            }
            Direction::Right => {
                if self.current_position.0 < 1 {
                    self.current_position.0 += 1;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_direction(c: char) -> Direction {
    match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction"),
    }
}

#[derive(Debug, Clone, Copy)]
struct KeyPad2 {
    current_position: (isize, isize),
}

impl KeyPad2 {
    fn new() -> Self {
        Self {
            current_position: (-2, 0),
        }
    }

    fn radius(&self) -> usize {
        (self.current_position.0.abs() + self.current_position.1.abs()) as usize
    }

    fn read_current_position(&self) -> char {
        match self.current_position {
            (0, 2) => '1',
            (-1, 1) => '2',
            (0, 1) => '3',
            (1, 1) => '4',
            (-2, 0) => '5',
            (-1, 0) => '6',
            (0, 0) => '7',
            (1, 0) => '8',
            (2, 0) => '9',
            (-1, -1) => 'A',
            (0, -1) => 'B',
            (1, -1) => 'C',
            (0, -2) => 'D',
            _ => panic!("Invalid position"),
        }
    }

    fn process_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.current_position.1 += 1;
                if self.radius() > 2 {
                    self.current_position.1 -= 1;
                }
            }
            Direction::Down => {
                self.current_position.1 -= 1;
                if self.radius() > 2 {
                    self.current_position.1 += 1;
                }
            }
            Direction::Left => {
                self.current_position.0 -= 1;
                if self.radius() > 2 {
                    self.current_position.0 += 1;
                }
            }
            Direction::Right => {
                self.current_position.0 += 1;
                if self.radius() > 2 {
                    self.current_position.0 -= 1;
                }
            }
        }
    }
}
