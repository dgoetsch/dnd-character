use crate::core::effect::Effect;
use crate::core::roll::{AbilityScoreBonus, Advantage, CheckBonus, CheckRoll, CheckRollType};
use crate::core::Dice;
use crate::util::format_modifier;
use iced::{Column, Element, HorizontalAlignment, Length, Row, Text, VerticalAlignment};
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

#[derive(Debug, Clone)]
pub struct ModifiedAbilityScores {
    strength: ModifiedAbilityScore,
    dexterity: ModifiedAbilityScore,
    constitution: ModifiedAbilityScore,
    intelligence: ModifiedAbilityScore,
    wisdom: ModifiedAbilityScore,
    charisma: ModifiedAbilityScore,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
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
    pub fn to_state(self) -> AbilityScoresState {
        AbilityScoresState {
            ability_scores: self.clone(),
            strength: self.strength.to_state(),
            dexterity: self.dexterity.to_state(),
            constitution: self.constitution.to_state(),
            intelligence: self.intelligence.to_state(),
            wisdom: self.wisdom.to_state(),
            charisma: self.charisma.to_state(),
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
}

impl ModifiedAbilityScores {
    pub fn get(&self, ability: Ability) -> ModifiedAbilityScore {
        match ability {
            Ability::Strength => self.strength.clone(),
            Ability::Dexterity => self.dexterity.clone(),
            Ability::Constitution => self.constitution.clone(),
            Ability::Intelligence => self.intelligence.clone(),
            Ability::Wisdom => self.wisdom.clone(),
            Ability::Charisma => self.charisma.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AbilityScoresState {
    ability_scores: AbilityScores,
    strength: AbilityScoreState,
    dexterity: AbilityScoreState,
    constitution: AbilityScoreState,
    intelligence: AbilityScoreState,
    wisdom: AbilityScoreState,
    charisma: AbilityScoreState,
}

impl AbilityScoresState {
    pub fn modified(&self) -> ModifiedAbilityScores {
        ModifiedAbilityScores {
            strength: self.strength.modified(),
            dexterity: self.dexterity.modified(),
            constitution: self.constitution.modified(),
            intelligence: self.intelligence.modified(),
            wisdom: self.wisdom.modified(),
            charisma: self.charisma.modified(),
        }
    }

    fn reset_modifiers(&mut self) {
        self.strength.reset();
        self.dexterity.reset();
        self.constitution.reset();
        self.intelligence.reset();
        self.wisdom.reset();
        self.charisma.reset();
    }

    pub fn apply_all(&mut self, modifiers: &Vec<Effect>) {
        self.reset_modifiers();
        for modifier in modifiers {
            self.apply(modifier.clone());
        }
    }

    fn get_mut<'a>(&'a mut self, ability: Ability) -> &'a mut AbilityScoreState {
        match ability {
            Ability::Strength => &mut self.strength,
            Ability::Dexterity => &mut self.dexterity,
            Ability::Constitution => &mut self.constitution,
            Ability::Intelligence => &mut self.intelligence,
            Ability::Wisdom => &mut self.wisdom,
            Ability::Charisma => &mut self.charisma,
        }
    }

    fn apply(&mut self, modifier: Effect) {
        match modifier {
            Effect::Ability { ability, bonus } => {
                let score = self.get_mut(ability.clone());
                score.value_modifiers.push(bonus);
            }
            Effect::Check { bonus, roll } => match roll {
                CheckRollType::Ability(ability) => {
                    let mut score = self.get_mut(ability.clone());
                    score.bonus_modifiers.push(bonus);
                }
                _ => {}
            },
            _ => {}
        };
    }

    pub fn persistable(&self) -> AbilityScores {
        self.ability_scores.clone()
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

#[derive(Debug, Clone, Default)]
pub struct AbilityScoreState {
    ability_score: AbilityScore,
    value_modifiers: Vec<AbilityScoreBonus>,
    bonus_modifiers: Vec<CheckBonus>,
}

impl AbilityScoreState {
    pub fn reset(&mut self) {
        self.value_modifiers = vec![];
        self.bonus_modifiers = vec![];
    }
    pub fn view<T: Debug + Clone>(&mut self, name: &str) -> Row<T> {
        let ModifiedAbilityScore { score, modifier } = self.modified();

        let modified_score = score;

        let name_cell = Text::new(name)
            .size(16)
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Bottom)
            .width(Length::FillPortion(1));

        let score_text = if modified_score == self.ability_score {
            format!("{}", modified_score.value)
        } else {
            format!(
                "{} (base {})",
                modified_score.value, self.ability_score.value
            )
        };

        let value_cell = Text::new(score_text)
            .size(16)
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Bottom)
            .width(Length::FillPortion(1));

        let bonus_cell = modifier.with_extra_bonus(modified_score.modifier()).view();

        Row::new()
            .width(Length::Fill)
            .spacing(4)
            .push(name_cell)
            .push(value_cell)
            .push(bonus_cell)
    }

    fn modified(&self) -> ModifiedAbilityScore {
        let AbilityScoreState {
            ability_score,
            value_modifiers,
            bonus_modifiers,
        } = self;

        let mut become_value: Option<isize> = None;
        let mut score_modifiers = vec![];

        for value_modifier in value_modifiers {
            match value_modifier {
                AbilityScoreBonus::Become { value } => {
                    become_value = Some(value.clone());
                }
                AbilityScoreBonus::Modifier { modifier } => score_modifiers.push(modifier.clone()),
            }
        }

        let score_modifier_total = if score_modifiers.is_empty() || become_value.is_some() {
            0
        } else {
            score_modifiers.into_iter().sum::<isize>()
        };

        let score = match become_value {
            Some(value) => AbilityScore::of(value),
            None => AbilityScore::of(ability_score.value + score_modifier_total),
        };

        let modifier = CheckRoll::from(bonus_modifiers.clone());

        ModifiedAbilityScore { score, modifier }
    }
}
#[derive(Debug, Clone)]
pub struct ModifiedAbilityScore {
    score: AbilityScore,
    modifier: CheckRoll,
}

impl ModifiedAbilityScore {
    pub fn roll(&self) -> CheckRoll {
        self.modifier.with_extra_bonus(self.score.modifier())
    }
    pub fn score(&self) -> AbilityScore {
        self.score.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct AbilityScore {
    value: isize,
}

impl Default for AbilityScore {
    fn default() -> Self {
        AbilityScore::of(10)
    }
}

impl AbilityScore {
    pub fn to_state(self) -> AbilityScoreState {
        AbilityScoreState {
            ability_score: self,
            ..AbilityScoreState::default()
        }
    }

    pub fn of(value: isize) -> AbilityScore {
        AbilityScore { value }
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
