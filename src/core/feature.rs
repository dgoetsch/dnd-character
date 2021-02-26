use super::feature_path::FeaturePath;
use crate::character::class::Classes;
use crate::core::ability_score::AbilityScores;
use crate::core::effect::{Effect, EffectState, EffectsState};
use crate::core::overlay::{overlay_all, Overlay};
use crate::core::roll::{Roll, RollScope, RollState};
use crate::core::slot::{FromSlotCommand, Slot, SlotCommand, SlotState};
use iced::futures::StreamExt;
use iced::{button, Button, Column, Element, Length, Row, Text};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Default)]
pub struct FeaturesState {
    feature_state: Vec<FeatureState>,
}

#[derive(Debug, Clone, Default)]
pub struct FeatureState {
    feature: Feature,
    overlayed_feature: Feature,
    slot_state: Option<SlotState>,
    children: Vec<FeatureState>,
    effects_state: EffectsState,
    rolls_state: Vec<RollState>,
}

type IsDirty = bool;

#[derive(Debug, Clone)]
pub enum FeatureMessage {
    Slot(FeaturePath, SlotCommand),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Feature {
    name: String,
    description: Option<String>,
    slot: Option<Slot>,
    #[serde(default)]
    children: Vec<Feature>,
    show_reset_chidren: Option<bool>,
    child_display_orientation: Option<DisplayOrientation>,
    #[serde(default)]
    effects: Vec<Effect>,
    #[serde(default)]
    rolls: Vec<Roll>,
    #[serde(default)]
    templates: Vec<String>,
}

impl Overlay for Feature {
    fn overlay_by(&self) -> String {
        self.name.clone()
    }

    fn overlay(&self, overlay: &Self) -> Self {
        let Feature {
            name,
            description,
            slot,
            children,
            show_reset_chidren,
            child_display_orientation,
            effects,
            rolls,
            templates,
        } = overlay;
        let overlay_name = name;
        let overlay_descripion = description;
        let overlay_slot = slot;
        let overlay_children = children;
        let overlay_show_reset_children = show_reset_chidren;
        let overlay_child_display_orientation = child_display_orientation;
        let overlay_effects = effects;
        let overlay_rolls = rolls;
        let overlay_templates = templates;
        let Feature {
            name,
            description,
            slot,
            children,
            show_reset_chidren,
            child_display_orientation,
            effects,
            rolls,
            templates,
        } = self;

        let mut effects = effects.clone();
        effects.extend_from_slice(overlay_effects);

        let mut templates = templates.clone();
        templates.extend_from_slice(overlay_templates);
        templates.dedup();

        Feature {
            name: Some(overlay_name.clone())
                .filter(|n| !name.is_empty())
                .unwrap_or(name.clone()),
            description: overlay_descripion.clone().or_else(|| description.clone()),
            slot: overlay_slot.clone().or_else(|| slot.clone()),
            children: overlay_all(children, overlay_children),
            show_reset_chidren: overlay_show_reset_children
                .clone()
                .or_else(|| show_reset_chidren.clone()),
            child_display_orientation: overlay_child_display_orientation
                .clone()
                .or_else(|| child_display_orientation.clone()),
            effects: effects,
            rolls: overlay_all(rolls, overlay_rolls),
            templates: templates,
        }
    }
}

impl Feature {
    pub fn matches(&self, path: FeaturePath) -> (bool, FeaturePath) {
        path.matches(self.name.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisplayOrientation {
    Columns,
    Rows,
}

impl Default for DisplayOrientation {
    fn default() -> DisplayOrientation {
        DisplayOrientation::Rows
    }
}

impl FeaturesState {
    pub fn effects(&self) -> Vec<Effect> {
        let FeaturesState { feature_state } = self;
        let mut result = vec![];
        for state in feature_state {
            result.extend(state.effects())
        }
        result
    }

    pub fn apply_effects<'a, 'b>(&'a mut self, effects: &'b Vec<Effect>) {
        let FeaturesState { feature_state } = self;
        for state in feature_state {
            state.apply_effects(effects)
        }
    }
    pub fn is_empty(&self) -> bool {
        self.feature_state.is_empty()
    }

    pub fn persistable(&self) -> Vec<Feature> {
        self.feature_state
            .clone()
            .into_iter()
            .map(|f| f.persistable())
            .collect()
    }

    pub fn from(
        features: Vec<Feature>,
        feature_templates: &HashMap<String, Feature>,
    ) -> FeaturesState {
        FeaturesState {
            feature_state: features
                .into_iter()
                .map(|f| FeatureState::from(f, feature_templates))
                .collect(),
        }
    }

    pub fn update(&mut self, message: FeatureMessage) -> IsDirty {
        let FeaturesState { feature_state } = self;
        let mut dirty = false;
        for state in feature_state {
            dirty = state.update(message.clone()) || dirty;
        }
        dirty
    }

    pub fn view<'a, 'b, 'c, T, F>(
        &'a mut self,
        root_path: FeaturePath,
        ability_scores: &'b AbilityScores,
        classes: &'c Classes,
        f: &'a F,
    ) -> Column<'a, T>
    where
        T: Debug + Clone + 'a,
        F: Fn(FeatureMessage) -> T + 'a + Copy,
    {
        let mut column = Column::new().padding(2).spacing(8);

        let FeaturesState { feature_state } = self;

        for state in feature_state {
            column = column.push(state.view(root_path.clone(), ability_scores, classes, f));
        }

        column
    }
}

impl FeatureState {
    pub fn effects(&self) -> Vec<Effect> {
        let FeatureState {
            effects_state,
            children,
            ..
        } = self;
        let mut effects = vec![];
        effects.extend(effects_state.effect());
        for child in children {
            effects.extend(child.effects())
        }
        effects
    }

    pub fn apply_effects<'a, 'b>(&'a mut self, effects: &'b Vec<Effect>) {
        for effect in effects {
            self.apply_effect(effect)
        }
    }

    pub fn apply_effect<'a, 'b>(&'a mut self, effect: &'b Effect) {
        let FeatureState {
            feature,
            rolls_state,
            children,
            ..
        } = self;

        match effect.clone() {
            Effect::Roll { bonus, scope } => {
                let (matches, scope) = scope.matches(feature);
                let effect = Effect::Roll {
                    bonus: bonus.clone(),
                    scope: scope.clone(),
                };
                if (matches) {
                    for roll_state in rolls_state {
                        roll_state.apply(&effect)
                    }
                    for child in children {
                        child.apply_effect(&effect)
                    }
                }
            }
            _ => {}
        }
    }

    pub fn persistable(&self) -> Feature {
        let FeatureState {
            feature,
            slot_state,
            children,
            effects_state,
            rolls_state,
            overlayed_feature,
        } = self;
        let mut feature = feature.clone();
        feature.children = vec![];
        for child in children {
            feature.children.push(child.persistable())
        }

        match slot_state {
            Some(slot_state) => feature.slot = Some(slot_state.persistable()),
            None => {}
        }

        // feature.effects = effects_state.persistable();

        // feature.rolls = vec![];
        // for roll in rolls_state {
        //     feature.rolls.push(roll.persistable())
        // }

        feature
    }

    pub fn from(feature: Feature, feature_templates: &HashMap<String, Feature>) -> FeatureState {
        let original_feature = feature.clone();
        let overlayed_feature =
            feature
                .templates
                .clone()
                .iter()
                .fold(feature, |overlay, template_name| {
                    feature_templates
                        .get(template_name)
                        .map(|template| template.overlay(&overlay))
                        .unwrap_or(overlay)
                });

        let overlayed_feature = feature_templates
            .get(&overlayed_feature.name)
            .map(|template| template.overlay(&overlayed_feature))
            .unwrap_or(overlayed_feature);

        let slot_state = overlayed_feature.slot.clone().map(SlotState::from);

        FeatureState {
            feature: original_feature,
            overlayed_feature: overlayed_feature.clone(),
            children: overlayed_feature
                .children
                .clone()
                .into_iter()
                .map(|f| FeatureState::from(f, feature_templates))
                .collect(),
            slot_state,
            effects_state: EffectsState::from(overlayed_feature.effects.clone()),
            rolls_state: overlayed_feature
                .rolls
                .clone()
                .into_iter()
                .map(RollState::from)
                .collect(),
        }
    }

    pub fn update(&mut self, message: FeatureMessage) -> IsDirty {
        match message {
            FeatureMessage::Slot(path, command) => {
                self.apply_all(&vec![(path, &|feature_state: &mut FeatureState| {
                    let slot = &mut feature_state.slot_state;
                    match slot {
                        Some(slot) => slot.update(command.clone()),
                        None => false,
                    }
                })])
            }
        }
    }

    fn apply_all<F>(&mut self, actions: &Vec<(FeaturePath, &F)>) -> IsDirty
    where
        F: Fn(&mut FeatureState) -> IsDirty,
    {
        let matching_paths = actions
            .clone()
            .into_iter()
            .map(|(path, f)| (path.matches(self.feature.name.clone()), f))
            .filter(|((matched, _), _)| *matched)
            .map(|((_, remaining), f)| (remaining, f))
            .collect::<Vec<(FeaturePath, &F)>>();

        if !matching_paths.is_empty() {
            let (matched_self, mut apply_to_children): (
                Vec<(FeaturePath, &F)>,
                Vec<(FeaturePath, &F)>,
            ) = matching_paths.into_iter().partition(|(p, _)| p.is_empty());

            let mut dirty_self = false;
            for (_, f) in matched_self.clone() {
                dirty_self = f(self) || dirty_self;
            }
            let mut dirty_children = false;

            let matches_self_and_children = matched_self
                .clone()
                .into_iter()
                .filter(|(path, f)| path.include_children())
                .collect::<Vec<(FeaturePath, &F)>>();

            apply_to_children.extend(matches_self_and_children);
            if (!apply_to_children.is_empty()) {
                let FeatureState {
                    feature,
                    overlayed_feature,
                    slot_state,
                    children,
                    effects_state,
                    rolls_state,
                } = self;
                for child in children {
                    dirty_children = child.apply_all(&apply_to_children) || dirty_children;
                }
            }

            dirty_self || dirty_children
        } else {
            false
        }
    }

    fn apply<F>(&mut self, path: &FeaturePath, f: &F) -> IsDirty
    where
        F: Fn(&mut FeatureState) -> IsDirty,
    {
        let FeatureState {
            feature,
            overlayed_feature,
            slot_state,
            children,
            effects_state,
            rolls_state,
        } = self;

        let mut path = path.clone();
        match path.matches(feature.name.clone()) {
            (true, remaining) => {
                if remaining.is_empty() {
                    let mut dirty_children = false;
                    if remaining.include_children() {
                        for child in children {
                            dirty_children = child.apply(&remaining, f) || dirty_children;
                        }
                    }

                    let self_dirty = f(self);
                    self_dirty || dirty_children
                } else {
                    let mut dirty_child = false;
                    for child in children {
                        dirty_child = child.apply(&remaining, f) || dirty_child;
                    }
                    dirty_child
                }
            }
            (false, remaining) => false,
        }
    }

    pub fn view<'a, 'b, 'c, T, F>(
        &'a mut self,
        parent_path: FeaturePath,
        ability_scores: &'b AbilityScores,
        classes: &'c Classes,
        f: &'a F,
    ) -> Column<'a, T>
    where
        T: Debug + Clone + 'a,
        F: Fn(FeatureMessage) -> T + 'a + Copy,
    {
        let FeatureState {
            feature,
            overlayed_feature,
            slot_state,
            children,
            effects_state,
            rolls_state,
        } = self;

        let this_path = parent_path.with_child(feature.name.clone());

        let mut child_elements = vec![];
        if !children.is_empty() {
            for child in children {
                child_elements.push(
                    child
                        .view(this_path.clone(), ability_scores, classes, f)
                        .padding(4),
                )
            }
        }

        let Feature {
            name,
            description,
            slot,
            children,
            show_reset_chidren,
            child_display_orientation,
            effects,
            rolls,
            templates,
        } = feature;
        let slot_path = this_path.clone();
        let mut header_row: Row<'a, T> = Row::new()
            .spacing(20)
            .push(Text::new(name.clone()).size(24));

        header_row = header_row.push(FeatureState::slot_view(slot_state, &slot_path, f));

        let mut column = Column::new().push(header_row);

        column = column.push(Row::new().push(Text::new(format!("With {}", templates.join(", ")))));
        match description {
            Some(description) => column = column.push(Text::new(description.clone()).size(16)),
            None => {}
        }

        if !effects_state.is_empty() {
            column = column.push(effects_state.view().padding(2))
        }

        let display_orientation = child_display_orientation
            .as_ref()
            .unwrap_or(&DisplayOrientation::Rows)
            .clone();

        if (!rolls_state.is_empty()) {
            column = column.push(FeatureState::rolls_view(
                rolls_state,
                ability_scores,
                classes,
            ))
        }

        let child_element: Element<T> = match display_orientation {
            DisplayOrientation::Columns => child_elements
                .into_iter()
                .fold(Row::new(), |row, child| row.push(child))
                .into(),
            DisplayOrientation::Rows => child_elements
                .into_iter()
                .fold(Column::new(), |column, child| column.push(child))
                .into(),
        };

        column = column.push(child_element);

        column.width(Length::FillPortion(1))
    }

    fn rolls_view<'a, 'b, 'c, T>(
        rolls_states: &'a mut Vec<RollState>,
        ability_scores: &'b AbilityScores,
        classes: &'c Classes,
    ) -> Column<'a, T>
    where
        T: Debug + Clone + 'a,
    {
        let mut column = Column::new();
        for roll_state in rolls_states {
            column = column.push(Row::new().push(roll_state.view(ability_scores, classes)))
        }

        column
    }

    fn slot_view<'a, 'b, T, F>(
        slot_state: &'a mut Option<SlotState>,
        slot_path: &FeaturePath,
        f: &'b F,
    ) -> Column<'a, T>
    where
        T: Debug + Clone + 'a,
        F: Fn(FeatureMessage) -> T + 'b,
    {
        match slot_state {
            Some(slot_state) => slot_state
                .view(&|command: SlotCommand| f(FeatureMessage::Slot(slot_path.clone(), command))),
            _ => Column::new(),
        }
    }
}
#[cfg(test)]
mod test {
    use crate::core::ability_score::Ability;
    use crate::core::effect::Effect;
    use crate::core::feature::Feature;
    use crate::core::feature_path::FeaturePath;
    use crate::core::roll::{Dice, Roll, RollBonus, RollScope};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    fn emptyFeature() -> Feature {
        Feature::default()
    }
    fn saving_throw_proficiency(ability: Ability) -> (String, Feature) {
        let mut feature = Feature::default();
        let mut scope = RollScope::default();
        scope.path(FeaturePath::of(vec![
            "Saving Throws".to_string(),
            ability.to_string(),
        ]));
        feature.name = format!("{} Saving Throw Proficiency", ability);
        feature.effects = vec![Effect::Roll {
            bonus: RollBonus::Proficiency,
            scope: scope,
        }];

        (feature.name.clone(), feature)
    }

    fn proficiency_on_tag(name: String, tag: String, values: Vec<String>) -> (String, Feature) {
        let mut feature = Feature::default();
        let mut scope = RollScope::default();
        scope.tag(tag, values);
        feature.name = name;
        feature.effects = vec![Effect::Roll {
            bonus: RollBonus::Proficiency,
            scope: scope,
        }];
        (feature.name.clone(), feature)
    }

    fn proficiency_on_tags(name: String, tags: HashMap<String, Vec<String>>) -> (String, Feature) {
        let mut feature = Feature::default();
        let mut scope = RollScope::default();
        scope.tags(tags);
        feature.name = name;
        feature.effects = vec![Effect::Roll {
            bonus: RollBonus::Proficiency,
            scope: scope,
        }];
        (feature.name.clone(), feature)
    }
    fn weapon_proficiency(weapon_name: String) -> (String, Feature) {
        proficiency_on_tags(
            format!("{} Proficiency", weapon_name),
            vec![
                ("weapon".to_string(), vec![weapon_name]),
                ("type".to_string(), vec!["Attack".to_string()]),
            ]
            .into_iter()
            .collect::<HashMap<String, Vec<String>>>(),
        )
    }

    fn weapon_class_proficiency(weapon_class: String) -> (String, Feature) {
        proficiency_on_tags(
            format!("{} Proficiency", weapon_class),
            vec![
                ("weapon_class".to_string(), vec![weapon_class]),
                ("type".to_string(), vec!["Attack".to_string()]),
            ]
            .into_iter()
            .collect(),
        )
    }

    fn armor_proficiency(armor_class: String) -> (String, Feature) {
        proficiency_on_tag(
            format!("{} Armor Proficiency", armor_class),
            "armor_class".to_string(),
            vec![armor_class],
        )
    }

    fn skill_proficiency(skill: String) -> (String, Feature) {
        let mut feature = Feature::default();
        let mut scope = RollScope::default();
        scope.path(FeaturePath::of(vec![
            "Skills".to_string(),
            skill.to_string(),
        ]));
        feature.name = format!("{} Proficiency", skill);
        feature.effects = vec![Effect::Roll {
            bonus: RollBonus::Proficiency,
            scope: scope,
        }];

        (feature.name.clone(), feature)
    }

    fn skills_feature(skills: Vec<Skill>) -> (String, Feature) {
        let mut feature = Feature::default();

        feature.name = format!("Skills");
        feature.rolls = skills
            .into_iter()
            .map(|skill| {
                let mut roll = Roll::default();
                roll.name(skill.name.clone());
                roll.ability(skill.ability);
                roll.dice(vec![Dice::new(1, 20)]);
                roll
            })
            .collect();

        (feature.name.clone(), feature)
    }

    fn saving_throws_feature() -> (String, Feature) {
        let mut feature = Feature::default();

        feature.name = format!("Saving Throws");
        feature.rolls = vec![
            Ability::Strength,
            Ability::Dexterity,
            Ability::Constitution,
            Ability::Intelligence,
            Ability::Wisdom,
            Ability::Charisma,
        ]
        .into_iter()
        .map(|ability| {
            let mut roll = Roll::default();
            roll.name(ability.to_string());
            roll.ability(ability);
            roll.dice(vec![Dice::new(1, 20)]);
            roll
        })
        .collect();

        (feature.name.clone(), feature)
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Skill {
        name: String,
        ability: Ability,
    }

    #[test]
    fn generate_saving_throws() {
        let skills = serde_json::from_str::<Vec<Skill>>(
            &std::fs::read_to_string("./.store/skills.json").unwrap(),
        )
        .unwrap();

        let skillProficiencies = skills
            .clone()
            .iter()
            .map(|skill| skill.name.clone())
            .map(skill_proficiency)
            .collect::<HashMap<String, Feature>>();

        println!("{:?}", skillProficiencies);
        let mut proficiencies = vec![
            saving_throw_proficiency(Ability::Strength),
            saving_throw_proficiency(Ability::Dexterity),
            saving_throw_proficiency(Ability::Constitution),
            saving_throw_proficiency(Ability::Intelligence),
            saving_throw_proficiency(Ability::Wisdom),
            saving_throw_proficiency(Ability::Charisma),
            weapon_proficiency("Dagger".to_string()),
            weapon_proficiency("Dart".to_string()),
            weapon_proficiency("Sling".to_string()),
            weapon_proficiency("Quarterstaff".to_string()),
            weapon_proficiency("Light Crossbow".to_string()),
            weapon_class_proficiency("Simple Weapon".to_string()),
            weapon_class_proficiency("Martial Weapon".to_string()),
            armor_proficiency("All".to_string()),
            armor_proficiency("Light".to_string()),
            armor_proficiency("Medium".to_string()),
            armor_proficiency("Heavy".to_string()),
            skills_feature(skills),
            saving_throws_feature(),
        ]
        .into_iter()
        .collect::<HashMap<String, Feature>>();

        proficiencies.extend(skillProficiencies);
        println!(
            "{}",
            serde_json::to_string(&proficiencies).unwrap_or("".to_string())
        );
    }
}
