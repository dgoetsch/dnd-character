use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub mod ability_score;

type DamageType = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    count: isize,
    sides: isize,
}

impl Dice {
    pub fn new(count: isize, sides: isize) -> Dice {
        Dice { count, sides }
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.count, self.sides)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Damage {
    dice: Dice,
    damage_type: DamageType,
}

impl Damage {
    pub fn new(dice: Dice, damage_type: DamageType) -> Damage {
        Damage { dice, damage_type }
    }
}

impl Display for Damage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.dice.to_string(), self.damage_type)
    }
}

pub fn display_damage(damage: Vec<Damage>) -> String {
    damage
        .into_iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(" , ")
}
