use std::{
    fmt::{Debug, Display},
    time::{Duration, Instant},
};

use itertools::Itertools;
use rayon::vec;
use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2019/day_14_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut formulae = FxHashMap::default();
    for line in LINES.lines() {
        let reaction = Reaction::from_str(line);
        formulae.insert(reaction.formula.keys().next().unwrap().clone(), reaction);
    }
    dbg!(&formulae);
    let mut requirements = vec![Amount::from_str("1 FUEL")];
    dbg!(&requirements);
    get_requirements(&requirements[0].clone(), &formulae, &mut requirements);
    dbg!(&requirements);
    get_requirements(&requirements[0].clone(), &formulae, &mut requirements);
    dbg!(&requirements);
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Clone)]
struct Reaction {
    formula: FxHashMap<Amount, Vec<Amount>>,
}

impl Reaction {
    fn from_str(s: &str) -> Reaction {
        let mut formula = FxHashMap::default();
        for line in s.lines() {
            let mut iter = line.split(" => ");
            let inputs = iter.next().unwrap();
            let output = iter.next().unwrap();
            let output = Amount::from_str(output);
            let inputs = inputs.split(", ").map(Amount::from_str).collect();
            formula.insert(output, inputs);
        }
        Reaction { formula }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Amount {
    quantity: usize,
    chemical: String,
}

impl Amount {
    fn from_str(s: &str) -> Amount {
        let mut iter = s.split_whitespace();
        let quantity = iter.next().unwrap().parse().unwrap();
        let chemical = iter.next().unwrap().to_string();
        Amount { quantity, chemical }
    }

    fn get_requirements(&self, reactions: &FxHashMap<Amount, Reaction>) -> Option<Vec<Amount>> {
        let amount = self.quantity;
        let reaction = reactions.get(&self)?;
        let mut requirements = vec![];
        for (k, v) in reaction.formula.iter() {
            let mut amount = k.clone();
            amount.quantity *= self.quantity;
            requirements.push(amount);
        }
        Some(requirements)
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.quantity, self.chemical)
    }
}

impl Debug for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.quantity, self.chemical)
    }
}

impl Debug for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self.formula.iter() {
            write!(f, "{k} => {v:?}")?;
        }
        Ok(())
    }
}

fn get_requirements(
    amount: &Amount,
    reactions: &FxHashMap<Amount, Reaction>,
    requirements: &mut Vec<Amount>,
) -> Option<()> {
    let reaction = reactions.get(&amount)?;
    for (i, (k, v)) in reaction.formula.iter().enumerate() {
        let mut amount = k.clone();
        amount.quantity *= amount.quantity;
        let v_new = v
            .iter()
            .map(|c| Amount {
                quantity: c.quantity * amount.quantity,
                ..c.clone()
            })
            .collect_vec();
        requirements.extend(v_new);
        requirements.remove(i);
    }
    Some(())
}
