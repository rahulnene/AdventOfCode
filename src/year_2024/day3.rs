use std::time::{Duration, Instant};

use rustc_hash::{FxHashMap, FxHashSet};

const LINES: &str = include_str!("../../problem_inputs_2023/day_3.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let mut schematic = FxHashMap::default();
    let mut max_x = 0;
    let mut max_y = 0;
    for (r, line) in LINES.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch != '.' {
                schematic.insert((r, c), ch);
            }
            max_x = max_x.max(c);
        }
        max_y = max_y.max(r);
    }
    (
        solve01(&schematic, max_x, max_y),
        solve02(&schematic, max_x, max_y),
    )
}

fn solve01(
    schematic: &FxHashMap<(usize, usize), char>,
    max_x: usize,
    max_y: usize,
) -> (usize, Duration) {
    let now = Instant::now();
    let mut current_number = String::new();
    let mut is_symbol_adj = false;
    let mut ans = 0;

    for r in 0..=max_y {
        for c in 0..=max_x {
            let current_char = schematic.get(&(r, c)).unwrap_or(&'.');
            if !current_char.is_numeric() {
                if is_symbol_adj {
                    if current_number.len() > 0 {
                        ans += current_number.parse::<usize>().unwrap();
                    }
                }
                current_number.clear();
                is_symbol_adj = false;
            } else {
                current_number.push(*current_char);
                is_symbol_adj |= has_symbol_next_to(&schematic, r, c);
            }
        }
    }
    (ans, now.elapsed())
}

fn solve02(
    schematic: &FxHashMap<(usize, usize), char>,
    max_x: usize,
    max_y: usize,
) -> (usize, Duration) {
    let now = Instant::now();
    let mut gear_to_number = FxHashMap::default();
    let mut p2_current_number = String::new();
    let mut p2_is_symbol_adj = false;
    let mut latest_gear = (0, 0);
    for r in 0..=max_y {
        for c in 0..=max_x {
            let current_char = schematic.get(&(r, c)).unwrap_or(&'.');
            if !current_char.is_numeric() {
                if p2_is_symbol_adj && p2_current_number.len() > 0 {
                    gear_to_number
                        .entry(latest_gear)
                        .and_modify(|e: &mut FxHashSet<usize>| {
                            e.insert(p2_current_number.parse::<usize>().unwrap());
                        })
                        .or_insert_with(|| {
                            FxHashSet::from_iter([p2_current_number.parse::<usize>().unwrap()])
                        });
                }
                p2_current_number.clear();
                p2_is_symbol_adj = false;
            } else {
                p2_current_number.push(*current_char);
                let (has_gear, pos) = has_gear_next_to(&schematic, r, c);
                p2_is_symbol_adj |= has_gear;
                if has_gear {
                    latest_gear = pos;
                }
            }
        }
    }
    let mut ans = 0;
    for (_, numbers) in gear_to_number.iter() {
        if numbers.len() == 2 {
            ans += numbers.iter().product::<usize>();
        }
    }
    (ans, now.elapsed())
}

fn has_symbol_next_to(schematic: &FxHashMap<(usize, usize), char>, r: usize, c: usize) -> bool {
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            if let Some(ch) =
                schematic.get(&((r as isize + dr) as usize, (c as isize + dc) as usize))
            {
                if !ch.is_numeric() && *ch != '.' {
                    return true;
                }
            }
        }
    }
    false
}
fn has_gear_next_to(
    schematic: &FxHashMap<(usize, usize), char>,
    r: usize,
    c: usize,
) -> (bool, (isize, isize)) {
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            if let Some(ch) =
                schematic.get(&((r as isize + dr) as usize, (c as isize + dc) as usize))
            {
                if *ch == '*' {
                    return (true, ((r as isize + dr), (c as isize + dc)));
                }
            }
        }
    }
    (false, (-1, -1))
}
