use md5::compute;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
pub fn solution() -> ((String, Duration), (usize, Duration)) {
    let now = Instant::now();
    let start_hash = "ioramepc";
    let ans = get_shortest_and_longest_paths(&now, start_hash);
    ans
}

type Position = (isize, isize);

//[up, down, left, right]
type OpenDoors = [bool; 4];

fn get_open_doors(pos: &Position, hash_str: &str) -> OpenDoors {
    let hash = compute(hash_str.as_bytes());
    let hash = format!("{:x}", hash);
    let hash = hash.chars().take(4).collect::<String>();

    let mut doors = [false; 4];
    for (i, c) in hash.chars().enumerate() {
        doors[i] = c > 'a' && c <= 'f';
    }
    if pos.0 == 0 {
        doors[2] = false;
    }
    if pos.1 == 0 {
        doors[0] = false;
    }
    if pos.0 == 3 {
        doors[3] = false;
    }
    if pos.1 == 3 {
        doors[1] = false;
    }
    doors
}

fn get_shortest_and_longest_paths(
    start_time: &Instant,
    start_hash: &str,
) -> ((String, Duration), (usize, Duration)) {
    let mut longest_path = 0;
    let mut stack = VecDeque::new();
    let mut shortest_path = String::default();
    let mut time_p1 = Duration::default();
    stack.push_back((0, 0, start_hash.to_string()));
    while let Some((x, y, hash)) = stack.pop_front() {
        if x == 3 && y == 3 {
            if shortest_path.len() == 0 {
                shortest_path = hash[start_hash.len()..].to_string();
                time_p1 = start_time.elapsed();
            }
            longest_path = longest_path.max(hash[start_hash.len()..].len());
        } else {
            let doors = get_open_doors(&(x, y), &hash);
            if doors[0] {
                stack.push_back((x, y - 1, format!("{}{}", hash, "U")));
            }
            if doors[1] {
                stack.push_back((x, y + 1, format!("{}{}", hash, "D")));
            }
            if doors[2] {
                stack.push_back((x - 1, y, format!("{}{}", hash, "L")));
            }
            if doors[3] {
                stack.push_back((x + 1, y, format!("{}{}", hash, "R")));
            }
        }
    }
    (
        (shortest_path, time_p1),
        (longest_path, start_time.elapsed()),
    )
}
