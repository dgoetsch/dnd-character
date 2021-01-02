use crate::character::Message;
use crate::util::two_column_row;
use iced::{Row, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Weight {
    lbs: isize,
    oz: isize,
}

impl Weight {
    pub fn new(lbs: isize, oz: isize) -> Weight {
        Weight { lbs, oz }
    }

    pub fn view<'a>(self) -> Row<'a, Message> {
        two_column_row(
            Text::new("Weight"),
            Text::new(format!("{} lbs, {} oz", self.lbs, self.oz)),
        )
    }
}
