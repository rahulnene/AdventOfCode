use itertools::Itertools;

pub fn solution(part: u8) -> isize {
    let lines = include_str!("../../../problem_inputs_2021/day_2.txt");
    solve(lines, part)
}

pub fn solve(lines: &str, part: u8) -> isize {
    let mut submarine = Submarine::new();
    lines
        .lines()
        .for_each(|line| submarine.command(line, part == 1));
    submarine.depth as isize * submarine.forward as isize
}

#[derive(Debug, Clone, Copy)]
struct Submarine {
    depth: usize,
    forward: usize,
    aim: isize,
}

impl Submarine {
    fn new() -> Self {
        Self {
            depth: 0,
            forward: 0,
            aim: 0,
        }
    }

    fn command(&mut self, command: &str, part_1: bool) {
        let (direction, value) = command.split_ascii_whitespace().collect_tuple().unwrap();
        match direction {
            "forward" => {
                let fwd = value.parse::<usize>().unwrap();
                self.forward += fwd;
                self.depth += self.aim as usize * fwd;
            }
            "down" => {
                self.depth += value.parse::<usize>().unwrap() * usize::from(!part_1);
                self.aim += value.parse::<isize>().unwrap() * isize::from(!part_1);
            }
            "up" => {
                self.depth -= value.parse::<usize>().unwrap() * usize::from(!part_1);
                self.aim -= value.parse::<isize>().unwrap() * isize::from(!part_1);
            }
            _ => panic!("Invalid command"),
        }
    }
}
