use fxhash::FxHashMap;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2019/day_2.txt");
    match part {
        1 => solve01(12, 2, lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(noun: usize, verb: usize, lines: &str) -> usize {
    let mut memory: FxHashMap<usize, usize> = FxHashMap::default();
    lines
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .enumerate()
        .for_each(|(a, b)| {
            memory.insert(a, b.parse::<usize>().unwrap());
        });
    memory.insert(1, noun);
    memory.insert(2, verb);
    let mut instruct_pointer = 0;
    loop {
        let current_instruction = memory.get(&instruct_pointer).unwrap();
        match current_instruction {
            1 => {
                let a = memory.get(&(instruct_pointer + 1)).unwrap();
                let b = memory.get(&(instruct_pointer + 2)).unwrap();
                let c = memory.get(&(instruct_pointer + 3)).unwrap();
                memory.insert(*c, memory.get(a).unwrap() + memory.get(b).unwrap());
            }
            2 => {
                let a = memory.get(&(instruct_pointer + 1)).unwrap();
                let b = memory.get(&(instruct_pointer + 2)).unwrap();
                let c = memory.get(&(instruct_pointer + 3)).unwrap();
                memory.insert(*c, memory.get(a).unwrap() * memory.get(b).unwrap());
            }
            99 => {
                break;
            }
            _ => panic!("Invalid instruction"),
        }
        instruct_pointer += 4;
    }
    *memory.get(&0).unwrap()
}

fn solve02(lines: &str) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            if solve01(noun, verb, lines) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}
