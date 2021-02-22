use crate::character::class::Classes;
use crate::character::proficiencies::{Proficiency, ProficiencyType};
use crate::character::Message;
use crate::core::ability_score::{
    Ability, AbilityScore, AbilityScores, ModifiedAbilityScore, ModifiedAbilityScores,
};
use crate::core::roll::check::CheckBonus;
use crate::core::roll::rollable::Rollable;
use iced::{Column, Length, Row, Text};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct SavingThrowsState {
    saving_throws: SavingThrows,
    strength: SavingThrowState,
    dexterity: SavingThrowState,
    constitution: SavingThrowState,
    intelligence: SavingThrowState,
    wisdom: SavingThrowState,
    charisma: SavingThrowState,
}

pub struct SavingThrowState {
    effect: Vec<CheckBonus>,
}

pub struct ModifiedSavingThrows {
    strength: ModifiedSavingThrow,
    dexterity: ModifiedSavingThrow,
    constitution: ModifiedSavingThrow,
    intelligence: ModifiedSavingThrow,
    wisdom: ModifiedSavingThrow,
    charisma: ModifiedSavingThrow,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SavingThrow {
    additional_modifiers: HashMap<String, isize>,
}

pub struct ModifiedSavingThrow {
    additional_modifiers: HashMap<String, isize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SavingThrows {
    strength: SavingThrow,
    dexterity: SavingThrow,
    constitution: SavingThrow,
    intelligence: SavingThrow,
    wisdom: SavingThrow,
    charisma: SavingThrow,
}
impl SavingThrows {
    pub fn get(&self, ability: Ability) -> SavingThrow {
        match ability {
            Ability::Strength => self.strength.clone(),
            Ability::Dexterity => self.dexterity.clone(),
            Ability::Constitution => self.constitution.clone(),
            Ability::Intelligence => self.intelligence.clone(),
            Ability::Wisdom => self.wisdom.clone(),
            Ability::Charisma => self.charisma.clone(),
        }
    }

    pub fn view<'a>(
        &mut self,
        ability_scores: &ModifiedAbilityScores,
        proficiencies: Vec<Proficiency>,
        class: Classes,
    ) -> Column<'a, Message> {
        let proficiencies: HashMap<String, isize> = proficiencies
            .into_iter()
            .map(|p| {
                (
                    p.name().to_lowercase(),
                    p.proficiency_type().modifier(class.clone()),
                )
            })
            .collect();

        let abilities = vec![
            Ability::Strength,
            Ability::Dexterity,
            Ability::Constitution,
            Ability::Intelligence,
            Ability::Wisdom,
            Ability::Charisma,
        ];
        let mut column = Column::new().push(Row::new().push(Text::new("Saving Throws").size(24)));
        for ability in abilities {
            column = column.push(
                self.get(ability.clone()).view(
                    ability.clone().to_string(),
                    ability_scores.get(ability.clone()),
                    proficiencies
                        .get(ability.to_string().to_lowercase().as_str())
                        .unwrap_or(&0)
                        .clone(),
                ),
            )
        }
        column
    }
}

impl SavingThrow {
    pub fn view<'a>(
        self,
        name: String,
        ability_score: ModifiedAbilityScore,
        proficiency_modifier: isize,
    ) -> Row<'a, Message> {
        let modifier = self.modifier(ability_score, proficiency_modifier);
        Row::new()
            .width(Length::Fill)
            .spacing(2)
            .push(Text::new(name).size(16).width(Length::FillPortion(1)))
            .push(
                Column::new()
                    .push(modifier.view())
                    .width(Length::FillPortion(1)),
            )
    }

    pub fn modifier(
        self,
        ability_score: ModifiedAbilityScore,
        proficiency_modifier: isize,
    ) -> Rollable {
        let mut rollable = ability_score.roll();

        rollable.add_bonus(proficiency_modifier);
        rollable.add_bonus(self.additional_modifiers.values().sum::<isize>());
        rollable
    }
}
