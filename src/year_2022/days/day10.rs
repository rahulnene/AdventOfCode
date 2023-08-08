use crate::util::read_lines;
pub fn solution() -> u32 {
    let lines = read_lines("./problem_inputs/day10.txt").unwrap();
    part1(lines)
}

fn part1(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> u32 {
    let mut cpu = CPU {
        clock_cycle: 0,
        register: 1,
        signal_strength: 0,
    };
    let mut screen = CRT {
        screen: vec![vec![false; 40]; 6],
    };
    lines.map(Result::unwrap).for_each(|mut command| {
        match command.as_str() {
            "noop" => cpu.noop(&mut screen),
            _ => cpu.addx(command.split_off(5).parse::<i64>().unwrap(), &mut screen),
        };
    });
    for row in &screen.screen {
        for col in row.iter() {
            print!("{}", if *col { '#' } else { '.' });
        }
        println!();
    }
    cpu.signal_strength as u32
}

#[derive(Debug, Clone)]
struct CRT {
    screen: Vec<Vec<bool>>,
}

#[derive(Debug, Clone, Copy)]
struct CPU {
    clock_cycle: i64,
    register: i64,
    signal_strength: i64,
}

impl CPU {
    fn clock_check(&mut self) {
        match self.clock_cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                self.signal_strength += self.clock_cycle * self.register;
            }
            _ => {}
        }
    }

    fn noop(&mut self, screen: &mut CRT) {
        self.clock_cycle += 1;
        self.clock_check();
        self.draw(screen);
    }

    fn addx(&mut self, x: i64, screen: &mut CRT) {
        self.draw(screen);
        self.clock_cycle += 1;
        self.clock_check();
        self.draw(screen);
        self.clock_cycle += 1;
        self.clock_check();

        self.register += x;
    }

    fn draw(&self, screen: &mut CRT) {
        if (self.clock_cycle % 40) >= self.register - 1
            && (self.clock_cycle % 40) <= self.register + 1
        {
            let row = (self.clock_cycle / 40) as usize;
            let col = (self.clock_cycle % 40) as usize;
            screen.screen[row][col] = true;
        }
    }
}
