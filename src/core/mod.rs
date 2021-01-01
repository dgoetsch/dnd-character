use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

type DamageType = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    count: isize,
    sides: isize,
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
