use fxhash::FxHashMap;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_14.txt");
    match part {
        1 => solve(lines),
        2 => solve(lines),
        _ => 1,
    }
}

fn solve(lines: &str) -> usize {
    let mut memory = Memory::new();
    for line in lines.lines() {
        memory.process(line);
        println!("{:?}", memory.mem);
    }
    memory.mem.values().sum()
}

#[derive(Debug, Clone)]
struct Memory {
    mem: FxHashMap<usize, usize>,
    mask: Vec<BitMask>,
}

impl Memory {
    fn new() -> Self {
        Self {
            mem: FxHashMap::default(),
            mask: vec![BitMask::X; 36],
        }
    }

    fn process(&mut self, instr: &str) {
        if instr.starts_with("mask") {
            self.set_mask(instr.split(" = ").nth(1).unwrap());
        } else {
            let addr = instr
                .split("[")
                .nth(1)
                .unwrap()
                .split("]")
                .nth(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let val = instr.split(" = ").nth(1).unwrap().parse::<usize>().unwrap();
            self.set_mem(addr, val);
        }
    }

    fn set_mask(&mut self, mask: &str) {
        self.mask = mask
            .chars()
            .map(|c| match c {
                '0' => BitMask::Zero,
                '1' => BitMask::One,
                'X' => BitMask::X,
                _ => panic!("Invalid mask"),
            })
            .collect::<Vec<BitMask>>();
    }

    fn set_mem(&mut self, addr: usize, val: usize) {
        let val = format!("{:036b}", val);
        let val = val
            .chars()
            .zip(self.mask.iter())
            .map(|(c, m)| match m {
                BitMask::Zero => '0',
                BitMask::One => '1',
                BitMask::X => c,
            })
            .collect::<String>();
        let val = usize::from_str_radix(&val, 2).unwrap();
        self.mem.insert(addr, val);
    }
}

#[derive(Debug, Clone)]
enum BitMask {
    Zero,
    One,
    X,
}
