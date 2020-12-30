use crate::character::Message;
use iced::{Column, Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Proficiencies {
    armor: Vec<String>,
    weapons: Vec<String>,
    tools: Vec<String>,
    languages: Vec<String>,
}

impl Proficiencies {
    pub fn view(&mut self) -> Column<Message> {
        Column::new()
            .push(Row::new().push(Text::new("Proficiences & Languages").size(24)))
            .push(Row::new().push(Text::new("Armor").size(20)))
            .push(Proficiencies::proficiency_list_row(self.armor.clone()))
            .push(Row::new().push(Text::new("Weapons").size(20)))
            .push(Proficiencies::proficiency_list_row(self.weapons.clone()))
            .push(Row::new().push(Text::new("Tools").size(20)))
            .push(Proficiencies::proficiency_list_row(self.tools.clone()))
            .push(Row::new().push(Text::new("Languages").size(20)))
            .push(Proficiencies::proficiency_list_row(self.languages.clone()))
    }

    fn proficiency_list_row<'a>(proficiency: Vec<String>) -> Row<'a, Message> {
        let text = if (proficiency.is_empty()) {
            "None".to_string()
        } else {
            proficiency.join(", ")
        };

        Row::new().push(Text::new(text).size(16)).padding(2)
    }
}
