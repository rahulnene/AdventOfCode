use fxhash::FxHashMap;
use fxhash::FxHashMap;
use itertools::Itertools;

pub fn solution(part: u16) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_4.txt");
    let logs = lines
        .lines()
        .map(parse_log)
        .sorted_by_key(|l| l.date)
        .collect_vec();
    let mut guard_sleep_timings: FxHashMap<Id, Vec<u16>> = FxHashMap::default();
    let mut guard_sleep_time: FxHashMap<Id, usize> = FxHashMap::default();
    let (mut current_guard_id, mut guard_slept_at) = (0, 0);

    for log in logs {
        match log.event_type {
            LogType::ShiftStart(id) => current_guard_id = id,
            LogType::Sleep => {
                guard_slept_at = log.date.minute;
            }
            LogType::Wakes => {
                let time_slept = (log.date.minute - guard_slept_at) as usize;
                guard_sleep_time
                    .entry(current_guard_id)
                    .and_modify(|e| *e += time_slept)
                    .or_insert(time_slept);

                let mut sleep_time_range = (guard_slept_at..log.date.minute).collect_vec();
                guard_sleep_timings
                    .entry(current_guard_id)
                    .and_modify(|e| e.append(&mut sleep_time_range))
                    .or_insert(sleep_time_range);
            }
        }
    }
pub fn solution(part: u16) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_4.txt");
    let logs = lines
        .lines()
        .map(parse_log)
        .sorted_by_key(|l| l.date)
        .collect_vec();
    let mut guard_sleep_timings: FxHashMap<Id, Vec<u16>> = FxHashMap::default();
    let mut guard_sleep_time: FxHashMap<Id, usize> = FxHashMap::default();
    let (mut current_guard_id, mut guard_slept_at) = (0, 0);

    for log in logs {
        match log.event_type {
            LogType::ShiftStart(id) => current_guard_id = id,
            LogType::Sleep => {
                guard_slept_at = log.date.minute;
            }
            LogType::Wakes => {
                let time_slept = (log.date.minute - guard_slept_at) as usize;
                guard_sleep_time
                    .entry(current_guard_id)
                    .and_modify(|e| *e += time_slept)
                    .or_insert(time_slept);

                let mut sleep_time_range = (guard_slept_at..log.date.minute).collect_vec();
                guard_sleep_timings
                    .entry(current_guard_id)
                    .and_modify(|e| e.append(&mut sleep_time_range))
                    .or_insert(sleep_time_range);
            }
        }
    }
    match part {
        1 => solve01(&guard_sleep_time, &guard_sleep_timings),
        2 => solve02(&guard_sleep_timings),
        1 => solve01(&guard_sleep_time, &guard_sleep_timings),
        2 => solve02(&guard_sleep_timings),
        _ => 1,
    }
}

fn solve01(
    guard_sleep_time: &FxHashMap<u16, usize>,
    guard_sleep_timings: &FxHashMap<u16, Vec<u16>>,
) -> usize {
    guard_sleep_time
        .iter()
        .max_by_key(|f| f.1)
        .and_then(|(sleepy_guard_id, _)| {
            guard_sleep_timings
                .get(sleepy_guard_id)
                .and_then(|timings| {
                    timings
                        .iter()
                        .counts()
                        .iter()
                        .sorted_unstable_by_key(|f| f.1)
                        .last()
                        .map(|(time, _)| *sleepy_guard_id * **time)
                })
        })
        .unwrap() as usize
}

fn solve02(guard_sleep_timings: &FxHashMap<Id, Vec<u16>>) -> usize {
    let result: Vec<(u16, (u16, u16))> = guard_sleep_timings
        .iter()
        .map(|(key, values)| {
            let freq_map = values.iter().copied().counts();
            let (max_num, max_freq) = freq_map.iter().max_by_key(|(_, freq)| *freq).unwrap();
            (*key, (*max_num, *max_freq as u16))
        })
        .collect();

    result
        .iter()
        .max_by_key(|(_, (_, count))| *count)
        .map(|(guard_id, (time, _))| guard_id * time)
        .unwrap() as usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Timestamp {
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
}

type Id = u16;

#[derive(Debug, Clone, Copy)]
enum LogType {
    ShiftStart(Id),
    Sleep,
    Wakes,
}

#[derive(Debug, Clone, Copy)]
struct LogEvent {
    date: Timestamp,
    event_type: LogType,
fn solve02(guard_sleep_timings: &FxHashMap<Id, Vec<u16>>) -> usize {
    let result: Vec<(u16, (u16, u16))> = guard_sleep_timings
        .iter()
        .map(|(key, values)| {
            let freq_map = values.iter().copied().counts();
            let (max_num, max_freq) = freq_map.iter().max_by_key(|(_, freq)| *freq).unwrap();
            (*key, (*max_num, *max_freq as u16))
        })
        .collect();

    result
        .iter()
        .max_by_key(|(_, (_, count))| *count)
        .map(|(guard_id, (time, _))| guard_id * time)
        .unwrap() as usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Timestamp {
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
}

type Id = u16;

#[derive(Debug, Clone, Copy)]
enum LogType {
    ShiftStart(Id),
    Sleep,
    Wakes,
}

#[derive(Debug, Clone, Copy)]
struct LogEvent {
    date: Timestamp,
    event_type: LogType,
}

fn parse_log(s: &str) -> LogEvent {
    {
        {
            let parts: Vec<&str> = s.split_whitespace().collect();
            let date_parts: Vec<u16> = parts[0][1..]
                .split('-')
                .map(|x| x.parse().unwrap())
                .collect();
            let time_parts: Vec<u16> = parts[1][..5]
                .split(':')
                .map(|x| x.parse().unwrap())
                .collect();
            let timestamp = Timestamp {
                month: date_parts[1],
                day: date_parts[2],
                hour: time_parts[0],
                minute: time_parts[1],
            };
            let event = match parts[2] {
                "Guard" => LogType::ShiftStart(parts[3][1..].parse().unwrap()),
                "falls" => LogType::Sleep,
                "wakes" => LogType::Wakes,
                _ => panic!("Unknown event type"),
            };
            LogEvent {
                date: timestamp,
                event_type: event,
            }
        }
    }
fn parse_log(s: &str) -> LogEvent {
    {
        {
            let parts: Vec<&str> = s.split_whitespace().collect();
            let date_parts: Vec<u16> = parts[0][1..]
                .split('-')
                .map(|x| x.parse().unwrap())
                .collect();
            let time_parts: Vec<u16> = parts[1][..5]
                .split(':')
                .map(|x| x.parse().unwrap())
                .collect();
            let timestamp = Timestamp {
                month: date_parts[1],
                day: date_parts[2],
                hour: time_parts[0],
                minute: time_parts[1],
            };
            let event = match parts[2] {
                "Guard" => LogType::ShiftStart(parts[3][1..].parse().unwrap()),
                "falls" => LogType::Sleep,
                "wakes" => LogType::Wakes,
                _ => panic!("Unknown event type"),
            };
            LogEvent {
                date: timestamp,
                event_type: event,
            }
        }
    }
}
