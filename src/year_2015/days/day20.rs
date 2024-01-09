use std::collections::BTreeSet;
const MAX: u32 = 4294967295;
pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2016/day_20.txt");
    let mut ranges = RangeSet::new();
    for line in lines.lines() {
        let mut split = line.split('-');
        let start = split.next().unwrap().parse::<u32>().unwrap();
        let end = split.next().unwrap().parse::<u32>().unwrap();
        ranges.add_range(start, end);
    }
    dbg!(&ranges.set);
    dbg!((0..=MAX).filter(|f| ranges.passes(*f)).next().unwrap() as usize);
    dbg!((0..=MAX).filter(|f| ranges.passes(*f)).count() as usize);
    (0, 0)
}

// fn solve01(blacklist: &RangeSet<[RangeInclusive<u32>; 2]>) -> usize {
//     // (0..9).filter(|f| blacklist.contains(*f)).next().unwrap() as usize;
//     0
// }

// fn solve02(blacklist: &RangeSet<[RangeInclusive<u32>; 2]>) -> usize {
//     0
// }

pub struct RangeSet {
    set: BTreeSet<(u32, u32)>,
}

impl RangeSet {
    pub fn new() -> Self {
        Self {
            set: BTreeSet::new(),
        }
    }

    pub fn add_range(&mut self, start: u32, end: u32) {
        let mut new_range = (start, end);

        // Collect overlapping ranges
        let overlapping: Vec<_> = self
            .set
            .range((u32::MIN, start)..=(end, u32::MAX))
            .cloned()
            .collect();

        // Merge overlapping ranges
        for (s, e) in overlapping {
            new_range.0 = new_range.0.min(s);
            new_range.1 = new_range.1.max(e);
            self.set.remove(&(s, e));
        }

        self.set.insert(new_range);
    }

    pub fn passes(&self, value: u32) -> bool {
        !self
            .set
            .range(..=(value, u32::MAX))
            .next_back()
            .map_or(false, |&(start, end)| start <= value && value <= end)
    }
}
