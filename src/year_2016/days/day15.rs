use itertools::Itertools;
use scan_fmt::scan_fmt;
pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2016/day_15.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> usize {
    let mut disks: Vec<Disk> = lines.lines().map(parse_disk).collect();
    disks
        .iter_mut()
        .enumerate()
        .map(|(i, disk)| disk.tick_n(i))
        .collect_vec();
    let mut x = 0;
    loop {
        if disks.iter().all(|disk| disk.current == 0)  {
            return x-1;
        } else {
            x += 1;
            for disk in disks.iter_mut() {
                disk.tick_n(1);
            }
        }
    }
}

fn solve02(lines: &str) -> usize {
    let mut disks: Vec<Disk> = lines.lines().map(parse_disk).collect();
    disks.push(Disk {
        position_count: 11,
        current: 0,
    });
    disks
        .iter_mut()
        .enumerate()
        .map(|(i, disk)| disk.tick_n(i))
        .collect_vec();
    let mut x = 0;
    loop {
        if disks.iter().all(|disk| disk.current == 0)  {
            return x-1;
        } else {
            x += 1;
            for disk in disks.iter_mut() {
                disk.tick_n(1);
            }
        }
    }
}

fn parse_disk(line: &str) -> Disk {
    let (_, position_count, current) = scan_fmt!(
        line,
        "Disc #{d} has {d} positions; at time=0, it is at position {d}.",
        usize,
        usize,
        usize
    )
    .unwrap();
    Disk {
        position_count,
        current,
    }
}

#[derive(Debug, Clone, Copy)]
struct Disk {
    position_count: usize,
    current: usize,
}

impl Disk {
    fn tick_n(&mut self, n: usize) {
        self.current = (self.current + n) % self.position_count;
    }
}
