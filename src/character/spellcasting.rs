use crate::character::ability_score::{Ability, AbilityScore, AbilityScores};
use crate::character::class::{Class, Classes};
use crate::character::Message;
use crate::util::{format_modifier, two_column_row};
use iced::{Column, Row, Text};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn view<'a>(
    spellcasting: Vec<Spellcasting>,
    class: &Classes,
    ability_scores: AbilityScores,
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
    fn modifier(&self, proficiency_modifier: isize, ability: &AbilityScore) -> isize {
        proficiency_modifier
            + ability.modifier()
            + self.additional_modifiers.values().sum::<isize>()
    }

    fn save_dc(&self, proficiency_modifier: isize, ability: &AbilityScore) -> isize {
        proficiency_modifier
            + ability.modifier()
            + 8
            + self.additional_save_modifiers.values().sum::<isize>()
    }

    pub fn view<'a>(
        self,
        proficiency_modifier: isize,
        ability: AbilityScore,
    ) -> Column<'a, Message> {
        Column::new()
            .push(Row::new().push(Text::new(format!("{} spellcasting", self.class)).size(24)))
            .push(two_column_row(
                Text::new("Modifier").size(16),
                Text::new(format_modifier(
                    self.modifier(proficiency_modifier, &ability),
                ))
                .size(16),
            ))
            .push(two_column_row(
                Text::new("Spell Save DC").size(16),
                Text::new(self.save_dc(proficiency_modifier, &ability).to_string()).size(16),
            ))
    }
}
