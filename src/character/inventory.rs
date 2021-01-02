use crate::character::Message;
use crate::core::feature::{Feature, FeaturesState};
use crate::resources::item::Item;
use crate::util::two_column_row;
use iced::{Column, Row, Text};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inventory {
    equipped: Vec<InventoryItem>,
    on_person: Vec<InventoryItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryItem {
    name: String,
    item_name: String,
    features: Option<Vec<Feature>>,
}

impl Inventory {
    pub fn to_state(self) -> InventoryState {
        InventoryState {
            equipped: self
                .equipped
                .clone()
                .into_iter()
                .map(|i| i.to_state())
                .collect(),
            on_person: self.on_person.into_iter().map(|i| i.to_state()).collect(),
        }
    }
}

impl InventoryItem {
    fn to_state(self) -> InventoryItemState {
        let features = self.features.clone().map(FeaturesState::from);
        InventoryItemState {
            item: self,
            features: features,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InventoryState {
    equipped: Vec<InventoryItemState>,
    on_person: Vec<InventoryItemState>,
}

#[derive(Debug, Clone, Default)]
pub struct InventoryItemState {
    item: InventoryItem,
    features: Option<FeaturesState>,
}

// features: Vec<Feature>,
//TODO inventory (number available)
//TODO modifiers
impl InventoryState {
    pub fn persistable(&self) -> Inventory {
        Inventory {
            equipped: self
                .equipped
                .clone()
                .into_iter()
                .map(|i| i.persistable())
                .collect(),
            on_person: self
                .on_person
                .clone()
                .into_iter()
                .map(|i| i.persistable())
                .collect(),
        }
    }

    pub fn view(&mut self, items: Vec<Item>) -> Column<Message> {
        let items: HashMap<String, Item> = items.into_iter().map(|e| (e.name(), e)).collect();
        let InventoryState {
            equipped,
            on_person,
        } = self;

        let mut column = Column::new()
            .push(Text::new("Inventory").size(32))
            .push(Text::new("Equipped").size(24));

        for item in equipped {
            column = column
                .padding(8)
                .push(item.view(items.get(&item.item.item_name).map(|e| e.clone())))
        }

        column = column.push(Text::new("On Person").size(24));

        for item in on_person {
            column = column
                .padding(8)
                .push(item.view(items.get(&item.item.item_name).map(|e| e.clone())))
        }

        column
    }
}

impl InventoryItemState {
    fn persistable(&self) -> InventoryItem {
        self.item.clone()
    }

    fn view<'a>(&'a mut self, item: Option<Item>) -> Column<'a, Message> {
        let item_resource = item;
        let InventoryItemState { item, features } = self;
        let feature_state = features;
        let InventoryItem {
            name,
            item_name,
            features,
        } = item;
        let mut column = Column::new();

        column = column.push(Row::new().push(Text::new(name.clone())));

        match item_resource {
            Some(item) => column = column.push(item.clone().view()),
            None => {}
        }
        match feature_state {
            Some(features) => {
                println!("adding features to {}", name.clone());
                column = column.push(features.view(vec![item.name.clone()], Message::Feature))
            }
            None => {}
        }

        column
    }
}
