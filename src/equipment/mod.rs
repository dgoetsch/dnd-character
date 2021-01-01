use crate::character::feature::{Feature, FeaturesState};
use crate::dimensions::Weight;
use crate::equipment::weapon::Attack;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

mod weapon;

type EquipmentType = String;

pub struct EquipmentState {
    equipment: Equipment,
    features_state: FeaturesState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    name: String,
    weight: Weight,
    equipment_type: EquipmentType,
    attack: Option<Attack>,
    features: Vec<Feature>,
}

impl EquipmentState {
    // pub fn from(equipment: Equipment) -> EquipmentState {
    //
    // }
}
