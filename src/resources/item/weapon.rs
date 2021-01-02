use crate::character::feature::{Feature, FeaturesState};
use crate::character::Message;
use crate::core::{display_damage, Damage};
use crate::dimensions::Weight;
use crate::util::two_column_row;
use iced::{Column, Row, Text};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use WeaponProperty::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    damage: Vec<Damage>,
    properties: Vec<WeaponProperty>,
}

impl Weapon {
    pub fn new(damage: Vec<Damage>, properties: Vec<WeaponProperty>) -> Weapon {
        Weapon { damage, properties }
    }
    pub fn view<'a>(self) -> Column<'a, Message> {
        let text = self
            .properties
            .clone()
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        Column::new()
            .push(two_column_row(
                Text::new("Damage"),
                Text::new(display_damage(self.damage)),
            ))
            .push(two_column_row(Text::new("Properties"), Text::new(text)))
    }
}

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
    Ammunition,
    Reach,
    Loading,
    Light,
    Finesse,
    Heavy,
    TwoHanded,
    Special,
}

impl Display for WeaponProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Versatile { damage } => write!(f, "Versatile({})", display_damage(damage.clone())),
            Range { normal, long } => write!(f, "Range({}/{})", normal, long),
            Thrown {
                normal,
                long,
                available,
            } => write!(f, "Thrown({}/{})", normal, long),
            Ammunition => write!(f, "Ammunition"),
            Reach => write!(f, "Reach"),
            Loading => write!(f, "Loading"),
            Light => write!(f, "Light"),
            Finesse => write!(f, "Finesse"),
            Heavy => write!(f, "Heavy"),
            TwoHanded => write!(f, "Two Handed"),
            Special => write!(f, "Special"),
        }
    }
}
