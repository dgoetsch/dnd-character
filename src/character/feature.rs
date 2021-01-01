use crate::character::Message;
use iced::{button, Button, Column, Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct FeaturesState {
    feature_state: Vec<FeatureState>,
}

#[derive(Debug, Clone, Default)]
pub struct FeatureState {
    feature: Feature,
    slot_controls: Option<FeatureSlotControl>,
    children: Vec<FeatureState>,
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
pub struct Feature {
    name: String,
    description: Option<String>,
    slot: Option<FeatureSlot>,
    children: Vec<Feature>,
    show_reset_chidren: bool,
}

impl FeatureState {
    fn persistable(&self) -> Feature {
        let FeatureState {
            feature,
            slot_controls,
            children,
        } = self;
        let mut feature = feature.clone();
        feature.children = vec![];
        for child in children {
            feature.children.push(child.persistable())
        }

        feature
    }

    pub fn from(feature: Feature) -> FeatureState {
        FeatureState {
            feature: feature.clone(),
            children: feature
                .children
                .into_iter()
                .map(FeatureState::from)
                .collect(),
            ..FeatureState::default()
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
        } = self;
        let mut path = path.clone();
        match path.pop() {
            Some(head) => {
                if (feature.name == head) {
                    if (path.is_empty()) {
                        let Feature {
                            name,
                            description,
                            slot,
                            children,
                            show_reset_chidren,
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
                            if (child.feature.name == head) {
                                dirty_child = child.use_slot(&path) || dirty_child;
                            }
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
        } = self;
        let mut path = path.clone();
        match path.pop() {
            Some(head) => {
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
                        } = feature;
                        let self_dirty = match slot {
                            Some(slot) => {
                                slot.current = slot.max;
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
                    } = feature;
                    let self_dirty = match slot {
                        Some(slot) => {
                            slot.current = slot.max;
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

    pub fn view(&mut self, parent_path: Vec<String>) -> Column<Message> {
        let FeatureState {
            feature,
            slot_controls,
            children,
        } = self;

        let mut this_path = parent_path.clone();
        this_path.push(feature.name.clone());

        let mut child_column = Column::new();
        if !children.is_empty() {
            child_column = child_column.spacing(2);
            for child in children {
                child_column = child_column.push(child.view(this_path.clone()).padding(4))
            }
        }

        let Feature {
            name,
            description,
            slot,
            children,
            show_reset_chidren,
        } = feature;
        let mut header_row = Row::new().push(Text::new(name.clone()).size(24));

        match (slot_controls, slot) {
            (Some(slot_controls), Some(slot)) => {
                let FeatureSlot { current, max } = slot;

                header_row = header_row
                    .push(Text::new(format!("{} / {}", current.clone(), max.clone())).size(24));

                let FeatureSlotControl {
                    use_slot,
                    reset,
                    reset_all,
                } = slot_controls;

                let button = Button::new(use_slot, Text::new("Use").size(16))
                    .on_press(Message::Feature(FeatureMessage::Use(this_path.clone())))
                    .padding(8);
                header_row = header_row.push(button);

                let button = Button::new(reset, Text::new("Reset").size(16))
                    .on_press(Message::Feature(FeatureMessage::Reset(
                        this_path.clone(),
                        false,
                    )))
                    .padding(8);
                header_row = header_row.push(button);

                if *show_reset_chidren {
                    let button = Button::new(reset_all, Text::new("Reset All").size(16))
                        .padding(8)
                        .on_press(Message::Feature(FeatureMessage::Reset(
                            this_path.clone(),
                            true,
                        )));
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

        column = column.push(child_column);

        column
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeatureSlot {
    current: isize,
    max: isize,
}
