use super::Message;
use iced::{HorizontalAlignment, Row, Text, VerticalAlignment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Name {
    name: String,
}
impl Name {
    pub fn view(&mut self) -> Row<Message> {
        Row::new().push(
            Text::new(self.name.clone())
                .size(48)
                .horizontal_alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Bottom),
        )
    }
}
