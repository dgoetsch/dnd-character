use crate::core::ability_score::Ability;
use crate::core::roll::{AbilityScoreBonus, CheckBonus, CheckRollType, DamageRollScope};
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
#[serde(tag = "type")]
pub enum Effect {
    Ability {
        bonus: AbilityScoreBonus,
        ability: Ability,
    },
    Check {
        bonus: CheckBonus,
        roll: CheckRollType,
    },
    Damage {
        damage: Damage,
        scope: DamageRollScope,
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
            Effect::Damage {
                damage,
                scope: roll,
            } => {
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
