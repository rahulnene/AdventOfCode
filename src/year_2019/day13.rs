use super::intcode::Computer;
use itertools::Itertools;
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
const LINES: &str = include_str!("../../problem_inputs_2019/day_13.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let (_, input_receiver) = mpsc::channel::<isize>();
    let (mut comp, output_receiver) = Computer::new(LINES, input_receiver);
    thread::spawn(move || comp.run_to_halt());
    let ans = output_receiver
        .iter()
        .collect_vec()
        .chunks_exact(3)
        .filter(|chunk| chunk[2] == 2)
        .count();
    // let mut comp = Computer::new(LINES, &[]);
    // let output = Vec::new();
    // let ans = output.chunks_exact(3).filter(|chunk| chunk[2] == 2).count();
    // dbg!(ans);
    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let (input_sender, input_receiver) = mpsc::channel::<isize>();
    let (mut comp, output_receiver) = Computer::new(LINES, input_receiver);
    comp.set_memory(0, 2);
    let comp_handle = thread::spawn(move || comp.start());
    let mut output_triple = Vec::new();
    let mut ball_x = 0;
    let mut paddle_x = 0;
    while !comp_handle.is_finished() {
        let received = output_receiver.recv().unwrap();
        if received == isize::MAX {
            break;
        }
        output_triple.push(received);
        if output_triple.len() == 3 {
            if output_triple[0] == -1 && output_triple[1] == 0 {
                println!("Score: {}", output_triple[2]);
            } else {
                match output_triple[2] {
                    3 => paddle_x = output_triple[0],
                    4 => ball_x = output_triple[0],
                    _ => (),
                }
            }

            output_triple.clear();
        }
        input_sender
            .send(if paddle_x < ball_x {
                1
            } else if paddle_x > ball_x {
                -1
            } else {
                0
            })
            .unwrap();
    }
    comp_handle.join().unwrap();
    (0, now.elapsed())
}
