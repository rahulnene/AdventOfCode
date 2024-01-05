use fxhash::FxHashMap;
use itertools::Itertools;

pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2017/day_25_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &'static str) -> usize {
    let input_strs = lines.split("\n\n").collect_vec();
    // for line in input_strs.iter().enumerate() {
    //     println!("INDEX:{} \n{}",line.0, line.1);
    // }
    let init_state = input_strs[0]
        .lines()
        .next()
        .unwrap()
        .chars()
        .nth(15)
        .unwrap();
    let checksum_steps = input_strs[0]
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse::<isize>()
        .unwrap();
    let mut states = FxHashMap::default();
    for state_str in input_strs[1..].iter() {
        dbg!(state_str);
        let state = parse_state(state_str);
        states.insert(
            state_str.lines().next().unwrap().chars().nth(9).unwrap(),
            state,
        );
    }
    let mut machine = TuringMachine::new(init_state, states);
    dbg!(machine.state);
    0
}

fn solve02(lines: &str) -> usize {
    0
}

struct TuringMachine {
    tape: FxHashMap<isize, bool>,
    cursor: isize,
    state: char,
    states: FxHashMap<char, Box<StateProcess>>,
}

impl TuringMachine {
    fn new(init_state: char, states: FxHashMap<char, Box<StateProcess>>) -> Self {
        Self {
            tape: FxHashMap::default(),
            cursor: 0,
            state: init_state,
            states,
        }
    }
}

type StateProcess = dyn FnMut(&mut TuringMachine);

fn parse_state(state_str: &'static str) -> Box<StateProcess> {
    let mut lines = state_str.lines();
    let re = regex::Regex::new(r"In state (.*):\n  If the current value is (.*):\n    - Write the value (.*).\n    - Move one slot to the (.*).\n    - Continue with state (.*).\n  If the current value is (.*):\n    - Write the value (.*).\n    - Move one slot to the (.*).\n    - Continue with state (.*).").unwrap();
    let captures = re.captures(lines.next().unwrap()).unwrap();
    let state = captures.get(1).unwrap().as_str().chars().next().unwrap();
    let mut process = move |machine: &mut TuringMachine| {
        let mut tape_value = machine.tape.get(&machine.cursor).unwrap_or(&false).clone();
        let mut write_value = false;
        let mut move_direction = 0;
        let mut next_state = ' ';
        if tape_value {
            write_value = captures.get(3).unwrap().as_str().parse::<bool>().unwrap();
            move_direction = match captures.get(4).unwrap().as_str() {
                "left" => -1,
                "right" => 1,
                _ => unreachable!(),
            };
            next_state = captures.get(5).unwrap().as_str().chars().next().unwrap();
        } else {
            write_value = captures.get(7).unwrap().as_str().parse::<bool>().unwrap();
            move_direction = match captures.get(8).unwrap().as_str() {
                "left" => -1,
                "right" => 1,
                _ => unreachable!(),
            };
            next_state = captures.get(9).unwrap().as_str().chars().next().unwrap();
        }
        machine.tape.insert(machine.cursor, write_value);
        machine.cursor += move_direction;
        machine.state = next_state;
    };
    Box::new(process)
}
