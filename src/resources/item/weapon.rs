use crate::character::proficiencies::{Proficiency, ProficiencyType};
use crate::character::Message;
use crate::core::ability_score::{Ability, ModifiedAbilityScore, ModifiedAbilityScores};
use crate::core::attack::{AttackRange, WeaponAttack};
use crate::core::effect::Effect;
use crate::core::feature::{Feature, FeaturesState};
use crate::core::roll::{CheckRoll, DamageRoll};
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
    pub fn attacks(&self, ability_scores: ModifiedAbilityScores) -> Vec<WeaponAttack> {
        let Weapon { damage, properties } = self;

        let mut range = None;
        let mut thrown = None;
        let mut versatile = None;
        let mut finesse = false;

        for property in properties {
            match property {
                WeaponProperty::Range { normal, long } => {
                    range = Some(AttackRange::Range {
                        normal: normal.clone(),
                        long: long.clone(),
                    })
                }
                WeaponProperty::Thrown { normal, long } => {
                    thrown = Some(AttackRange::Range {
                        normal: normal.clone(),
                        long: long.clone(),
                    })
                }
                WeaponProperty::Versatile { damage } => versatile = Some(damage),
                WeaponProperty::Finesse => finesse = true,
                _ => {}
            }
        }

        let dexterity_roll = ability_scores.get(Ability::Dexterity).roll();
        let strength_roll = ability_scores.get(Ability::Strength).roll();

        //TODO proficiency
        //TODO other bonuses
        let strengh_attack_roll = strength_roll.clone();
        let dexterity_attack_roll = dexterity_roll.clone();

        let dexterity_damage =
            DamageRoll::from(damage.clone()).with_modifier(dexterity_roll.bonus());
        let strength_damage = DamageRoll::from(damage.clone()).with_modifier(strength_roll.bonus());

        let mut rolls = vec![];
        match range {
            Some(range) => rolls.push(WeaponAttack::new(
                "Range",
                range,
                dexterity_attack_roll.clone(),
                dexterity_damage.clone(),
            )),
            None => rolls.push(WeaponAttack::new(
                "Melee",
                AttackRange::Melee,
                strengh_attack_roll.clone(),
                strength_damage.clone(),
            )),
        }

        match thrown {
            Some(range) => rolls.push(WeaponAttack::new(
                "Throw",
                range,
                dexterity_attack_roll.clone(),
                dexterity_damage.clone(),
            )),
            None => {}
        }

        if finesse {
            rolls.push(WeaponAttack::new(
                "Melee (Finesse)",
                AttackRange::Melee,
                dexterity_attack_roll.clone(),
                dexterity_damage.clone(),
            ))
        }

        match versatile {
            Some(damage) => rolls.push(WeaponAttack::new(
                "Melee (Two-Handed)",
                AttackRange::Melee,
                strengh_attack_roll.clone(),
                strength_damage.clone(),
            )),
            _ => {}
        }

        rolls
    }

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
    Versatile { damage: Vec<Damage> },
    Range { normal: isize, long: isize },
    Thrown { normal: isize, long: isize },
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
            Thrown { normal, long } => write!(f, "Thrown({}/{})", normal, long),
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
