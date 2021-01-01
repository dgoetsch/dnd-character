use crate::character::Message;
use iced::{HorizontalAlignment, Length, Row, Text, VerticalAlignment};

pub fn format_modifier(value: isize) -> String {
    if value < 0 {
        format!("{}", value)
    } else {
        format!("+{}", value)
    }
}

pub fn two_column_row<'a>(left: Text, right: Text) -> Row<'a, Message> {
    Row::new()
        .push(
            left.size(16)
                .horizontal_alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Bottom)
                .width(Length::FillPortion(1)),
        )
        .push(
            right
                .size(16)
                .horizontal_alignment(HorizontalAlignment::Right)
                .vertical_alignment(VerticalAlignment::Bottom)
                .width(Length::FillPortion(1)),
        )
}

pub fn three_column_row<'a>(left: Text, middle: Text, right: Text) -> Row<'a, Message> {
    Row::new()
        .push(
            left.horizontal_alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Bottom)
                .width(Length::FillPortion(1)),
        )
        .push(
            middle
                .horizontal_alignment(HorizontalAlignment::Right)
                .vertical_alignment(VerticalAlignment::Bottom)
                .width(Length::FillPortion(1)),
        )
        .push(
            right
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Bottom)
                .width(Length::FillPortion(1)),
        )
}
