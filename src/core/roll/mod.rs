use crate::character::class::{Class, Classes};
use crate::core::ability_score::{Ability, AbilityScores, ModifiedAbilityScores};
use crate::core::effect::Effect;
use crate::core::feature::Feature;
use crate::core::feature_path::FeaturePath;
use crate::core::overlay::Overlay;
use crate::core::roll::rollable::Rollable;
use iced::{Column, Element, Length, Row, Text};
use serde::export::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct Roll {
    name: String,
    #[serde(default)]
    tags: HashMap<String, Vec<String>>,
    ability: Option<Ability>,
    range: Option<Range>,
    dice: Vec<Dice>,
    #[serde(default)]
    bonuses: Vec<RollBonus>,
}

impl Overlay for Roll {
    fn overlay_by(&self) -> String {
        self.name.clone()
    }

    fn overlay(&self, overlay: &Roll) -> Roll {
        let Roll {
            name,
            tags,
            ability,
            range,
            dice,
            bonuses,
        } = overlay;

        let overlay_name = name;
        let overlay_tags = tags;
        let overlay_ability = ability;
        let overlay_range = range;
        let overlay_dice = dice;
        let overlay_bonuses = bonuses;

        let Roll {
            name,
            tags,
            ability,
            range,
            dice,
            bonuses,
        } = self;

        let merged_tags = Roll::merge_tags(tags, overlay_tags);
        let mut dice = dice.clone();
        dice.extend_from_slice(overlay_dice);
        let mut bonuses = bonuses.clone();
        bonuses.extend_from_slice(overlay_bonuses);
        Roll {
            name: overlay_name.clone(),
            tags: merged_tags,
            ability: overlay_ability.clone().or_else(|| ability.clone()),
            range: overlay_range.clone().or_else(|| range.clone()),
            dice: dice,
            bonuses: bonuses,
        }
    }
}

impl Roll {
    fn merge_tags(
        tags: &HashMap<String, Vec<String>>,
        overlay_tags: &HashMap<String, Vec<String>>,
    ) -> HashMap<String, Vec<String>> {
        let mut keys = overlay_tags.keys().collect::<HashSet<&String>>();
        keys.extend(tags.keys().collect::<HashSet<&String>>());

        let mut merged_tags = HashMap::new();
        for key in keys {
            match (tags.get(key), overlay_tags.get(key)) {
                (Some(tags), Some(overlay)) => {
                    let mut tags = tags.clone();
                    tags.extend(overlay.clone());
                    tags.dedup();
                    merged_tags.insert(key.clone(), tags);
                }
                (Some(tags), None) => {
                    merged_tags.insert(key.clone(), tags.clone());
                }
                (None, Some(tags)) => {
                    merged_tags.insert(key.clone(), tags.clone());
                }
                (None, None) => {}
            }
        }

        merged_tags
    }
    pub fn name(&mut self, name: String) {
        self.name = name
    }

    pub fn ability(&mut self, ability: Ability) {
        self.ability = Some(ability);
    }
    pub fn dice(&mut self, dice: Vec<Dice>) {
        self.dice = dice;
    }

    pub fn tags(&mut self, tags: HashMap<String, Vec<String>>) {
        self.tags = Roll::merge_tags(&self.tags, &tags);
    }
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
pub enum Advantage {
    Advantage,
    Disadvantage,
}

impl Display for Advantage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Advantage::Advantage => write!(f, "Advantage"),
            Advantage::Disadvantage => write!(f, "Disadvantage"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct RollScope {
    name: Option<String>,
    path: Option<FeaturePath>,
    tags: Option<HashMap<String, Vec<String>>>,
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

    pub fn name(&mut self, name: String) {
        self.name = Some(name)
    }

    pub fn ability(&mut self, ability: Ability) {
        self.ability = Some(ability)
    }

    pub fn path(&mut self, path: FeaturePath) {
        self.path = Some(path)
    }

    pub fn tag(&mut self, tag: String, value: Vec<String>) {
        let mut tags = self.tags.clone();
        match tags {
            Some(mut tags) => {
                tags.insert(tag, value);
                self.tags = Some(tags);
            }
            None => {
                let mut map = HashMap::new();
                map.insert(tag, value);
                self.tags = Some(map)
            }
        }
    }

    pub fn tags(&mut self, tags: HashMap<String, Vec<String>>) {
        match &self.tags {
            Some(existing_tags) => self.tags = Some(Roll::merge_tags(existing_tags, &tags)),
            None => self.tags = Some(tags),
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
        tags,
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
                    tags,
                    ability,
                    range,
                    dice,
                    bonuses,
                } = roll;

                let roll_name = name;
                let roll_tags = tags;
                let roll_ability = ability;
                let roll_range = range;

                let RollScope {
                    name,
                    tags,
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

                let roll_tags_match = match tags {
                    None => true,
                    Some(scoped_tags) => scoped_tags.into_iter().all(|(tag_name, tag_values)| {
                        match roll_tags.get(tag_name) {
                            None => false,
                            Some(roll_tag_values) => {
                                tag_values.into_iter().all(|v| roll_tag_values.contains(v))
                            }
                        }
                    }),
                };

                let is_matching = path_matches
                    && roll_tags_match
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
            tags,
            ability,
            range,
            dice,
            bonuses,
        } = roll;

        let tags_text = tags
            .into_iter()
            .fold("".to_string(), |s, (tag_name, tag_values)| {
                if (s.is_empty()) {
                    format!("{}: {}", tag_name, tag_values.join(", "))
                } else {
                    format!("{}; {}: {}", s, tag_name, tag_values.join(", "))
                }
            });

        let mut row = Row::new().push(Text::new(format!("{}", name)).width(Length::FillPortion(1)));

        match ability {
            Some(ability) => {
                row = row.push(Text::new(ability.to_string()).width(Length::FillPortion(1)))
            }
            None => {}
        }

        match range {
            Some(range) => {
                row = row.push(Text::new(range.to_string()).width(Length::FillPortion(1)))
            }
            None => {}
        }

        row = row.push(
            Row::new()
                .push(rollable.view())
                .width(Length::FillPortion(1)),
        );

        let mut column = Column::new().push(row);
        column = column.push(Text::new(tags_text).size(12));

        column
    }
}
