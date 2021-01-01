use crate::character::feature::Feature;
use crate::dimensions::Weight;
use serde::{Deserialize, Serialize};

type EquipmentType = String;
type DamageType = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    name: String,
    weight: Weight,

    equipment_type: EquipmentType,
    attack: Option<Attack>,

    properties: Vec<EquipmentProperty>,
    features: Vec<Feature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EquipmentProperty {
    Versatile { damage: Vec<Damage> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attack {
    range: AttackRange,
    damage: Vec<Damage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AttackRange {
    Melee,
    Ranged { standard: isize, extended: isize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    count: isize,
    sides: isize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Damage {
    dice: Dice,
    damage_type: DamageType,
}
