use crate::character::class::{Class, Classes};
use crate::character::proficiencies::Proficiencies;
use crate::character::Message;
use crate::core::ability_score::{
    Ability, AbilityScore, AbilityScores, ModifiedAbilityScore, ModifiedAbilityScores,
};
use crate::core::effect::Effect;
use crate::core::roll::{CheckBonus, CheckRoll, CheckRollType};
use crate::util::{format_modifier, two_column_row, two_element_row};
use iced::{Column, HorizontalAlignment, Row, Text, VerticalAlignment};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SpellcastingsState {
    spellcasting_states: Vec<SpellcastingState>,
}

impl SpellcastingsState {
    pub fn from(persistable: Vec<Spellcasting>, classes: Classes) -> SpellcastingsState {
        let proficiency = classes.proficiency_bonus();
        SpellcastingsState {
            spellcasting_states: persistable
                .clone()
                .into_iter()
                .map(|s| SpellcastingState::from(s, proficiency))
                .collect(),
        }
    }

    pub fn persistable(&self) -> Vec<Spellcasting> {
        self.spellcasting_states
            .clone()
            .into_iter()
            .map(|s| s.persistable())
            .collect()
    }

    pub fn apply_all(&mut self, effects: &Vec<Effect>) {
        let SpellcastingsState {
            spellcasting_states,
        } = self;

        for state in spellcasting_states {
            state.apply_all(effects)
        }
    }

    pub fn modified(
        &self,
        modified_ability_scores: &ModifiedAbilityScores,
    ) -> ModifiedSpellcastings {
        let SpellcastingsState {
            spellcasting_states,
        } = self;

        let mut spellcastings = vec![];

        for state in spellcasting_states {
            spellcastings.push(state.modified(modified_ability_scores))
        }
        ModifiedSpellcastings { spellcastings }
    }

    pub fn view(&mut self, ability_scores: ModifiedAbilityScores) -> Column<Message> {
        let SpellcastingsState {
            spellcasting_states,
        } = self;
        let mut column = Column::new();

        for state in spellcasting_states {
            column = column.push(state.view(&ability_scores))
        }
        column
    }
}

#[derive(Debug, Clone)]
pub struct ModifiedSpellcastings {
    spellcastings: Vec<ModifiedSpellcasting>,
}

impl ModifiedSpellcastings {
    pub fn spellcastings(&self) -> Vec<ModifiedSpellcasting> {
        self.spellcastings.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SpellcastingState {
    spellcasting: Spellcasting,
    proficiency_modifier: isize,
    spell_modifiers: Vec<CheckBonus>,
}

#[derive(Debug, Clone)]
pub struct ModifiedSpellcasting {
    class: String,
    spellcasting: CheckRoll,
}

impl SpellcastingState {
    fn from(persistable: Spellcasting, proficiency_modifier: isize) -> SpellcastingState {
        SpellcastingState {
            spellcasting: persistable,
            proficiency_modifier,
            spell_modifiers: vec![],
        }
    }

    fn persistable(&self) -> Spellcasting {
        self.spellcasting.clone()
    }

    fn reset_effects(&mut self) {
        self.spell_modifiers = vec![]
    }

    pub fn apply_all(&mut self, effects: &Vec<Effect>) {
        for effect in effects {
            self.apply(effect.clone())
        }
    }

    pub fn apply(&mut self, effect: Effect) {
        match effect {
            Effect::Check { bonus, roll } => match roll {
                CheckRollType::SpellAttack => self.spell_modifiers.push(bonus),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn modified(&self, ability_scores: &ModifiedAbilityScores) -> ModifiedSpellcasting {
        ModifiedSpellcasting {
            class: self.spellcasting.class.clone(),
            spellcasting: self
                .spellcasting
                .clone()
                .modifier(
                    self.proficiency_modifier.clone(),
                    &ability_scores.get(self.spellcasting.ability.clone()),
                )
                .merge(CheckRoll::from(self.spell_modifiers.clone())),
        }
    }

    pub fn save_dc(&self, ability_scores: &ModifiedAbilityScores) -> isize {
        self.proficiency_modifier
            + ability_scores
                .get(self.spellcasting.ability.clone())
                .score()
                .modifier()
            + 8
            + self
                .spellcasting
                .additional_save_modifiers
                .values()
                .sum::<isize>()
    }

    pub fn view<'a>(&mut self, ability_scores: &ModifiedAbilityScores) -> Column<'a, Message> {
        let spell_modifier = self.modified(ability_scores).spell_modifier();
        let save_dc = self.save_dc(ability_scores);
        Column::new()
            .push(
                Row::new()
                    .push(Text::new(format!("{} spellcasting", self.spellcasting.class)).size(24)),
            )
            .push(two_element_row(
                Text::new("Modifier")
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Bottom)
                    .size(16)
                    .into(),
                spell_modifier.view().into(),
            ))
            .push(two_column_row(
                Text::new("Spell Save DC").size(16),
                Text::new(save_dc.to_string()).size(16),
            ))
    }
}

impl ModifiedSpellcasting {
    pub fn class_name(&self) -> String {
        self.class_name().clone()
    }
    pub fn spell_modifier(&self) -> CheckRoll {
        self.spellcasting.clone()
    }
}

pub fn view<'a>(
    spellcasting: Vec<Spellcasting>,
    class: &Classes,
    ability_scores: ModifiedAbilityScores,
) -> Column<'a, Message> {
    let mut column = Column::new();
    let proficiency_modifier = class.proficiency_bonus();

    for casting in spellcasting {
        let score = ability_scores.get(casting.ability.clone());
        column = column.push(casting.view(proficiency_modifier, score))
    }
    column
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spellcasting {
    class: String,
    ability: Ability,
    additional_modifiers: HashMap<String, isize>,
    additional_save_modifiers: HashMap<String, isize>,
}

impl Spellcasting {
    fn modifier(&self, proficiency_modifier: isize, ability: &ModifiedAbilityScore) -> CheckRoll {
        ability
            .roll()
            .with_extra_bonus(proficiency_modifier)
            .with_extra_bonus(self.additional_modifiers.values().sum::<isize>())
    }

    fn save_dc(&self, proficiency_modifier: isize, ability: &ModifiedAbilityScore) -> isize {
        proficiency_modifier
            + ability.score().modifier()
            + 8
            + self.additional_save_modifiers.values().sum::<isize>()
    }

    pub fn view<'a>(
        self,
        proficiency_modifier: isize,
        ability: ModifiedAbilityScore,
    ) -> Column<'a, Message> {
        Column::new()
            .push(Row::new().push(Text::new(format!("{} spellcasting", self.class)).size(24)))
            .push(two_element_row(
                Text::new("Modifier")
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Bottom)
                    .size(16)
                    .into(),
                self.modifier(proficiency_modifier, &ability).view().into(),
            ))
            .push(two_column_row(
                Text::new("Spell Save DC").size(16),
                Text::new(self.save_dc(proficiency_modifier, &ability).to_string()).size(16),
            ))
    }
}
