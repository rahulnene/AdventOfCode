use fxhash::FxHashMap;

pub fn solution() -> (usize, usize) {
    let num_elves = 5;
    (solve01(num_elves), solve02(num_elves))
}

fn solve01(num_elves: usize) -> usize {
    // return 0;
    let mut circle = Circle::new(num_elves);
    circle.get_final_survivor(Circle::get_next_alive_elf);
    println!("PART 1 FINISHED________");
    circle.current_elf_id
}

fn solve02(num_elves: usize) -> usize {
    let mut circle = Circle::new(num_elves);
    circle.get_final_survivor(Circle::get_elf_across_from);
    circle.current_elf_id
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Elf {
    present_count: usize,
}

impl Elf {
    fn new() -> Self {
        Self { present_count: 1 }
    }
}

#[derive(Debug)]
struct Circle {
    elves: FxHashMap<usize, Elf>,
    current_elf_id: usize,
    original_num_elves: usize,
}

impl Circle {
    fn new(num_elves: usize) -> Self {
        let elves = (1..=num_elves).map(|i| (i, Elf::new())).collect();
        Self {
            elves,
            current_elf_id: 1,
            original_num_elves: num_elves,
        }
    }

    fn get_next_alive_elf_after(&self, current: usize) -> usize {
        let mut next_elf_id = current + 1;
        while !self.elves.contains_key(&next_elf_id) {
            next_elf_id += 1;
            if next_elf_id > self.original_num_elves {
                next_elf_id = 1;
            }
        }
        next_elf_id
    }

    fn get_next_alive_elf(&self) -> usize {
        self.get_next_alive_elf_after(self.current_elf_id)
    }

    fn get_elf_across_from(&self) -> usize {
        if self.elves.len() <= 3 {
            // println!("{}", self.get_next_alive_elf());
            return self.get_next_alive_elf();
        }
        let mut current = self.current_elf_id;
        for _ in 0..(self.original_num_elves / 2) {
            current = self.get_next_alive_elf_after(current);
        }
        current
    }

    fn remove_elf(&mut self, elf_id: usize) {
        self.elves.remove(&elf_id);
    }

    fn get_final_survivor(&mut self, next_elf_strategy: fn(&Self) -> usize) {
        while self.elves.len() > 1 {
            let next_elf_id = next_elf_strategy(self);
            let next_elf = self.elves.get_mut(&next_elf_id).unwrap();
            let next_elf_present_count = next_elf.present_count;
            let current_elf = self.elves.get_mut(&self.current_elf_id).unwrap();
            current_elf.present_count += next_elf_present_count;
            self.remove_elf(next_elf_id);
            // println!(
            //     "Elf {} takes {} presents from elf {}",
            //     self.current_elf_id, next_elf_present_count, next_elf_id
            // );
            self.current_elf_id = self.get_next_alive_elf();
            // println!("Next selector is elf {}", self.current_elf_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve01() {
        let num_elves = 5;
        let ans = solve01(num_elves);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_solve02() {
        let num_elves = 5;
        let ans = solve02(num_elves);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_get_next_elf_id() {
        let num_elves = 5;
        let circle = Circle::new(num_elves);
        let ans = circle.get_next_alive_elf();
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_get_directly_across() {
        let num_elves = 5;
        let circle = Circle::new(num_elves);
        let ans = circle.get_elf_across_from();
        assert_eq!(ans, 3);
    }
}
