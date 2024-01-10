use std::time::{Duration, Instant};

const WEAPONS: [(usize, usize, usize); 5] =
    [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];

const ARMOR: [(usize, usize, usize); 6] = [
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
    (0, 0, 0),
];

const RINGS: [(usize, usize, usize); 8] = [
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
    (0, 0, 0),
    (0, 0, 0),
];

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let now = Instant::now();
    let boss = Character::new(103, 9, 2);
    let mut least_cost = usize::MAX;
    let mut max_cost = usize::MIN;
    for weapon in WEAPONS.iter() {
        for armor in ARMOR.iter() {
            for ring1 in RINGS.iter() {
                for ring2 in RINGS.iter() {
                    if ring1 == ring2 {
                        continue;
                    }
                    let cost = weapon.0 + armor.0 + ring1.0 + ring2.0;
                    let damage = weapon.1 + armor.1 + ring1.1 + ring2.1;
                    let armor = weapon.2 + armor.2 + ring1.2 + ring2.2;
                    let player = Character::new(100, damage, armor);
                    if player_wins(&player, &boss) {
                        least_cost = least_cost.min(cost);
                    } else {
                        max_cost = max_cost.max(cost);
                    }
                }
            }
        }
    }
    ((least_cost, now.elapsed()), (max_cost, now.elapsed()))
}

#[derive(Debug, Copy, Clone)]
struct Character {
    hp: usize,
    damage: usize,
    armor: usize,
}

impl Character {
    fn new(hp: usize, damage: usize, armor: usize) -> Self {
        Self { hp, damage, armor }
    }
    fn attack(&self, other: &mut Self) {
        let damage = self.damage.saturating_sub(other.armor).max(1);
        other.hp = other.hp.saturating_sub(damage);
    }
}

fn player_wins(player: &Character, boss: &Character) -> bool {
    let mut player = *player;
    let mut boss = *boss;
    loop {
        player.attack(&mut boss);
        if boss.hp == 0 {
            return true;
        }
        boss.attack(&mut player);
        if player.hp == 0 {
            return false;
        }
    }
}
