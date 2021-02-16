use crate::core::ability_score::Ability;
use crate::core::feature_path::FeaturePath;
use crate::core::{Damage, Dice};
use crate::resources::skill::SkillName;
use crate::util::format_modifier;
use iced::{Column, Element, HorizontalAlignment, Length, Row, Text, VerticalAlignment};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Advantage {
    Advantage,
    Disadvantage,
}

impl Display for Advantage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Advantage::Advantage => write!(f, "Advantage"),
            Advantage::Disadvantage => write!(f, "Disadvantage"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum CheckBonus {
    Advantage(Advantage),
    Modifier(isize),
    Dice(Dice),
}

impl Display for CheckBonus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckBonus::Advantage(advantage) => write!(f, "{}", advantage),
            CheckBonus::Modifier(modifier) => write!(f, "{}", format_modifier(modifier.clone())),
            CheckBonus::Dice(dice) => write!(f, "{}", dice.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum CheckRollType {
    SavingThrow(Ability),
    Ability(Ability),
    Attack,
    SpellAttack,
    Skill(SkillName),
    Feature(FeaturePath),
}

impl Display for CheckRollType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckRollType::SavingThrow(ability) => write!(f, "{:?} saving throws", ability),
            CheckRollType::Ability(ability) => write!(f, "{:?} checks", ability),
            CheckRollType::Attack => write!(f, "attack rolls"),
            CheckRollType::SpellAttack => write!(f, "spell attack rolls"),
            CheckRollType::Skill(name) => write!(f, "{} checks", name),
            CheckRollType::Feature(path) => write!(f, "{}", path.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum DamageRollScope {
    Attack,
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum AbilityScoreBonus {
    Modifier { modifier: isize },
    Become { value: isize },
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

#[derive(Debug, Clone)]
pub struct CheckRoll {
    bonus: isize,
    dice: Vec<Dice>,
    advantage_count: isize,
    disadvantage_count: isize,
}

impl CheckRoll {
    pub fn from(bonuses: Vec<CheckBonus>) -> CheckRoll {
        let mut advantage_count = 0;
        let mut disadvantage_count = 0;
        let mut static_modifiers = vec![];
        let mut dice = vec![];

        for bonus in bonuses {
            match bonus {
                CheckBonus::Advantage(Advantage::Advantage) => {
                    advantage_count = advantage_count + 1
                }
                CheckBonus::Advantage(Advantage::Disadvantage) => {
                    disadvantage_count = disadvantage_count + 1
                }
                CheckBonus::Modifier(bonus) => static_modifiers.push(bonus),
                CheckBonus::Dice(some_dice) => dice.push(some_dice.clone()),
            }
        }

        let bonus = static_modifiers.into_iter().sum();

        CheckRoll {
            bonus,
            dice,
            advantage_count,
            disadvantage_count,
        }
    }

    pub fn advantage(&self) -> Option<Advantage> {
        if self.advantage_count == self.disadvantage_count {
            None
        } else if self.advantage_count > self.disadvantage_count {
            Some(Advantage::Advantage)
        } else {
            Some(Advantage::Disadvantage)
        }
    }

    pub fn dice(&self) -> Vec<Dice> {
        self.dice.clone()
    }

    pub fn bonus(&self) -> isize {
        self.bonus
    }

    pub fn merge(&self, other: CheckRoll) -> CheckRoll {
        let mut dice = self.dice.clone();
        dice.extend(other.dice);
        CheckRoll {
            bonus: self.bonus + other.bonus,
            dice: dice,
            advantage_count: self.advantage_count + other.advantage_count,
            disadvantage_count: self.disadvantage_count + other.disadvantage_count,
        }
    }

    pub fn with_extra_bonus(&self, bonus: isize) -> CheckRoll {
        let mut new = self.clone();
        new.bonus = new.bonus + bonus;
        new
    }

    pub fn view<'a, T: Debug + Clone>(self) -> Element<'a, T> {
        let dice = Some(
            self.dice()
                .into_iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join("+"),
        )
        .filter(|s| !s.is_empty());

        let bonus = Some(format_modifier(self.bonus())).filter(|s| !s.is_empty());
        let advantage = self.advantage().map(|a| format!("({})", a.to_string()));
        let bonus_dice_and_modifier = Some(
            vec![dice, bonus]
                .into_iter()
                .flatten()
                .collect::<Vec<String>>()
                .join("+"),
        )
        .filter(|b| !b.is_empty());

        let text = vec![bonus_dice_and_modifier, advantage]
            .into_iter()
            .flatten()
            .collect::<Vec<String>>()
            .join(" ");

        Text::new(text)
            .size(16)
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Bottom)
            .width(Length::FillPortion(1))
            .into()
    }
}
