use std::time::{Duration, Instant};

const CBOUNDS: isize = 10;

use fxhash::FxHashMap;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2020/day_17.txt");
    let mut big_cube = Reactor::new();
    for (y, line) in lines.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                big_cube.add_cube(x as isize, y as isize, 0, 0, true);
            } else {
                big_cube.add_cube(x as isize, y as isize, 0, 0, false);
            }
        }
    }
    (solve01(big_cube.clone()), solve02(big_cube))
}

fn solve01(mut big_cube: Reactor) -> (usize, Duration) {
    let now = Instant::now();
    for c in 0..6 {
        big_cube.cycle_3d(c);
    }
    (big_cube.count_active(), now.elapsed())
}

fn solve02(mut big_cube: Reactor) -> (usize, Duration) {
    let now = Instant::now();
    for cycle in 0..6 {
        big_cube.cycle_4d(cycle);
    }
    (big_cube.count_active(), now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Reactor {
    core: FxHashMap<(isize, isize, isize, isize), bool>,
}

impl Reactor {
    fn new() -> Self {
        Self {
            core: FxHashMap::default(),
        }
    }

    fn add_cube(&mut self, x: isize, y: isize, z: isize, w: isize, status: bool) {
        self.core.insert((x, y, z, w), status);
    }

    fn read_cube(&self, x: isize, y: isize, z: isize, w: isize) -> bool {
        *self.core.get(&(x, y, z, w)).unwrap_or(&false)
    }

    fn count_active(&self) -> usize {
        self.core.values().filter(|&&v| v).count()
    }

    fn get_neighbor_active_count(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue;
                        }
                        if self.read_cube(x + dx, y + dy, z + dz, w + dw) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn cycle_3d(&mut self, cycle: isize) {
        let old_state = self.clone();
        let bounds = CBOUNDS + cycle;
        for x in -bounds..bounds {
            for y in -bounds..bounds {
                for z in -bounds..bounds {
                    let count = old_state.get_neighbor_active_count(x, y, z, 0);
                    let cube_status = old_state.read_cube(x, y, z, 0);
                    if cube_status {
                        if count != 2 && count != 3 {
                            self.core.insert((x, y, z, 0), false);
                        }
                    } else {
                        if count == 3 {
                            self.core.insert((x, y, z, 0), true);
                        }
                    }
                }
            }
        }
    }

    fn cycle_4d(&mut self, cycle: isize) {
        let old_state = self.clone();
        let bounds = CBOUNDS + cycle;
        for x in -bounds..bounds {
            for y in -bounds..bounds {
                for z in -bounds..bounds {
                    for w in -bounds..bounds {
                        let count = old_state.get_neighbor_active_count(x, y, z, w);
                        let cube_status = old_state.read_cube(x, y, z, w);
                        if cube_status {
                            if count != 2 && count != 3 {
                                self.core.insert((x, y, z, w), false);
                            }
                        } else {
                            if count == 3 {
                                self.core.insert((x, y, z, w), true);
                            }
                        }
                    }
                }
            }
        }
    }
}
