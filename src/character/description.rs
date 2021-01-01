use crate::character::Message;
use crate::dimensions::Weight;
use crate::util::two_column_row;
use iced::{Column, Element, HorizontalAlignment, Length, Row, Text, VerticalAlignment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Description {
    race: String,
    size: Size,
    alignment: Alignment,
    speed: isize,
    age: Option<isize>,
    height: Option<Height>,
    weight: Option<Weight>,
    hair: Option<String>,
    eyes: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Height {
    feet: isize,
    inches: isize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl Default for Size {
    fn default() -> Size {
        Size::Medium
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Alignment {
    order: OrderAlignment,
    morality: MoralAlignment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderAlignment {
    Chaotic,
    Neutral,
    Lawful,
}

impl Default for OrderAlignment {
    fn default() -> Self {
        OrderAlignment::Neutral
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MoralAlignment {
    Evil,
    Neutral,
    Good,
}
impl Default for MoralAlignment {
    fn default() -> Self {
        MoralAlignment::Neutral
    }
}

impl Description {
    pub fn view(&mut self) -> Column<Message> {
        let Description {
            race,
            size,
            alignment,
            speed,
            age,
            height,
            weight,
            hair,
            eyes,
        } = self;
        let mut column_1 = Column::new()
            .push(two_column_row(Text::new("Race"), Text::new(race.clone())))
            .push(two_column_row(
                Text::new("Size"),
                Text::new(format!("{:?}", size)),
            ))
            .push(two_column_row(
                Text::new("Alignment"),
                Text::new(format!("{:?} {:?}", alignment.order, alignment.morality)),
            ))
            .push(two_column_row(
                Text::new("Speed"),
                Text::new(format!("{} feet", speed)),
            ));

        let mut column_2 = Column::new();
        column_2 = match age {
            None => column_2,
            Some(age) => column_2.push(two_column_row(
                Text::new("Age"),
                Text::new(format!("{} years", age)),
            )),
        };
        column_2 = match height {
            None => column_2,
            Some(height) => column_2.push(height.view()),
        };
        column_2 = match weight {
            None => column_2,
            Some(weight) => column_2.push(weight.view()),
        };
        column_2 = match hair {
            None => column_2,
            Some(hair) => column_2.push(two_column_row(Text::new("Hair"), Text::new(hair.clone()))),
        };
        column_2 = match eyes {
            None => column_2,
            Some(eyes) => column_2.push(two_column_row(Text::new("Eyes"), Text::new(eyes.clone()))),
        };

        Column::new().push(
            Row::new()
                .spacing(8)
                .push(column_1.width(Length::FillPortion(1)))
                .push(column_2.width(Length::FillPortion(1))),
        )
    }
}

impl Height {
    pub fn view(&mut self) -> Row<Message> {
        two_column_row(
            Text::new("Height"),
            Text::new(format!("{}' {}\"", self.feet, self.inches)),
        )
    }
}
