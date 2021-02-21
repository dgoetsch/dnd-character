use crate::core::ability_score::ModifiedAbilityScores;
use crate::core::effect::Effect;
use crate::core::feature_path::FeaturePath;
use crate::core::roll::check::{CheckBonus, CheckRoll, CheckRollType};
use crate::core::roll::damage::{Damage, DamageRoll, DamageRollScope};

use crate::util::two_element_row;
use iced::{Column, HorizontalAlignment, Row, Text, VerticalAlignment};
use serde::export::fmt::Debug;
use serde::export::Formatter;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum AttackRange {
    Melee,
    Range { normal: isize, long: isize },
}

impl Display for AttackRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AttackRange::Melee => write!(f, "Melee"),
            AttackRange::Range { normal, long } => write!(f, "Ranged({}/{})", normal, long),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attack {
    name: String,
    range: AttackRange,
    attack: CheckRoll,
    damage: DamageRoll,
}

impl Attack {
    pub fn new<T: Into<String>>(
        name: T,
        range: AttackRange,
        attack: CheckRoll,
        damage: DamageRoll,
    ) -> Attack {
        Attack {
            name: name.into(),
            range,
            attack,
            damage,
        }
    }
    pub fn matches(&self, featurePath: FeaturePath) -> (bool, FeaturePath) {
        featurePath.matches(self.name.clone())
    }

    pub fn with_extra_damage(&self, additional: Damage) -> Attack {
        let mut attack = self.clone();
        attack.damage = attack.damage.with_extra_damage(additional);
        attack
    }

    pub fn with_extra_check(&self, additional: CheckRoll) -> Attack {
        let mut attack = self.clone();
        attack.attack = attack.attack.merge(additional);
        attack
    }

    pub fn view<'a, T: Clone + Debug + 'a>(self) -> Column<'a, T> {
        Column::new()
            .push(Row::new().push(Text::new(format!("{} Attack", self.name))))
            .push(Row::new().push(Text::new(self.range.to_string())))
            .push(two_element_row(
                Text::new("To Hit")
                    .size(16)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Bottom)
                    .into(),
                self.attack.view(),
            ))
            .push(two_element_row(
                Text::new("Damage")
                    .size(16)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Bottom)
                    .into(),
                self.damage.view(),
            ))
    }
}
