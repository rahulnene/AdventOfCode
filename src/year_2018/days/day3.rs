use fxhash::FxHashMap;
use itertools::Itertools;

use regex::Regex;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_3.txt");
    let mut claims: FxHashMap<usize, Claim> = FxHashMap::default();
    let mut fabric: Fabric = FxHashMap::default();
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for line in lines.lines() {
        let found = re
            .captures(line)
            .unwrap()
            .extract::<5>()
            .1
            .iter()
            .map(|k|str::parse::<usize>(k).unwrap())
            .collect_vec();
        let claim = Claim::new(&found);
        claims.insert(claim.id, claim);
        for x in claim.left_padding..claim.left_padding + claim.width {
            for y in claim.top_padding..claim.top_padding + claim.height {
                if fabric.contains_key(&(x, y)) {
                    let temp = fabric.get_mut(&(x, y)).unwrap();
                    temp.0 += 1;
                    temp.1.push(claim.id);
                } else {
                    fabric.insert((x, y), (1, vec![claim.id]));
                }
            }
        }
    }
    match part {
        1 => solve01(&fabric),
        2 => solve02(&fabric, &claims),
        _ => 1,
    }
}

fn solve01(fabric: &FxHashMap<Position, (usize, Vec<usize>)>) -> usize {
    fabric.values().filter(|f| f.0 > 1).count()
}

fn solve02(fabric: &Fabric, claims: &FxHashMap<usize, Claim>) -> usize {
    let unique_claims = claims.keys().unique();
    let solo_claimed = fabric
        .values()
        .filter_map(|f| if f.0 == 1 { Some(f.1[0]) } else { None })
        .collect_vec();
    for unique_claimer in unique_claims {
        let sqinches = &solo_claimed.iter().filter(|f| *f == unique_claimer).count();
        let claim_to_check = claims.get(&unique_claimer).unwrap();
        if *sqinches == claim_to_check.height * claim_to_check.width {
            return *unique_claimer;
        }
    }
    unreachable!()
}

#[derive(Debug, Clone, Copy)]
struct Claim {
    id: usize,
    left_padding: usize,
    top_padding: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn new(params: &[usize]) -> Self {
        Claim {
            id: params[0],
            left_padding: params[1],
            top_padding: params[2],
            width: params[3],
            height: params[4],
        }
    }
}

type Position = (usize, usize);
type Fabric = FxHashMap<Position, (usize, Vec<usize>)>;