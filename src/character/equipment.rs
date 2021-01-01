use crate::character::feature::FeaturesState;
use crate::character::Message;
use crate::resources::equipment::Equipment;
use crate::util::two_column_row;
use iced::{Column, Row, Text};

pub struct CharacterEquipment {}

pub struct EquipmentState {
    equipment: Equipment,
    features_state: FeaturesState,
}

// features: Vec<Feature>,
//TODO inventory (number available)
//TODO modifiers
impl EquipmentState {
    pub fn from(equipment: Equipment) -> EquipmentState {
        // let equipment_features = equipment.features();
        let features_state = FeaturesState::from(vec![]);

        EquipmentState {
            equipment,
            features_state,
        }
    }

    pub fn persistable(&self) -> Equipment {
        self.equipment.clone()
    }

    pub fn view(&mut self) -> Column<Message> {
        let EquipmentState {
            equipment,
            features_state,
        } = self;

        let mut column = equipment.view().padding(2).spacing(2);

        if !features_state.is_empty() {
            column = column.push(Row::new().push(features_state.view()));
        }

        column
    }
}
