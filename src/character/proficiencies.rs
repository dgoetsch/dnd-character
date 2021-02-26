use crate::character::class::Classes;
use crate::character::Message;
use iced::{Column, Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProficiencyType {
    None,
    Half,
    Full,
}

impl Default for ProficiencyType {
    fn default() -> Self {
        ProficiencyType::None
    }
}

impl ProficiencyType {
    pub fn modifier_for_bonus(&self, class_proficiency_bonus: isize) -> isize {
        match self {
            ProficiencyType::None => 0,
            ProficiencyType::Half => class_proficiency_bonus / 2,
            ProficiencyType::Full => class_proficiency_bonus,
        }
    }
    pub fn modifier(&self, class: Classes) -> isize {
        match self {
            ProficiencyType::None => 0,
            ProficiencyType::Half => class.proficiency_bonus() / 2,
            ProficiencyType::Full => class.proficiency_bonus(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Proficiency {
    name: String,
    proficiency_type: ProficiencyType,
}

impl Proficiency {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn proficiency_type(&self) -> ProficiencyType {
        self.proficiency_type.clone()
    }
    pub fn view(self) -> String {
        match self.proficiency_type {
            ProficiencyType::Full => self.name.clone(),
            ProficiencyType::Half => format!("{} (Half)", self.name),
            ProficiencyType::None => "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Proficiencies {
    tools: Vec<Proficiency>,
    languages: Vec<Proficiency>,
}

impl Proficiencies {
    pub fn view(&mut self) -> Column<Message> {
        let Proficiencies { tools, languages } = self;
        Column::new()
            .push(Row::new().push(Text::new("Proficiences & Languages").size(24)))
            .push(Row::new().push(Text::new("Tools").size(20)))
            .push(Proficiencies::proficiency_list_row(tools.clone()))
            .push(Row::new().push(Text::new("Languages").size(20)))
            .push(Proficiencies::proficiency_list_row(languages.clone()))
    }

    fn proficiency_list_row<'a>(proficiencies: Vec<Proficiency>) -> Row<'a, Message> {
        let text = if (proficiencies.is_empty()) {
            "None".to_string()
        } else {
            proficiencies
                .into_iter()
                .map(|p| p.view())
                .collect::<Vec<String>>()
                .join(", ")
        };

        Row::new().push(Text::new(text).size(16)).padding(2)
    }
}
