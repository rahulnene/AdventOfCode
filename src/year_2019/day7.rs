use std::time::{Duration, Instant};

use itertools::Itertools;

use super::intcode::Computer;

const LINES: &str = include_str!("../../problem_inputs_2019/day_7_test.txt");

pub fn solution() -> ((isize, Duration), (isize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (isize, Duration) {
    return (0, Duration::default());
    let now = Instant::now();
    let phases = [0, 1, 2, 3, 4].iter().permutations(5);
    let mut max_signal = 0;
    for combo in phases {
        let mut input = 0;
        for phase in combo {
            let mut comp = Computer::new(LINES, &[*phase, input]);
            input = comp.run_to_halt();
        }
        if input > max_signal {
            max_signal = input;
        }
    }
    (max_signal, now.elapsed())
}

fn solve02() -> (isize, Duration) {
    let now = Instant::now();
    let phases = [5, 6, 7, 8, 9].iter().permutations(5);
    let mut max_signal = 0;
    for combo in phases {
        println!("{:?}", combo);
        let mut input = 0;
        loop {
            let mut finished = false;
            for phase in &combo {
                let mut comp = Computer::new(LINES, &[**phase, input]);
                input = comp.run_to_output();
                finished |= comp.is_halted();
            }
            if finished {
                if input > max_signal {
                    max_signal = input;
                    break;
                }
            }
        }
    }
    (max_signal, now.elapsed())
}
