use std::collections::VecDeque;

pub fn solution() -> (usize, usize) {
    let step_size: u32 = 355;
    (solve01(step_size, 2017), solve02(step_size, 50_000_000))
}

fn solve01(step_size: u32, max_value: u32) -> usize {
    let mut spinlock = Spinlock::new();
    while spinlock.max_num != max_value {
        spinlock.step(step_size);
    }
    spinlock.get_last() as usize
}

fn solve02(step_size: u32, max_value: u32) -> usize {
    let mut spinlock = Spinlock::new();
    while spinlock.max_num != max_value {
        spinlock.step(step_size);
    }
    dbg!(spinlock.buffer[0]);
    spinlock.get_after_0() as usize
}

#[derive(Debug, Clone)]
struct Spinlock {
    buffer: VecDeque<u32>,
    max_num: u32,
}

impl Spinlock {
    fn new() -> Self {
        let mut buffer = VecDeque::with_capacity(2018);
        buffer.push_back(0);
        Self { buffer, max_num: 0 }
    }

    fn step(&mut self, step_size: u32) {
        self.max_num += 1;
        let step_size = step_size % self.buffer.len() as u32;
        self.buffer.rotate_right(step_size as usize);
        self.buffer.push_front(self.max_num);
    }

    fn print(&self) {
        println!("{:?}", self.buffer);
    }

    fn get_last(&self) -> u32 {
        *self.buffer.back().unwrap()
    }

    fn get_after_0(&self) -> u32 {
        self.buffer[self.buffer.iter().position(|&x| x == 0).unwrap() + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve01() {
        assert_eq!(solve01(3, 2017), 638);
    }
}
