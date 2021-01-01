use crate::character::ability_score::{Ability, AbilityScores};
use crate::character::class::Classes;
use crate::character::proficiencies::{Proficiency, ProficiencyType};
use crate::character::Message;
use crate::resources::Skill;
use crate::util::{format_modifier, three_column_row};
use iced::{Column, HorizontalAlignment, Length, Row, Text};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SkillState {
    skills: Vec<Skill>,
    proficiencies: Vec<Proficiency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillProficiency {
    name: String,
    proficiency_type: ProficiencyType,
}

impl SkillProficiency {
    fn of(skill: Skill, proficiency_type: ProficiencyType) -> SkillProficiency {
        SkillProficiency {
            name: skill.name(),
            proficiency_type,
        }
    }
}

pub fn view<'a>(
    skills: Vec<Skill>,
    proficiencies: Vec<Proficiency>,
    class: Classes,
    ability_scores: AbilityScores,
) -> Column<'a, Message> {
    let mut column = Column::new().push(Row::new().push(Text::new("Skills")));

    let proficiencies = proficiencies
        .into_iter()
        .map(|p| (p.name(), p.proficiency_type()))
        .collect::<HashMap<String, ProficiencyType>>();
    for skill in skills {
        let proficiency = proficiencies
            .get(skill.name().as_str())
            .unwrap_or(&ProficiencyType::None)
            .clone();
        let name = skill.name();
        let ability = format!("{:?}", skill.ability());
        let modifier = modifier(skill, proficiency, class.clone(), ability_scores.clone());
        let row = three_column_row(
            Text::new(name).size(16),
            Text::new(ability).size(16),
            Text::new(format_modifier(modifier)).size(20),
        )
        .padding(2);

        column = column.push(row);
    }
    column
}

fn modifier(
    skill: Skill,
    proficiency: ProficiencyType,
    class: Classes,
    ability_scores: AbilityScores,
) -> isize {
    proficiency.modifier(class) + ability_scores.get(skill.ability()).modifier()
}

#[cfg(test)]
mod test {
    use super::{Skill, SkillProficiency};
    use crate::character::ability_score::{Ability, AbilityScore, AbilityScores};
    use crate::character::class::{Class, Classes};
    use crate::character::proficiencies::ProficiencyType;
    use Ability::*;

    fn s(name: &str, ability: Ability) -> Skill {
        Skill::of(name, ability)
    }

    fn class_leveled(level: isize) -> Classes {
        Classes::from(vec![Class::new("test".to_string(), level)])
    }

    fn ability_scores(skill: Skill, score: isize) -> AbilityScores {
        AbilityScores::default().with(skill.ability(), AbilityScore::of(score))
    }

    fn ability_score_modifier(score: isize) -> isize {
        AbilityScore::of(score).modifier()
    }

    fn assert_modifier(skill: Skill) {
        println!("{}", 1 / 2);
        println!("{}", (1 / 2));
        (0..20).into_iter().for_each(|level| {
            let class = class_leveled(level);
            (6..20).into_iter().for_each(|score| {
                let ability_scores = ability_scores(skill.clone(), score);
                let expected_ability_score_modifier = ability_score_modifier(score);

                let no_proficiency = ProficiencyType::None;
                let no_proficiency_modifier = super::modifier(skill.clone(), no_proficiency, class.clone(), ability_scores.clone());

                assert_eq!(no_proficiency_modifier, expected_ability_score_modifier,
                           "Expected {} {} to be {} composed of {}({:?} {}) +0 (no proficiency level {})",
                           skill.name().clone(), no_proficiency_modifier, expected_ability_score_modifier, expected_ability_score_modifier, skill.ability().clone(), score, level);

                let half_proficiency = ProficiencyType::Half;
                let half_proficiency_modifier = super::modifier(skill.clone(), half_proficiency, class.clone(), ability_scores.clone());
                assert_eq!(half_proficiency_modifier, ability_score_modifier(score) + class.proficiency_bonus() / 2,
                           "Expected {} {} to be {} composed of {}({:?} {}) +{} (half proficiency level {})",
                           skill.name().clone(), half_proficiency_modifier, expected_ability_score_modifier, expected_ability_score_modifier, skill.ability().clone(), score, ProficiencyType::Half.modifier(class.clone()), level);

                let proficiency = ProficiencyType::Full;
                let proficiency_modifier = super::modifier(skill.clone(), proficiency, class.clone(), ability_scores.clone());
                assert_eq!(proficiency_modifier, ability_score_modifier(score) + class.proficiency_bonus(),
                           "Expected {} {} to be {} composed of {}({:?} {}) +{} (full proficiency level {})",
                           skill.name().clone(), proficiency_modifier, expected_ability_score_modifier, expected_ability_score_modifier, skill.ability().clone(), score, ProficiencyType::Full.modifier(class.clone()), level);
            })
        })
    }

    #[test]
    fn create_data() {
        let skills = vec![
            s("Acrobatics", Dexterity),
            s("Animal Handling", Wisdom),
            s("Arcana", Intelligence),
            s("Athletics", Strength),
            s("Deception", Charisma),
            s("History", Intelligence),
            s("Insight", Wisdom),
            s("Intimidation", Charisma),
            s("Investigation", Intelligence),
            s("Medicine", Wisdom),
            s("Nature", Intelligence),
            s("Perception", Wisdom),
            s("Performance", Charisma),
            s("Persuasion", Charisma),
            s("Religion", Intelligence),
            s("Sleight of Hand", Dexterity),
            s("Stealth", Dexterity),
            s("Survival", Wisdom),
        ];
        // let json = serde_json::to_string_pretty(&skills).unwrap_or("".to_string());
        // println!("{}", json);
        skills.into_iter().for_each(assert_modifier)
    }
}
