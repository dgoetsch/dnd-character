use crate::character::ability_score::{Ability, AbilityScore, AbilityScores};
use crate::character::class::Classes;
use crate::character::proficiencies::{Proficiency, ProficiencyType};
use crate::character::Message;
use iced::{Column, Length, Row, Text};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        ability_scores: &AbilityScores,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SavingThrow {
    additional_modifiers: HashMap<String, isize>,
}

impl SavingThrow {
    pub fn view<'a>(
        self,
        name: String,
        ability_score: AbilityScore,
        proficiency_modifier: isize,
    ) -> Row<'a, Message> {
        let modifier = self.modifier(ability_score, proficiency_modifier);
        let modifier = if modifier < 0 {
            format!("{}", modifier)
        } else {
            format!("+{}", modifier)
        };

        Row::new()
            .width(Length::Fill)
            .spacing(2)
            .push(Text::new(name).size(16).width(Length::FillPortion(1)))
            .push(Text::new(modifier).size(24).width(Length::FillPortion(1)))
    }

    pub fn modifier(self, ability_score: AbilityScore, proficiency_modifier: isize) -> isize {
        ability_score.modifier()
            + proficiency_modifier
            + self.additional_modifiers.values().sum::<isize>()
    }
}
