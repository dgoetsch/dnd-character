use crate::character::class::{Class, Classes};
use crate::character::Message;
use crate::core::ability_score::{
    Ability, AbilityScore, AbilityScores, ModifiedAbilityScore, ModifiedAbilityScores,
};
use crate::core::effect::CheckRollModifier;
use crate::util::{format_modifier, two_column_row, two_element_row};
use iced::{Column, HorizontalAlignment, Row, Text, VerticalAlignment};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    fn modifier(
        &self,
        proficiency_modifier: isize,
        ability: &ModifiedAbilityScore,
    ) -> CheckRollModifier {
        ability
            .modifier()
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
