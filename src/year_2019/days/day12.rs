use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_12.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut ship = Ship::new();
    for instr in lines.lines() {
        dbg!(instr);
        ship.action(instr);

        dbg!(ship);
    }
    ship.manhattan_distance()
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ship {
    x: i32,
    y: i32,
    direction: i32,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: 90,
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn action(&mut self, instr: &str) {
        let (action, value) = instr.split_at(1);
        let value = value.parse::<u32>().unwrap();
        match action {
            "N" => self.go_north(value),
            "S" => self.go_south(value),
            "E" => self.go_east(value),
            "W" => self.go_west(value),
            "L" => self.turn_left(value),
            "R" => self.turn_right(value),
            "F" => self.go_forward(value),
            _ => {
                dbg!(action);
                panic!("Invalid action")
            }
        }
        self.direction = self.direction.rem_euclid(360);
    }

    fn turn_left(&mut self, value: u32) {
        self.direction -= value as i32;
    }
    fn turn_right(&mut self, value: u32) {
        self.direction += value as i32;
    }

    fn go_north(&mut self, value: u32) {
        self.y += value as i32;
    }
    fn go_south(&mut self, value: u32) {
        self.y -= value as i32;
    }
    fn go_east(&mut self, value: u32) {
        self.x += value as i32;
    }
    fn go_west(&mut self, value: u32) {
        self.x -= value as i32;
    }
    fn go_forward(&mut self, value: u32) {
        match self.direction {
            0 => self.go_north(value),
            90 => self.go_east(value),
            180 => self.go_south(value),
            270 => self.go_west(value),
            _ => {
                dbg!(self);
                panic!("Invalid action")
            }
        }
    }
}
