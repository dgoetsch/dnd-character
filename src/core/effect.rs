use crate::core::ability_score::Ability;
use crate::core::{Damage, Dice};
use crate::resources::skill::SkillName;
use crate::util::format_modifier;
use iced::{Column, Element, HorizontalAlignment, Length, Row, Text, VerticalAlignment};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

pub type DamageType = String;

#[derive(Debug, Clone, Default)]
pub struct EffectsState {
    effects: Vec<EffectState>,
}

impl EffectsState {
    pub fn effect(&self) -> Vec<Effect> {
        let EffectsState { effects } = self;
        let mut result = vec![];
        for state in effects {
            result.push(state.effect());
        }
        result
    }

    pub fn from(effects: Vec<Effect>) -> EffectsState {
        EffectsState {
            effects: effects.into_iter().map(|e| e.to_state()).collect(),
        }
    }

    pub fn persistable(&self) -> Vec<Effect> {
        self.effects
            .clone()
            .into_iter()
            .map(|e| e.persistable())
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    pub fn view<'a, T>(&'a mut self) -> Column<'a, T>
    where
        T: Debug + Clone + 'a,
    {
        let EffectsState { effects } = self;
        let mut column = Column::new();
        for effect in effects {
            column = column.push(Row::new().push(effect.view()));
        }
        column
    }
}

#[derive(Debug, Clone)]
pub struct EffectState {
    effect: Effect,
}

impl EffectState {
    pub fn effect(&self) -> Effect {
        self.effect.clone()
    }

    pub fn persistable(&self) -> Effect {
        self.effect.clone()
    }

    pub fn view<'a, T>(&'a mut self) -> Element<'a, T>
    where
        T: Debug + Clone + 'a,
    {
        Text::new(self.effect.to_string()).size(16).into()
    }
}

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
#[serde(tag = "type")]
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
#[serde(tag = "type")]
pub enum CheckRoll {
    SavingThrow(Ability),
    Ability(Ability),
    Attack,
    SpellAttack,
    Skill(SkillName),
    Feature(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct CheckRollModifier {
    bonus: isize,
    dice: Vec<Dice>,
    advantage_count: isize,
    disadvantage_count: isize,
}

impl CheckRollModifier {
    pub fn from(bonuses: Vec<CheckBonus>) -> CheckRollModifier {
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

        CheckRollModifier {
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

    pub fn merge(&self, other: CheckRollModifier) -> CheckRollModifier {
        let mut dice = self.dice.clone();
        dice.extend(other.dice);
        CheckRollModifier {
            bonus: self.bonus + other.bonus,
            dice: dice,
            advantage_count: self.advantage_count + other.advantage_count,
            disadvantage_count: self.disadvantage_count + other.disadvantage_count,
        }
    }

    pub fn with_extra_bonus(&self, bonus: isize) -> CheckRollModifier {
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

impl Display for CheckRoll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckRoll::SavingThrow(ability) => write!(f, "{:?} saving throws", ability),
            CheckRoll::Ability(ability) => write!(f, "{:?} checks", ability),
            CheckRoll::Attack => write!(f, "attack rolls"),
            CheckRoll::SpellAttack => write!(f, "spell attack rolls"),
            CheckRoll::Skill(name) => write!(f, "{} checks", name),
            CheckRoll::Feature(path) => write!(f, "{}", path.join(" ")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum DamageRoll {
    Attack,
    Spell,
    Feature(Vec<String>),
}

impl Display for DamageRoll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageRoll::Attack => write!(f, "Attack Damage"),
            DamageRoll::Spell => write!(f, "Spell Damage"),
            DamageRoll::Feature(path) => write!(f, "Damage from {}", path.join(" ")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum AbilityScoreBonus {
    Modifier { modifier: isize },
    Become { value: isize },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Effect {
    Ability {
        bonus: AbilityScoreBonus,
        ability: Ability,
    },
    Check {
        bonus: CheckBonus,
        roll: CheckRoll,
    },
    Damage {
        damage: Damage,
        roll: DamageRoll,
    },
}

impl Effect {
    pub fn to_state(self) -> EffectState {
        EffectState { effect: self }
    }
}

impl Display for Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Ability { bonus, ability } => match bonus {
                AbilityScoreBonus::Modifier { modifier } => {
                    write!(f, "{} {:?}", format_modifier(modifier.clone()), ability)
                }
                AbilityScoreBonus::Become { value } => {
                    write!(f, "{:?} becomes {}", ability, value)
                }
            },
            Effect::Check { bonus, roll } => match bonus {
                CheckBonus::Advantage(advantage) => {
                    write!(f, "{} on {}", advantage.to_string(), roll.to_string())
                }
                CheckBonus::Modifier(modifier) => {
                    write!(f, "{} to {}", bonus.to_string(), roll.to_string())
                }
                CheckBonus::Dice(dice) => {
                    write!(f, "{} to {}", bonus.to_string(), roll.to_string())
                }
            },
            Effect::Damage { damage, roll } => {
                write!(f, "{} to {}", damage.to_string(), roll.to_string())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn ability_becomes(value: isize, ability: Ability) -> Effect {
        let bonus = AbilityScoreBonus::Become { value };
        Effect::Ability { bonus, ability }
    }

    #[test]
    pub fn wand_of_the_war_mage() {
        let effect = ability_becomes(19, Ability::Constitution);
        // println!(
        //     "{}",
        //     serde_json::to_string_pretty(&effect).unwrap_or("".to_string())
        // );
        assert_eq!(effect.to_string(), "Constitution becomes 19".to_string());
    }
}
