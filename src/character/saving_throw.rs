use crate::character::ability_score::{Ability, AbilityScore, AbilityScores};
use crate::character::Message;
use iced::{Column, Length, Row, Text};
use serde::{Deserialize, Serialize};

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

    pub fn view(&mut self, ability_scores: &AbilityScores) -> Column<Message> {
        Column::new()
            .push(Row::new().push(Text::new("Saving Throws").size(24)))
            .push(
                self.strength
                    .view("Strength", ability_scores.get(Ability::Strength)),
            )
            .push(
                self.dexterity
                    .view("Dexterity", ability_scores.get(Ability::Dexterity)),
            )
            .push(
                self.constitution
                    .view("Constitution", ability_scores.get(Ability::Constitution)),
            )
            .push(
                self.intelligence
                    .view("Intelligence", ability_scores.get(Ability::Intelligence)),
            )
            .push(
                self.wisdom
                    .view("Wisdom", ability_scores.get(Ability::Wisdom)),
            )
            .push(
                self.charisma
                    .view("Charisma", ability_scores.get(Ability::Charisma)),
            )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SavingThrow {
    proficiency: isize,
    additional_modifier: isize,
}

impl SavingThrow {
    pub fn view(&mut self, name: &str, ability_score: AbilityScore) -> Row<Message> {
        let modifier = self.modifier(ability_score);
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

    pub fn modifier(&self, ability_score: AbilityScore) -> isize {
        ability_score.modifier() + self.proficiency + self.additional_modifier
    }
}
