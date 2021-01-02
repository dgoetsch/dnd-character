use crate::core::effect::{Effect, EffectState, EffectsState};
use iced::{button, Button, Column, Element, Length, Row, Text};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub struct FeaturesState {
    feature_state: Vec<FeatureState>,
}

#[derive(Debug, Clone, Default)]
pub struct FeatureState {
    feature: Feature,
    slot_controls: Option<FeatureSlotControl>,
    children: Vec<FeatureState>,
    effects_state: EffectsState,
}

#[derive(Debug, Clone, Default)]
pub struct FeatureSlotControl {
    use_slot: button::State,
    reset: button::State,
    reset_all: button::State,
}

type FeaturePath = Vec<String>;
type IsDirty = bool;
type IncludeChildren = bool;

#[derive(Debug, Clone)]
pub enum FeatureMessage {
    Use(FeaturePath),
    Reset(FeaturePath, IncludeChildren),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeatureSlot {
    current: isize,
    max: Option<isize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Feature {
    name: String,
    description: Option<String>,
    slot: Option<FeatureSlot>,
    children: Vec<Feature>,
    show_reset_chidren: Option<bool>,
    child_display_orientation: Option<DisplayOrientation>,
    effects: Vec<Effect>,
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
        }
    }

    pub fn with_description<T: Into<String>>(&self, description: T) -> Feature {
        let mut new = self.clone();
        new.description = Some(description.into());
        new
    }

    pub fn with_slot(&self, current: isize, max: Option<isize>) -> Feature {
        let mut new = self.clone();
        new.slot = Some(FeatureSlot { current, max });
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

    pub fn view<'a, T, F>(&'a mut self, root_path: Vec<String>, f: F) -> Column<'a, T>
    where
        T: Debug + Clone + 'a,
        F: Fn(FeatureMessage) -> T + 'a,
    {
        let mut column = Column::new().padding(2).spacing(8);

        let FeaturesState { feature_state } = self;

        for state in feature_state {
            column = column.push(state.view(root_path.clone(), &f));
        }

        column
    }
}

impl FeatureState {
    pub fn persistable(&self) -> Feature {
        let FeatureState {
            feature,
            slot_controls,
            children,
            effects_state,
        } = self;
        let mut feature = feature.clone();
        feature.children = vec![];
        for child in children {
            feature.children.push(child.persistable())
        }

        feature.effects = effects_state.persistable();

        feature
    }

    pub fn from(feature: Feature) -> FeatureState {
        let slot_controls = if feature.slot.is_some() {
            Some(FeatureSlotControl::default())
        } else {
            None
        };

        FeatureState {
            feature: feature.clone(),
            children: feature
                .children
                .into_iter()
                .map(FeatureState::from)
                .collect(),
            slot_controls,
            effects_state: EffectsState::from(feature.effects),
        }
    }

    pub fn update(&mut self, message: FeatureMessage) -> IsDirty {
        match message {
            FeatureMessage::Use(path) => self.use_slot(&path),
            FeatureMessage::Reset(path, include_children) => self.reset(&path, include_children),
        }
    }

    fn use_slot(&mut self, path: &Vec<String>) -> IsDirty {
        let FeatureState {
            feature,
            slot_controls,
            children,
            effects_state,
        } = self;
        let mut path = path.clone();
        let head = path.get(0).map(|s| s.clone());
        match head {
            Some(head) => {
                path.remove(0);
                if (feature.name == head) {
                    if (path.is_empty()) {
                        let Feature {
                            name,
                            description,
                            slot,
                            children,
                            show_reset_chidren,
                            child_display_orientation,
                            effects,
                        } = feature;
                        match slot {
                            Some(slot) => {
                                slot.current = slot.current - 1;
                                true
                            }
                            None => false,
                        }
                    } else {
                        let mut dirty_child = false;
                        for child in children {
                            dirty_child = child.use_slot(&path) || dirty_child;
                        }
                        dirty_child
                    }
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn reset(&mut self, path: &Vec<String>, include_children: bool) -> IsDirty {
        let FeatureState {
            feature,
            slot_controls,
            children,
            effects_state,
        } = self;
        let mut path = path.clone();
        let head = path.first().map(|s| s.clone());
        match head {
            Some(head) => {
                path.remove(0);
                if feature.name == head {
                    if path.is_empty() {
                        let mut dirty_children = false;
                        if include_children {
                            for child in children {
                                dirty_children =
                                    child.reset(&path, include_children) || dirty_children;
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
                        } = feature;
                        let self_dirty = match slot {
                            Some(slot) => {
                                slot.current = slot.max.unwrap_or(0);
                                true
                            }
                            None => false,
                        };
                        self_dirty || dirty_children
                    } else {
                        let mut dirty_child = false;
                        for child in children {
                            dirty_child = child.reset(&path, include_children) || dirty_child;
                        }
                        dirty_child
                    }
                } else {
                    false
                }
            }
            None => {
                if include_children {
                    let mut dirty_child = false;
                    for child in children {
                        dirty_child = child.reset(&path, include_children) || dirty_child;
                    }

                    let Feature {
                        name,
                        description,
                        slot,
                        children,
                        show_reset_chidren,
                        child_display_orientation,
                        effects,
                    } = feature;
                    let self_dirty = match slot {
                        Some(slot) => {
                            slot.current = slot.max.unwrap_or(0);
                            true
                        }
                        None => false,
                    };
                    self_dirty || dirty_child
                } else {
                    false
                }
            }
        }
    }

    pub fn view<'a, T, F>(&'a mut self, parent_path: Vec<String>, f: &F) -> Column<'a, T>
    where
        T: Debug + Clone + 'a,
        F: Fn(FeatureMessage) -> T + 'a,
    {
        let FeatureState {
            feature,
            slot_controls,
            children,
            effects_state,
        } = self;

        let mut this_path = parent_path.clone();
        this_path.push(feature.name.clone());

        let mut child_elements = vec![];
        if !children.is_empty() {
            for child in children {
                child_elements.push(child.view(this_path.clone(), f).padding(4))
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
        } = feature;
        let mut header_row = Row::new()
            .spacing(20)
            .push(Text::new(name.clone()).size(24));

        match (slot_controls, slot) {
            (Some(slot_controls), Some(slot)) => {
                let FeatureSlot { current, max } = slot;
                header_row = match max {
                    Some(max) => header_row
                        .push(Text::new(format!("{} / {}", current.clone(), max.clone())).size(32)),
                    None => header_row.push(Text::new(format!("{}", current.clone())).size(32)),
                };

                let FeatureSlotControl {
                    use_slot,
                    reset,
                    reset_all,
                } = slot_controls;

                let button = Button::new(use_slot, Text::new("Use").size(16))
                    .on_press(f(FeatureMessage::Use(this_path.clone())))
                    .padding(8);
                header_row = header_row.push(button);

                let button = Button::new(reset, Text::new("Reset").size(16))
                    .on_press(f(FeatureMessage::Reset(this_path.clone(), false)))
                    .padding(8);
                header_row = header_row.push(button);

                if show_reset_chidren.unwrap_or(false) {
                    let button = Button::new(reset_all, Text::new("Reset All").size(16))
                        .padding(8)
                        .on_press(f(FeatureMessage::Reset(this_path.clone(), true)));
                    header_row = header_row.push(button);
                }
            }
            _ => {}
        };

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
}
