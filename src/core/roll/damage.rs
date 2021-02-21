use crate::core::effect::Effect;
use crate::core::feature_path::FeaturePath;
use crate::core::roll::Dice;
use crate::util::format_modifier;
use iced::{Column, Element, HorizontalAlignment, Length, Row, Text, VerticalAlignment};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

pub type DamageType = String;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
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
                .filter(|a| *a != 0)
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum DamageRollScope {
    Attack, //TODO attack type
    Spell,
    Feature(FeaturePath),
}

impl Display for DamageRollScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageRollScope::Attack => write!(f, "Attack Damage"),
            DamageRollScope::Spell => write!(f, "Spell Damage"),
            DamageRollScope::Feature(path) => write!(f, "Damage from {}", path.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DamageRoll {
    damage: Vec<Damage>,
}

impl DamageRoll {
    pub fn from(damage: Vec<Damage>) -> DamageRoll {
        DamageRoll { damage }
    }

    pub fn with_modifier(&self, modifier: isize) -> DamageRoll {
        let mut damage = self.damage.clone();
        match damage.first_mut() {
            Some(head) => head.additional = Some(modifier + head.additional.unwrap_or(0)),
            None => damage.push(Damage::new(
                Dice::new(0, 0),
                Some(modifier),
                "Unknown".to_string(),
            )),
        };
        DamageRoll { damage }
    }

    pub fn with_extra_damage(&self, additional: Damage) -> DamageRoll {
        let mut damage = self.damage.clone();
        damage.push(additional);
        DamageRoll { damage }
    }

    pub fn view<'a, T: Debug + Clone>(self) -> Element<'a, T> {
        let damage = self
            .damage
            .into_iter()
            .map(|d| d.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join(" + ");

        Text::new(damage)
            .size(16)
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Bottom)
            .width(Length::FillPortion(1))
            .into()
    }
}
