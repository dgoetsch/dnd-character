use crate::character::feature::{Feature, FeaturesState};
use crate::core::{display_damage, Damage};
use crate::dimensions::Weight;
use crate::equipment::weapon::WeaponProperty::{Range, Thrown, Versatile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WeaponProperty {
    Versatile {
        damage: Vec<Damage>,
    },
    Range {
        normal: isize,
        long: isize,
    },
    Thrown {
        normal: isize,
        long: isize,
        available: isize,
    },
    Ammunition {
        remaining: isize,
    },
    Reach,
    Loading,
    Reload,
    Misfire,
    Light,
    Finesse,
    Heavy,
    TwoHanded,
}

impl WeaponProperty {
    pub fn feature(&self) -> Feature {
        match self {
            Versatile { damage } => Feature::new("Versatile").with_description(format!(
                "Two handed Damage: {}",
                display_damage(damage.clone())
            )),
            Range { normal, long } => {
                Feature::new("Range").with_description(format!("{}/{}", normal, long))
            }
            Thrown {
                normal,
                long,
                available,
            } => Feature::new("Thrown")
                .with_description(format!("{}/{}", normal, long))
                .with_slot(available.clone(), Some(1)),
            _ => Feature::new("unimplemented"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attack {
    range: AttackRange,
    damage: Vec<Damage>,
    properties: Vec<WeaponProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AttackRange {
    Melee,
    Ranged { standard: isize, extended: isize },
}
