use std::time::{Duration, Instant};

use itertools::Itertools;

const LINES: &str = include_str!("../../problem_inputs_2023/day_5.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let (seeds, maps) = parse_input();
    (solve01(&seeds, &maps), solve02(&seeds, &maps))
}

fn solve01(seeds: &[usize], maps: &[Map]) -> (usize, Duration) {
    let now = Instant::now();
    let ans = seeds.iter().map(|s| get_location_for_seed(*s, maps)).min();
    (ans.unwrap(), now.elapsed())
}

fn solve02(seeds: &[usize], maps: &[Map]) -> (usize, Duration) {
    let now = Instant::now();
    let loc_map = maps.iter().find(|m| m.source == "humidity").unwrap();
    let min_loc = loc_map
        .destination_ranges
        .iter()
        .min_by_key(|r| r.start)
        .unwrap()
        .start;
    let max_loc = loc_map
        .destination_ranges
        .iter()
        .max_by_key(|r| r.start + r.len)
        .unwrap()
        .start;
    let mut min_seed = seeds.chunks(2).min_by_key(|c| c[0]).unwrap()[0];
    let max_seed = seeds.chunks(2).max_by_key(|c| c[0] + c[1]).unwrap();
    let mut max_seed = max_seed[0] + max_seed[1];
    let mut range = max_seed - min_seed;
    while range > 100 {
        let mut min = usize::MAX;
        let sqrt_range = f32::sqrt(range as f32) as usize;
        let mut min_seed_found_at = 0;
        for seed in (min_seed..=max_seed).step_by(f32::sqrt(range as f32) as usize) {
            let loc_at_seed = get_location_for_seed(seed, maps);
            if loc_at_seed > max_loc || loc_at_seed < min_loc {
                continue;
            }
            if loc_at_seed < min {
                min = loc_at_seed;
                min_seed_found_at = seed;
            }
        }
        min_seed = min_seed_found_at - sqrt_range / 2;
        max_seed = min_seed + sqrt_range / 2;
        range = max_seed - min_seed;
    }
    let mut min = usize::MAX;
    for seed in min_seed..=max_seed {
        let loc = get_location_for_seed(seed, maps);
        min = min.min(loc)
    }
    (min, now.elapsed())
}
#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    len: usize,
}

impl Range {
    fn contains(&self, value: usize) -> bool {
        value >= self.start && value < self.start + self.len
    }
    fn calc_offset(&self, value: usize) -> Option<usize> {
        if self.contains(value) {
            assert!(value < self.start + self.len);
            Some(value - self.start)
        } else {
            None
        }
    }
    fn read_offset(&self, offset: usize) -> usize {
        self.start + offset
    }
}

#[derive(Debug, Clone)]
struct Map {
    source: String,
    source_ranges: Vec<Range>,
    destination_ranges: Vec<Range>,
}
impl Map {
    fn from_str(s: &str) -> Self {
        let (name, rest) = s.split_once(':').unwrap();
        let (source_s, _) = name.split_once('-').unwrap();
        let source = source_s.to_string();
        let mut source_ranges = Vec::new();
        let mut destination_ranges = Vec::new();
        for mapping in rest.lines() {
            if mapping.len() == 0 {
                continue;
            }
            let (dest_start, source_start, range) =
                mapping.split_ascii_whitespace().collect_tuple().unwrap();
            let dest_start = dest_start.parse::<usize>().unwrap();
            let source_start = source_start.parse::<usize>().unwrap();
            let range = range.parse::<usize>().unwrap();
            source_ranges.push(Range {
                start: source_start,
                len: range,
            });
            destination_ranges.push(Range {
                start: dest_start,
                len: range,
            });
        }
        Self {
            source,
            source_ranges,
            destination_ranges,
        }
    }

    fn map(&self, value: usize) -> usize {
        if self.source_ranges.iter().any(|r| r.contains(value)) {
            let source_range = self
                .source_ranges
                .iter()
                .enumerate()
                .find(|(_, r)| r.calc_offset(value).is_some())
                .unwrap();
            let offset = source_range.1.calc_offset(value).unwrap();
            let dest_range = &self.destination_ranges[source_range.0];
            dest_range.read_offset(offset)
        } else {
            value
        }
    }
}

fn parse_input() -> (Vec<usize>, Vec<Map>) {
    let mut lines = LINES.split("\r\n\r\n");
    let seeds = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect_vec();
    let maps = lines.map(Map::from_str).collect_vec();
    (seeds, maps)
}

fn get_location_for_seed(s: usize, maps: &[Map]) -> usize {
    let seed_to_soil = maps.iter().find(|s| s.source == "seed").unwrap();
    let soil_to_fert = maps.iter().find(|s| s.source == "soil").unwrap();
    let fert_to_water = maps.iter().find(|s| s.source == "fertilizer").unwrap();
    let water_to_light = maps.iter().find(|s| s.source == "water").unwrap();
    let light_to_temperature = maps.iter().find(|s| s.source == "light").unwrap();
    let temp_to_humidity = maps.iter().find(|s| s.source == "temperature").unwrap();
    let humidity_to_location = maps.iter().find(|s| s.source == "humidity").unwrap();
    let soil = seed_to_soil.map(s);
    let fert = soil_to_fert.map(soil);
    let water = fert_to_water.map(fert);
    let light = water_to_light.map(water);
    let temp = light_to_temperature.map(light);
    let humidity = temp_to_humidity.map(temp);
    humidity_to_location.map(humidity)
}
