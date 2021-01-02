use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::util::format_modifier;

pub mod ability_score;
pub mod effect;
pub mod feature;

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
    additional: Option<isize>,
    damage_type: DamageType,
}

impl Damage {
    pub fn new(dice: Dice, additional: Option<isize>, damage_type: DamageType) -> Damage {
        Damage {
            dice,
            additional,
            damage_type,
        }
    }
}

impl Display for Damage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let damage = vec![
            Some(self.dice.clone())
                .filter(|d| d.count > 0)
                .map(|d| d.to_string()),
            self.additional
                .map(|additional| format_modifier(additional)),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("");

        write!(f, "{} {}", damage, self.damage_type)
    }
}

pub fn display_damage(damage: Vec<Damage>) -> String {
    damage
        .into_iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(" , ")
}
