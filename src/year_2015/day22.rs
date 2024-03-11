use std::{
    collections::BinaryHeap,
    time::{Duration, Instant},
};

const SPELLS: [Spell; 5] = [
    Spell {
        name: SpellType::MagicMissile,
        cost: 53,
        damage: 4,
        heal: 0,
        armor: 0,
        mana: 0,
        duration: 0,
    },
    Spell {
        name: SpellType::Drain,
        cost: 73,
        damage: 2,
        heal: 2,
        armor: 0,
        mana: 0,
        duration: 0,
    },
    Spell {
        name: SpellType::Shield,
        cost: 113,
        damage: 0,
        heal: 0,
        armor: 7,
        mana: 0,
        duration: 6,
    },
    Spell {
        name: SpellType::Poison,
        cost: 173,
        damage: 3,
        heal: 0,
        armor: 0,
        mana: 0,
        duration: 6,
    },
    Spell {
        name: SpellType::Recharge,
        cost: 229,
        damage: 0,
        heal: 0,
        armor: 0,
        mana: 101,
        duration: 5,
    },
];

const LINES: &str = include_str!("../../problem_inputs_2015/day_22.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve(false), solve(true))
}

fn solve(is_part_two: bool) -> (usize, Duration) {
    let now = Instant::now();
    let mut lines = LINES.lines();
    let boss_hp: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let boss_damage: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut situations: BinaryHeap<(Actor, Actor)> = BinaryHeap::new();
    let mut player = Actor {
        mana_consumed: 0,
        hp: 50,
        mana: 500,
        armor: 0,
        effects: Vec::new(),
    };
    if is_part_two {
        player.effects.push(Effect {
            effect_type: EffectType::Bleed,
            timer_left: usize::MAX,
        });
    }
    let boss = Actor {
        mana_consumed: 0,
        hp: boss_hp,
        mana: 0,
        armor: 0,
        effects: Vec::new(),
    };
    situations.push((player, boss));
    loop {
        let situation = situations.pop().unwrap();
        if situation.0.hp == 0 {
            continue;
        }
        if situation.1.hp == 0 {
            return (situation.0.mana_consumed.abs_diff(0), now.elapsed());
        }
        let usable_spells = get_applicable_spells(&situation.0, &situation.1);
        for spell in usable_spells {
            let mut new_player = situation.0.clone();
            let mut new_boss = situation.1.clone();
            complete_turn(&mut new_player, &mut new_boss, spell, boss_damage);
            if new_boss.hp == 0 {
                return (new_player.mana_consumed.abs_diff(0), now.elapsed());
            }
            if new_boss.hp > 0 && new_player.hp > 0 {
                situations.push((new_player, new_boss));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum EffectType {
    Shield,
    Poison,
    Recharge,
    Bleed,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Effect {
    effect_type: EffectType,
    timer_left: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}
struct Spell {
    name: SpellType,
    cost: usize,
    damage: usize,
    heal: usize,
    armor: usize,
    mana: usize,
    duration: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Actor {
    mana_consumed: isize,
    hp: usize,
    mana: usize,
    armor: usize,
    effects: Vec<Effect>,
}

fn complete_turn(player: &mut Actor, boss: &mut Actor, spell: SpellType, boss_damage: usize) {
    let spell = SPELLS.iter().find(|s| s.name == spell).unwrap();
    player.compute_effects();
    boss.compute_effects();

    player.cast_spell(spell, boss);

    player.compute_effects();
    boss.compute_effects();
    if boss.hp == 0 || player.hp == 0 {
        return;
    }
    player.take_damage(boss_damage);
}

fn get_applicable_spells(player: &Actor, boss: &Actor) -> Vec<SpellType> {
    let mut castable_spells = player.get_castable_spells();
    for effect in &player.effects {
        match effect.effect_type {
            EffectType::Recharge => {
                if let Some(index) = castable_spells
                    .iter()
                    .position(|spell| *spell == SpellType::Recharge)
                {
                    castable_spells.remove(index);
                }
            }
            EffectType::Shield => {
                if let Some(index) = castable_spells
                    .iter()
                    .position(|spell| *spell == SpellType::Shield)
                {
                    castable_spells.remove(index);
                }
            }
            _ => {}
        }
    }
    for effect in &boss.effects {
        match effect.effect_type {
            EffectType::Poison => {
                if let Some(index) = castable_spells
                    .iter()
                    .position(|spell| *spell == SpellType::Poison)
                {
                    castable_spells.remove(index);
                }
            }
            _ => {}
        }
    }
    castable_spells
}

impl Actor {
    fn cast_spell(&mut self, spell: &Spell, other: &mut Actor) {
        self.mana -= spell.cost;
        self.mana_consumed -= spell.cost as isize;
        match spell.name {
            SpellType::MagicMissile => {
                other.take_damage(4);
            }
            SpellType::Drain => {
                other.take_damage(2);
                self.hp += 2;
            }
            SpellType::Shield => {
                self.effects.push(Effect {
                    effect_type: EffectType::Shield,
                    timer_left: 6,
                });
            }
            SpellType::Poison => {
                other.effects.push(Effect {
                    effect_type: EffectType::Poison,
                    timer_left: 6,
                });
            }
            SpellType::Recharge => {
                self.effects.push(Effect {
                    effect_type: EffectType::Recharge,
                    timer_left: 5,
                });
            }
        }
    }

    fn get_castable_spells(&self) -> Vec<SpellType> {
        SPELLS
            .iter()
            .filter_map(|spell| {
                if (spell.cost <= self.mana) {
                    Some(spell.name)
                } else {
                    None
                }
            })
            .collect()
    }

    fn compute_effects(&mut self) {
        for effect in &self.effects.clone() {
            match effect.effect_type {
                EffectType::Shield => {
                    self.armor = 7;
                }
                EffectType::Poison => {
                    self.take_damage(3);
                }
                EffectType::Recharge => {
                    self.mana += 101;
                }
                EffectType::Bleed => {
                    self.hp = self.hp.saturating_sub(1);
                }
            }
        }
        self.effects.retain(|effect| effect.timer_left > 1);
        self.effects
            .iter_mut()
            .for_each(|effect| effect.timer_left -= 1);
    }
    fn take_damage(&mut self, damage: usize) {
        if self.armor > damage {
            self.armor -= damage;
            self.hp -= 1;
        } else {
            let reduced_damage = damage.saturating_sub(self.armor).max(1);
            self.armor = 0;
            self.hp = self.hp.saturating_sub(reduced_damage);
        }
    }
}
