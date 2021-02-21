use crate::character::class::{Class, Classes};
use crate::core::ability_score::{Ability, AbilityScores, ModifiedAbilityScores};
use crate::core::effect::Effect;
use crate::core::feature::Feature;
use crate::core::feature_path::FeaturePath;
use crate::core::roll::check::{Advantage, CheckRoll};
use crate::core::roll::damage::DamageRoll;
use crate::core::roll::rollable::Rollable;
use iced::{Column, Element, Row, Text};
use serde::export::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub mod check;
pub mod damage;
pub mod rollable;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Dice {
    count: isize,
    sides: isize,
}

impl Dice {
    pub fn new(count: isize, sides: isize) -> Dice {
        Dice { count, sides }
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.count, self.sides)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Range {
    Melee,
    Ranged { normal: isize, long: isize },
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Range::Melee => write!(f, "Melee"),
            Range::Ranged { normal, long } => write!(f, "{} / {}", normal, long),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Roll {
    name: String,
    #[serde(default)]
    types: Vec<String>,
    ability: Option<Ability>,
    range: Option<Range>,
    dice: Vec<Dice>,
    #[serde(default)]
    bonuses: Vec<RollBonus>,
}

fn isNoneOr<'a, 'b, T>(option: &'a Option<T>, compare_to: &'b T) -> bool
where
    T: PartialEq,
{
    match option {
        None => true,
        Some(thing) => thing == compare_to,
    }
}

fn isNoneOrOpt<'a, 'b, T>(option: &'a Option<T>, compare_to: &'b Option<T>) -> bool
where
    T: PartialEq,
{
    option.is_none() || option == compare_to
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum RollBonus {
    Reroll(HashSet<isize>),
    Advantage(Advantage),
    Modifier(isize),
    Roll(Roll),
    Proficiency,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct RollScope {
    name: Option<String>,
    path: Option<FeaturePath>,
    types: Option<Vec<String>>,
    ability: Option<Ability>,
    range: Option<Range>,
}

impl RollScope {
    pub fn matches<'a, 'b>(&'a self, feature: &'b Feature) -> (bool, RollScope) {
        let mut result = self.clone();
        match result.path.clone() {
            None => (true, result),
            Some(path) => {
                let (matches, remaining) = feature.matches(path);
                if (matches) {
                    result.path = Some(remaining)
                }
                (matches, result)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RollState {
    roll: Roll,
    external_bonuses: Vec<RollBonus>,
}

fn rollable<'a, 'b, 'c>(
    roll: &'a Roll,
    external_bonuses: &'a Vec<RollBonus>,
    ability_scores: &'b AbilityScores,
    classes: &'c Classes,
) -> Rollable {
    let Roll {
        name,
        types,
        ability,
        range,
        dice,
        bonuses,
    } = roll;

    let mut dice = dice.clone();

    let mut bonuses = bonuses.clone();
    if (!external_bonuses.is_empty()) {
        bonuses.extend(external_bonuses.clone());
    }

    let mut reroll = HashSet::new();
    let mut advantage_count = 0;
    let mut modifier = 0;

    let mut children = vec![];
    for bonus in bonuses {
        match bonus {
            RollBonus::Reroll(more) => reroll.extend(more),
            RollBonus::Advantage(advantage) => match advantage {
                Advantage::Advantage => advantage_count = advantage_count + 1,
                Advantage::Disadvantage => advantage_count = advantage_count - 1,
            },
            RollBonus::Modifier(more) => modifier = more + modifier,
            RollBonus::Roll(roll) => children.push(roll),
            RollBonus::Proficiency => modifier = modifier + classes.proficiency_bonus(),
        }
    }

    let mut result = Rollable::from(dice, reroll, advantage_count, modifier);

    for roll in children {
        let other = rollable(&roll, &vec![], ability_scores, classes);
        result.merge(other);
    }

    match ability {
        Some(ability) => result.add_bonus(ability_scores.get(ability.clone()).modifier()),
        None => {}
    }
    result
}

impl RollState {
    pub fn persistable(&self) -> Roll {
        self.roll.clone()
    }

    pub fn from(roll: Roll) -> RollState {
        RollState {
            roll: roll,
            external_bonuses: vec![],
        }
    }
    pub fn apply<'a, 'b>(&'a mut self, effect: &'b Effect) {
        match effect {
            Effect::Roll { bonus, scope } => {
                let RollState {
                    roll,
                    external_bonuses,
                } = self;
                let Roll {
                    name,
                    types,
                    ability,
                    range,
                    dice,
                    bonuses,
                } = roll;

                let roll_name = name;
                let roll_types = types;
                let roll_ability = ability;
                let roll_range = range;

                let RollScope {
                    name,
                    types,
                    path,
                    ability,
                    range,
                } = scope;

                let (path_matches, sub_path) = match path {
                    None => (true, None),
                    Some(path) => {
                        let (matches, remaining_path) = path.matches(roll_name.clone());
                        (matches, Some(remaining_path))
                    }
                };

                let roll_type_matches = match types {
                    None => true,
                    Some(scoped_types) => scoped_types
                        .into_iter()
                        .all(|scoped_type| roll_types.contains(scoped_type)),
                };

                let is_matching = path_matches
                    && roll_type_matches
                    && isNoneOr(name, roll_name)
                    && isNoneOrOpt(ability, roll_ability)
                    && isNoneOrOpt(range, roll_range);

                if (is_matching) {
                    external_bonuses.push(bonus.clone())
                }
            }
            _ => {}
        }
    }

    pub fn view<'a, 'b, 'c, T>(
        &'a mut self,
        ability_scores: &'b AbilityScores,
        classes: &'c Classes,
    ) -> Column<'a, T>
    where
        T: Debug + Clone + 'a,
    {
        let RollState {
            roll,
            external_bonuses,
        } = self;

        let rollable = rollable(roll, external_bonuses, ability_scores, classes);

        let Roll {
            name,
            types,
            ability,
            range,
            dice,
            bonuses,
        } = roll;

        let types_text = types.into_iter().fold("".to_string(), |s, next_type| {
            if (s.is_empty()) {
                next_type.trim().to_string()
            } else {
                format!("{}, {}", s, next_type)
            }
        });

        let mut column = Column::new().push(
            Row::new()
                .push(Text::new(format!("{}", name)))
                .push(Text::new(types_text)),
        );

        match ability {
            Some(ability) => column = column.push(Row::new().push(Text::new(ability.to_string()))),
            None => {}
        }

        match range {
            Some(range) => column = column.push(Row::new().push(Text::new(range.to_string()))),
            None => {}
        }

        column = column.push(Row::new().push(rollable.view()));

        column
    }
}
