use crate::character::Message;
use iced::{Column, Element, HorizontalAlignment, Length, Row, Text, VerticalAlignment};
use std::fmt::Debug;

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

pub fn two_element_row<'a, T: Debug + Clone + 'a>(
    left: Element<'a, T>,
    right: Element<'a, T>,
) -> Row<'a, T> {
    Row::new()
        .push(Column::new().width(Length::FillPortion(1)).push(left))
        .push(Column::new().width(Length::FillPortion(1)).push(right))
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

pub fn three_element_row<'a, T: Debug + Clone + 'a>(
    left: Element<'a, T>,
    middle: Element<'a, T>,
    right: Element<'a, T>,
) -> Row<'a, T> {
    Row::new()
        .push(Column::new().width(Length::FillPortion(1)).push(left))
        .push(Column::new().width(Length::FillPortion(1)).push(middle))
        .push(Column::new().width(Length::FillPortion(1)).push(right))
}
