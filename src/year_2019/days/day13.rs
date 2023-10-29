use rayon::prelude::*;
pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_13_test.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let now = std::time::Instant::now();
    let departure = lines.lines().nth(0).unwrap().parse::<usize>().unwrap();
    let buses = lines
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .filter(|&x| x != "x")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let wait_times = buses
        .iter()
        .map(|&bus| wait_time(departure, bus))
        .collect::<Vec<usize>>();
    let ans = wait_times.iter().min().unwrap()
        * buses[wait_times
            .iter()
            .position(|&x| x == *wait_times.iter().min().unwrap())
            .unwrap()];
    println!("Time: {:?}", now.elapsed());
    ans
}

fn solve02(lines: &str) -> usize {
    let buses = lines
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| if x == "x" { "0" } else { &x })
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut departure: usize = 1068700;
    let first_bus = buses[0];
    departure = departure.next_multiple_of(first_bus);
    (departure..)
        .any(|time| {
            buses
                .par_iter()
                .enumerate()
                .all(|(i, &bus)| bus == 0 || wait_time(time, bus) == i)
        })
        .then_some(departure)
        .unwrap()
}

fn wait_time(departure: usize, bus: usize) -> usize {
    (bus - (departure % bus)) % bus
}
