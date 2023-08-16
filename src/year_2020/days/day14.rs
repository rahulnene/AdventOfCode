use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_14.txt");
    match part {
        1 => solve(lines, 10),
        2 => solve(lines, 40),
        _ => 1,
    }
}

fn solve(lines: &str, iters: u8) -> usize {
    let now = std::time::Instant::now();
    let (start, instrs) = lines.split("\n\n").collect_tuple().expect("Invalid input");
    let mut pol = Polymer::from_str(start);
    let last_letter = start.chars().last().unwrap() as u8 - b'A';
    let mut map_instructions = [['\0'; 26]; 26];
    for instr in instrs.lines() {
        let bytes = instr.as_bytes();
        map_instructions[bytes[0] as usize - b'A' as usize][bytes[1] as usize - b'A' as usize] =
            bytes[6] as char;
    }
    for _ in 1..=iters {
        pol.apply(&map_instructions);
    }
    let counts = pol.count(last_letter);
    println!("Time: {:?}", now.elapsed());
    let (max_count, min_count) = counts
        .iter()
        .fold((0, std::usize::MAX), |(max, min), &count| {
            (max.max(count), min.min(count))
        });

    max_count - min_count
}

struct Polymer {
    polymer: [[usize; 26]; 26],
}

impl Polymer {
    fn from_str(s: &str) -> Self {
        let chain = s.to_string();
        let mut polymer = [[0; 26]; 26];
        for (a, b) in chain.chars().tuple_windows() {
            polymer[a as usize - b'A' as usize][b as usize - b'A' as usize] += 1;
        }
        Polymer { polymer }
    }

    fn apply(&mut self, instr: &[[char; 26]; 26]) {
        let mut additions = [[0; 26]; 26];
        for (a, row) in self.polymer.iter().enumerate() {
            for (b, count) in row.iter().enumerate() {
                let c = instr[a][b];
                if c != '\0' {
                    additions[a][c as usize - b'A' as usize] += *count;
                    additions[c as usize - b'A' as usize][b] += *count;
                }
            }
        }
        self.polymer = additions;
    }

    fn count(&self, last_letter: u8) -> Vec<usize> {
        let mut count = [0; 26];
        count[last_letter as usize] += 1;
        for (a, row) in self.polymer.iter().enumerate() {
            for (_, c) in row.iter().enumerate() {
                count[a] += *c;
            }
        }
        count.iter().filter(|f| **f > 0).copied().collect_vec()
    }
}
