use crate::character::class::Classes;
use crate::core::ability_score::{Ability, AbilityScores, ModifiedAbilityScores};
use crate::core::effect::Effect;
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
    types: Vec<String>,
    ability: Option<Ability>,
    range: Option<Range>,
    dice: Vec<Dice>,
    bonuses: Vec<RollBonus>,
}

impl Roll {}

fn isNoneOr<T>(option: Option<T>, compare_to: &T) -> bool
where
    T: PartialEq,
{
    match option {
        None => true,
        Some(thing) => &thing == compare_to,
    }
}

fn isNoneOrOpt<T>(option: Option<T>, compare_to: &Option<T>) -> bool
where
    T: PartialEq,
{
    option.is_none() || &option == compare_to
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum RollBonus {
    Reroll(HashSet<isize>),
    Advantage(Advantage),
    Modifier(isize),
    Roll(Roll),
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct RollScope {
    name: Option<String>,
    path: Option<FeaturePath>,
    types: Option<Vec<String>>,
    ability: Option<Ability>,
    range: Option<Range>,
}

#[derive(Debug, Clone)]
pub struct RollState {
    roll: Roll,
    external_bonuses: Vec<RollBonus>,
}

fn rollable<'a, 'b>(
    roll: &'a Roll,
    external_bonuses: &'a Vec<RollBonus>,
    ability_scores: &'b AbilityScores,
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
        }
    }

    let mut result = Rollable::from(dice, reroll, advantage_count, modifier);

    for roll in children {
        let other = rollable(&roll, &vec![], ability_scores);
        result.merge(other);
    }

    match ability {
        Some(ability) => result.add_bonus(ability_scores.get(ability.clone()).modifier()),
        None => {}
    }
    result
}

impl RollState {
    fn apply(&mut self, effect: Effect) {
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

                let (path_matches, sub_path) = match scope.path {
                    None => (true, None),
                    Some(path) => {
                        let (matches, remaining_path) = path.matches(name.clone());
                        (matches, Some(remaining_path))
                    }
                };

                let roll_type_matches = match scope.types {
                    None => true,
                    Some(scoped_types) => scoped_types
                        .into_iter()
                        .all(|scoped_type| types.contains(&scoped_type)),
                };

                let is_matching = path_matches
                    && roll_type_matches
                    && isNoneOr(scope.name, name)
                    && isNoneOrOpt(scope.ability, ability)
                    && isNoneOrOpt(scope.range, range);

                if (is_matching) {
                    external_bonuses.push(bonus)
                }
            }
            _ => {}
        }
    }

    fn view<'a, 'b, T>(&'a mut self, size: u16, ability_scores: &'b AbilityScores) -> Column<'a, T>
    where
        T: Debug + Clone + 'a,
    {
        let RollState {
            roll,
            external_bonuses,
        } = self;

        let rollable = rollable(roll, external_bonuses, ability_scores);

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
                .spacing(size)
                .push(Text::new(format!("{}", name)).size(size * 3))
                .push(Text::new(types_text).size(size * 2)),
        );

        match ability {
            Some(ability) => {
                column = column.push(Row::new().push(Text::new(ability.to_string()).size(size * 2)))
            }
            None => {}
        }

        match range {
            Some(range) => {
                column = column.push(Row::new().push(Text::new(range.to_string()).size(size)))
            }
            None => {}
        }

        column = column.push(Row::new().push(rollable.view()));

        column
    }
}
