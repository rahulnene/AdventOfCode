use std::{
    default,
    sync::mpsc::{self, Receiver, Sender},
    time::{Duration, Instant},
};

use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2023/day_20.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}
enum Signal {
    Low,
    High,
}
#[derive(Debug)]
struct Module {
    inputs: Vec<Receiver<Signal>>,
    outputs: Vec<Sender<Signal>>,
}

impl Module {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
    fn add_input_source(&mut self, input: Receiver<Signal>) {
        self.inputs.push(input);
    }
    fn add_output_sink(&mut self, output: Sender<Signal>) {
        self.outputs.push(output);
    }
}

struct ModuleManager {
    modules: FxHashMap<String, Module>,
    targets: FxHashMap<String, Vec<Sender<Signal>>>,
    receivers: FxHashMap<String, Vec<Receiver<Signal>>>,
}

impl ModuleManager {
    fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
            targets: FxHashMap::default(),
            receivers: FxHashMap::default(),
        }
    }
    fn add_module(&mut self, name: &str, module: Module) {
        self.modules.insert(name.to_owned(), module);
        let (tx, rx) = mpsc::channel::<Signal>();
        self.targets
            .entry(name.to_owned())
            .and_modify(|v| v.push(tx.clone()))
            .or_insert(vec![tx]);
        self.receivers
            .entry(name.to_owned())
            .and_modify(|v| v.push(rx))
            .or_insert(vec![rx.clone()]);
    }

    fn connect(&mut self, from: String, to: String) {
        let from_module = self.modules.get_mut(&from).unwrap();
        let to_module = self.modules.get_mut(&to).unwrap();
        let from_channels = self.targets.get(&from).unwrap();
    }
}
