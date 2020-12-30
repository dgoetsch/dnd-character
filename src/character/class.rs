use crate::character::Message;
use crate::util::format_modifier;
use iced::{Column, HorizontalAlignment, Length, Row, Text, VerticalAlignment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Classes {
    classes: Vec<Class>,
}

impl Classes {
    pub fn from(classes: Vec<Class>) -> Classes {
        Classes { classes }
    }
    pub fn persistable(&self) -> Vec<Class> {
        self.classes.clone()
    }

    pub fn view(&mut self) -> Column<Message> {
        let total_level = self.total_level();
        let proficiency = self.proficiency();
        let mut column = Column::new();

        if (self.classes.len() > 1) {
            let level_row = Row::new()
                .width(Length::Fill)
                .push(Text::new("Level").width(Length::FillPortion(1)).size(24))
                .push(
                    Text::new(total_level.to_string())
                        .width(Length::FillPortion(1))
                        .size(24),
                );

            column = column.push(level_row);
        }

        let Classes { classes } = self;

        for class in classes {
            column = column.push(class.view().width(Length::Fill))
        }

        let proficiency = format_modifier(proficiency);
        let proficiency_row = Row::new()
            .width(Length::Fill)
            .padding(4)
            .spacing(4)
            .push(
                Text::new("Proficiency")
                    .vertical_alignment(VerticalAlignment::Bottom)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .width(Length::FillPortion(1))
                    .size(16),
            )
            .push(
                Text::new(proficiency)
                    .vertical_alignment(VerticalAlignment::Bottom)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .width(Length::FillPortion(1))
                    .size(24),
            );

        column.push(proficiency_row)
    }

    fn total_level(&self) -> isize {
        self.classes.clone().into_iter().map(|c| c.level).sum()
    }

    fn proficiency(&self) -> isize {
        let total_level = self.total_level();
        if total_level < 5 {
            2
        } else if total_level < 10 {
            3
        } else if total_level < 14 {
            4
        } else if total_level < 17 {
            5
        } else if total_level <= 20 {
            6
        } else {
            total_level / 4 + 2
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    name: String,
    level: isize,
}

impl Class {
    fn view(&mut self) -> Row<Message> {
        Row::new().push(Text::new(format!("Level {} {}", self.level, self.name)).size(24))
    }
}
#[cfg(test)]
mod test {
    use crate::character::class::{Class, Classes};

    fn assert_proficiency(v: (Vec<isize>, isize)) {
        let (levels, proficiency) = v;
        let classes = Classes {
            classes: levels
                .into_iter()
                .map(|l| Class {
                    name: "test".to_string(),
                    level: l,
                })
                .collect(),
        };
        assert_eq!(classes.proficiency(), proficiency);
    }
    #[test]
    fn test_proficiency() {
        vec![
            (vec![1], 2),
            (vec![2], 2),
            (vec![3], 2),
            (vec![4], 2),
            (vec![5], 3),
            (vec![6], 3),
            (vec![7], 3),
            (vec![8], 3),
            (vec![9], 3),
            (vec![10], 4),
            (vec![11], 4),
            (vec![12], 4),
            (vec![13], 4),
            (vec![14], 5),
            (vec![15], 5),
            (vec![16], 5),
            (vec![17], 6),
            (vec![18], 6),
            (vec![19], 6),
            (vec![20], 6),
        ]
        .into_iter()
        .for_each(assert_proficiency)
    }
}
