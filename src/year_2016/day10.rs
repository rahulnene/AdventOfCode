use core::panic;
use std::{
    fmt::{Display, Formatter},
    time::{Duration, Instant},
};

use itertools::Itertools;

const LINES: &str = include_str!("../../problem_inputs_2016/day_10.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let mut bots = Vec::new();
    for _ in 0..210 {
        bots.push(Bot {
            target: [None, None],
            contents: Vec::new(),
        });
    }
    for line in LINES.lines() {
        parse_instr(&mut bots, line);
    }
    let mut bots_p2 = bots.clone();
    (solve01(&mut bots), solve02(&mut bots_p2))
}

fn solve01(bots: &mut [Bot]) -> (usize, Duration) {
    let now = Instant::now();

    loop {
        let giver_and_receivers = find_giver(bots);
        if let Some(ans) = process(bots, giver_and_receivers) {
            return (ans, now.elapsed());
        }
    }
}

fn solve02(bots: &mut [Bot]) -> (usize, Duration) {
    let now = Instant::now();
    let mut outputs = [0, 0, 0];
    loop {
        let giver_and_receivers = find_giver(bots);
        process_p2(bots, giver_and_receivers, &mut outputs);
        if outputs.iter().product::<usize>() != 0 {
            return (outputs.iter().product(), now.elapsed());
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Target {
    Bot(usize),
    Output(usize),
}

impl Target {
    fn get_value(&self) -> usize {
        match self {
            Target::Bot(id) | Target::Output(id) => *id,
        }
    }
}

#[derive(Debug, Clone)]
struct Bot {
    target: [Option<Target>; 2],
    contents: Vec<usize>,
}

impl Bot {
    fn give_target(&mut self, low: Target, high: Target) {
        self.target = [Some(low), Some(high)];
    }
    fn give_ball(&mut self, ball: usize) {
        self.contents.push(ball);
    }
    fn check(&self, id: usize) -> Option<usize> {
        if self.contents.len() == 2 && self.contents.contains(&17) && self.contents.contains(&61) {
            return Some(id);
        }
        None
    }
}

fn parse_instr(bots: &mut [Bot], s: &str) {
    if s.starts_with('v') {
        let (_, val, _, _, _, id) = s.split_ascii_whitespace().collect_tuple().unwrap();
        let val = val.parse::<usize>().unwrap();
        let id = id.parse::<usize>().unwrap();
        bots[id].give_ball(val);
    } else {
        let (
            _,
            id_giver,
            _,
            _,
            _,
            target_or_output_1,
            low_id,
            _,
            _,
            _,
            target_or_output_2,
            high_id,
        ) = s.split_ascii_whitespace().collect_tuple().unwrap();
        let id_giver = id_giver.parse::<usize>().unwrap();
        let low_id = low_id.parse::<usize>().unwrap();
        let high_id = high_id.parse::<usize>().unwrap();
        let low_target = if target_or_output_1 == "bot" {
            Target::Bot(low_id)
        } else {
            Target::Output(low_id)
        };
        let high_target = if target_or_output_2 == "bot" {
            Target::Bot(high_id)
        } else {
            Target::Output(high_id)
        };
        bots[id_giver].give_target(low_target, high_target);
    }
}

fn find_giver(bots: &[Bot]) -> (usize, Target, Target) {
    for (id, bot) in bots.iter().enumerate() {
        if bot.contents.len() == 2 {
            return (id, bot.target[0].unwrap(), bot.target[1].unwrap());
        }
    }
    panic!("No bot has 2 balls");
}

fn process(bots: &mut [Bot], giver_and_receivers: (usize, Target, Target)) -> Option<usize> {
    let (giver_id, low, high) = giver_and_receivers;
    let ans = bots[giver_id].check(giver_id);
    let contents = bots[giver_id].contents.clone();
    if let Target::Bot(id) = low {
        bots[id].give_ball(*contents.iter().min().unwrap());
    }
    if let Target::Bot(id) = high {
        bots[id].give_ball(*contents.iter().max().unwrap());
    }
    bots[giver_id].contents.clear();
    ans
}

fn process_p2(
    bots: &mut [Bot],
    giver_and_receivers: (usize, Target, Target),
    outputs: &mut [usize],
) -> Option<usize> {
    let (giver_id, low, high) = giver_and_receivers;
    let ans = bots[giver_id].check(giver_id);
    let contents = bots[giver_id].contents.clone();
    if let Target::Output(id) = low {
        if id < 3 {
            outputs[id] = *contents.iter().min().unwrap();
        }
    } else {
        bots[low.get_value()].give_ball(*contents.iter().min().unwrap());
    }
    if let Target::Output(id) = high {
        if id < 3 {
            outputs[id] = *contents.iter().max().unwrap();
        }
    } else {
        bots[high.get_value()].give_ball(*contents.iter().max().unwrap());
    }
    bots[giver_id].contents.clear();
    ans
}

fn pprint(bots: &[Bot]) {
    for (id, bot) in bots.iter().enumerate() {
        println!(
            "Bot {} has targets {:?}, {:?} and contents {:?}, {:?}",
            id,
            bot.target[0].map_or_else(String::default, |v| v.to_string()),
            bot.target[1].map_or_else(String::default, |v| v.to_string()),
            bot.contents
                .first()
                .map_or_else(String::default, std::string::ToString::to_string),
            bot.contents
                .get(1)
                .map_or_else(String::default, std::string::ToString::to_string)
        );
    }
    println!("-------");
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Bot(id) => write!(f, "Bot {id}"),
            Target::Output(id) => write!(f, "Output {id}"),
        }
    }
}
