use crate::core::effect::{AbilityScoreBonus, CheckBonus, CheckRoll, Effect};
use iced::{Column, HorizontalAlignment, Length, Row, Text, VerticalAlignment};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AbilityScores {
    strength: AbilityScore,
    dexterity: AbilityScore,
    constitution: AbilityScore,
    intelligence: AbilityScore,
    wisdom: AbilityScore,
    charisma: AbilityScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Display for Ability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AbilityScores {
    pub fn apply_all(self, modifiers: &Vec<Effect>) -> AbilityScores {
        let mut scores = self;
        for modifier in modifiers {
            scores = scores.apply(modifier.clone());
        }
        scores
    }

    pub fn apply(self, modifier: Effect) -> AbilityScores {
        match modifier {
            Effect::Ability { ability, bonus } => {
                let mut score = self.get(ability.clone());
                score.value_modifiers.push(bonus);
                self.with(ability, score)
            }
            Effect::Check { bonus, roll } => match roll {
                CheckRoll::Ability(ability) => {
                    let mut score = self.get(ability.clone());
                    score.bonus_modifiers.push(bonus);
                    self.with(ability, score)
                }
                _ => self,
            },
            _ => self,
        }
    }
    pub fn default() -> AbilityScores {
        AbilityScores {
            strength: AbilityScore::default(),
            dexterity: AbilityScore::default(),
            constitution: AbilityScore::default(),
            intelligence: AbilityScore::default(),
            wisdom: AbilityScore::default(),
            charisma: AbilityScore::default(),
        }
    }

    pub fn get(&self, ability: Ability) -> AbilityScore {
        match ability {
            Ability::Strength => self.strength.clone(),
            Ability::Dexterity => self.dexterity.clone(),
            Ability::Constitution => self.constitution.clone(),
            Ability::Intelligence => self.intelligence.clone(),
            Ability::Wisdom => self.wisdom.clone(),
            Ability::Charisma => self.charisma.clone(),
        }
    }

    pub fn with(&self, ability: Ability, score: AbilityScore) -> AbilityScores {
        let mut new = self.clone();
        match ability {
            Ability::Strength => new.strength = score,
            Ability::Dexterity => new.dexterity = score,
            Ability::Constitution => new.constitution = score,
            Ability::Intelligence => new.intelligence = score,
            Ability::Wisdom => new.wisdom = score,
            Ability::Charisma => new.charisma = score,
        };
        new
    }

    pub fn view<'a, T: Debug + Clone + 'a>(&'a mut self) -> Column<'a, T> {
        Column::new()
            .push(Row::new().push(Text::new("Ability Scores").size(24)))
            .push(self.strength.view("STR"))
            .push(self.dexterity.view("DEX"))
            .push(self.constitution.view("CON"))
            .push(self.intelligence.view("INT"))
            .push(self.wisdom.view("WIS"))
            .push(self.charisma.view("CHA"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityScore {
    value: isize,
    value_modifiers: Vec<AbilityScoreBonus>,
    bonus_modifiers: Vec<CheckBonus>,
}

impl Default for AbilityScore {
    fn default() -> Self {
        AbilityScore::of(10)
    }
}

impl AbilityScore {
    pub fn view<T: Debug + Clone>(&mut self, name: &str) -> Row<T> {
        let modifier = if self.modifier() < 0 {
            format!("({})", self.modifier())
        } else {
            format!("(+{})", self.modifier())
        };
        Row::new()
            .width(Length::Fill)
            .spacing(4)
            .push(
                Text::new(name)
                    .size(16)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Bottom)
                    .width(Length::FillPortion(1)),
            )
            .push(
                Text::new(self.value.to_string())
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Bottom)
                    .size(24)
                    .width(Length::FillPortion(1)),
            )
            .push(
                Text::new(modifier)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Bottom)
                    .size(24)
                    .width(Length::FillPortion(1)),
            )
    }

    pub fn of(value: isize) -> AbilityScore {
        AbilityScore {
            value,
            value_modifiers: vec![],
            bonus_modifiers: vec![],
        }
    }
    pub fn modifier(&self) -> isize {
        if self.value < 10 {
            (self.value - 11) / 2
        } else {
            (self.value - 10) / 2
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::ability_score::AbilityScore;

    fn assert_modfier(value: isize, modifier: isize) {
        assert_eq!(
            AbilityScore::of(value).modifier(),
            modifier,
            "Expected modifier for ability score {} to be {}",
            value,
            modifier
        )
    }
    #[test]
    fn test_modifier() {
        vec![
            (4, -3),
            (5, -3),
            (6, -2),
            (7, -2),
            (8, -1),
            (9, -1),
            (10, 0),
            (11, 0),
            (12, 1),
            (13, 1),
            (14, 2),
            (15, 2),
            (16, 3),
            (17, 3),
            (18, 4),
            (19, 4),
            (20, 5),
            (21, 5),
            (22, 6),
            (23, 6),
        ]
        .into_iter()
        .for_each(|(value, modifier)| assert_modfier(value, modifier))
    }
}