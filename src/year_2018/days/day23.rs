use scan_fmt::scan_fmt;
pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_23_test.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut bots = lines.lines().map(NanoBot::parse).collect::<Vec<NanoBot>>();
    let strongest = bots
        .iter()
        .max_by_key(|bot| bot.radius)
        .expect("No bots found");
    let strongest_bot = *strongest; 
    bots.retain(|bot| strongest_bot.in_range_of(bot));
    bots.len()
}
fn solve02(lines: &str) -> usize {
    let mut bots = lines.lines().map(NanoBot::parse).collect::<Vec<NanoBot>>();
    let strongest = bots
        .iter()
        .max_by_key(|bot| bot.radius)
        .expect("No bots found");
    let strongest_bot = *strongest; // Use a separate variable instead of cloning
    bots.retain(|bot| strongest_bot.in_range_of(bot));
    bots.len();
    0
}

type Coordinate = (isize, isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NanoBot {
    position: Coordinate,
    radius: isize,
}

impl NanoBot {
    fn new(position: Coordinate, radius: isize) -> Self {
        NanoBot { position, radius }
    }

    fn parse(line: &str) -> Self {
        let (x, y, z, radius) =
            scan_fmt!(line, "pos=<{},{},{}>, r={}", isize, isize, isize, isize).unwrap();
        NanoBot::new((x, y, z), radius)
    }

    fn distance_to(&self, other: &NanoBot) -> isize {
        let (x1, y1, z1) = self.position;
        let (x2, y2, z2) = other.position;
        (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
    }

    fn in_range_of(&self, other: &NanoBot) -> bool {
        self.distance_to(other) <= self.radius
    }
}
