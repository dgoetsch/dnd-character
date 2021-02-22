use super::feature_path::FeaturePath;
use crate::character::class::Classes;
use crate::core::ability_score::AbilityScores;
use crate::core::effect::{Effect, EffectState, EffectsState};
use crate::core::roll::{Roll, RollScope, RollState};
use crate::core::slot::{FromSlotCommand, Slot, SlotCommand, SlotState};
use iced::futures::StreamExt;
use iced::{button, Button, Column, Element, Length, Row, Text};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Default)]
pub struct FeaturesState {
    feature_state: Vec<FeatureState>,
}

#[derive(Debug, Clone, Default)]
pub struct FeatureState {
    feature: Feature,
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
}

impl Feature {
    pub fn new<T: Into<String>>(name: T) -> Feature {
        Feature {
            name: name.into(),
            description: None,
            slot: None,
            children: vec![],
            show_reset_chidren: None,
            child_display_orientation: None,
            effects: vec![],
            rolls: vec![],
        }
    }

    pub fn matches(&self, path: FeaturePath) -> (bool, FeaturePath) {
        path.matches(self.name.clone())
    }

    pub fn with_description<T: Into<String>>(&self, description: T) -> Feature {
        let mut new = self.clone();
        new.description = Some(description.into());
        new
    }

    pub fn with_slot(&self, current: isize, max: Option<isize>) -> Feature {
        let mut new = self.clone();
        new.slot = Some(Slot::new(current, max));
        new
    }

    pub fn with_children(&self, children: Vec<Feature>) -> Feature {
        let mut new = self.clone();
        new.children = children;
        new
    }

    pub fn add_children(&self, children: Vec<Feature>) -> Feature {
        let mut new = self.clone();
        new.children.extend(children);
        new
    }

    pub fn enable_reset_children(&self) -> Feature {
        let mut new = self.clone();
        new.show_reset_chidren = Some(true);
        new
    }

    pub fn disable_reset_children(&self) -> Feature {
        let mut new = self.clone();
        new.show_reset_chidren = Some(false);
        new
    }

    pub fn with_child_display_orientation(
        &self,
        display_orientation: DisplayOrientation,
    ) -> Feature {
        let mut new = self.clone();
        new.child_display_orientation = Some(display_orientation);
        new
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

    pub fn from(features: Vec<Feature>) -> FeaturesState {
        FeaturesState {
            feature_state: features.into_iter().map(FeatureState::from).collect(),
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

        feature.effects = effects_state.persistable();

        feature.rolls = vec![];
        for roll in rolls_state {
            feature.rolls.push(roll.persistable())
        }

        feature
    }

    pub fn from(feature: Feature) -> FeatureState {
        let slot_state = feature.slot.clone().map(SlotState::from);

        FeatureState {
            feature: feature.clone(),
            children: feature
                .children
                .into_iter()
                .map(FeatureState::from)
                .collect(),
            slot_state,
            effects_state: EffectsState::from(feature.effects.clone()),
            rolls_state: feature
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
        } = feature;
        let slot_path = this_path.clone();
        let mut header_row: Row<'a, T> = Row::new()
            .spacing(20)
            .push(Text::new(name.clone()).size(24));

        header_row = header_row.push(FeatureState::slot_view(slot_state, &slot_path, f));

        let mut column = Column::new().push(header_row);

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
