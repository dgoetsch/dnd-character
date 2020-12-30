use serde::{Deserialize, Serialize};

use iced::{button, Align, Button, Column, HorizontalAlignment, Length, Row, Text, TextInput};

use super::Message;

#[derive(Debug, Clone, Default)]
pub struct HitPointControls {
    increment_button: button::State,
    decrement_button: button::State,
    heal_button: button::State,
    damage_button: button::State,
    reset_button: button::State,
    full_health_button: button::State,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HitPoints {
    current_hit_points: isize,
    max_hit_points: isize,
}

#[derive(Debug, Clone)]
pub enum HitPointMessage {
    IncrementHealthDelta,
    DecrementHealthDelta,
    ChangeHealthDelta(String),
    Heal,
    Damage,
    ResetDelta,
    FullHealth,
}

#[derive(Debug, Clone, Default)]
pub struct HitPointState {
    hit_points: HitPoints,
    hp_delta: isize,
    change_hp: iced::text_input::State,
    hp_controls: crate::character::hitpoints::HitPointControls,
}

impl HitPoints {
    pub fn to_state(self) -> HitPointState {
        HitPointState {
            hit_points: self,
            ..HitPointState::default()
        }
    }
}

type IsDirty = bool;

impl HitPointState {
    pub fn persistable(&self) -> HitPoints {
        self.hit_points.clone()
    }

    pub fn update(&mut self, message: HitPointMessage) -> IsDirty {
        match message {
            HitPointMessage::IncrementHealthDelta => {
                self.hp_delta = self.hp_delta + 1;
                false
            }
            HitPointMessage::DecrementHealthDelta => {
                self.hp_delta = self.hp_delta - 1;
                false
            }
            HitPointMessage::ChangeHealthDelta(delta) => {
                match delta.parse::<isize>() {
                    Ok(delta) => {
                        self.hp_delta = delta;
                    }
                    _ => (),
                }
                false
            }
            HitPointMessage::ResetDelta => {
                self.hp_delta = 0;
                false
            }
            HitPointMessage::Heal => {
                self.hit_points.current_hit_points =
                    self.hit_points.current_hit_points + self.hp_delta;
                true
            }
            HitPointMessage::Damage => {
                self.hit_points.current_hit_points =
                    self.hit_points.current_hit_points - self.hp_delta;
                true
            }
            HitPointMessage::FullHealth => {
                self.hit_points.current_hit_points = self.hit_points.max_hit_points;
                true
            }
        }
    }

    pub fn view(&mut self) -> Column<Message> {
        let HitPointState {
            hit_points,
            hp_delta,
            change_hp,
            hp_controls,
        } = self;

        let hp = Text::new(format!(
            "{} / {} HP",
            hit_points.current_hit_points, hit_points.max_hit_points
        ))
        .width(Length::Fill)
        .size(60)
        .color([0.5, 0.5, 0.5])
        .horizontal_alignment(HorizontalAlignment::Center);

        let delta_input = TextInput::new(
            change_hp,
            "Change current HP by",
            hp_delta.to_string().as_str(),
            |v| Message::HitPoint(HitPointMessage::ChangeHealthDelta(v)),
        )
        .padding(15)
        .size(30)
        .on_submit(Message::HitPoint(HitPointMessage::Heal));

        let hp_controls = hp_controls.view();

        Column::new().push(hp).push(delta_input).push(hp_controls)
    }
}

impl HitPointControls {
    fn view(&mut self) -> Row<Message> {
        let HitPointControls {
            increment_button,
            decrement_button,
            heal_button,
            damage_button,
            reset_button,
            full_health_button,
        } = self;

        let button = |state, label, message, style, width| {
            let label = Text::new(label).size(30);
            let button = Button::new(state, label)
                .style(style)
                .width(Length::FillPortion(width));

            button.on_press(message).padding(8)
        };

        Row::new().spacing(20).align_items(Align::Center).push(
            Row::new()
                .width(Length::Fill)
                .spacing(10)
                .push(button(
                    damage_button,
                    "Damage",
                    Message::HitPoint(HitPointMessage::Damage),
                    style::Button::Damage,
                    2,
                ))
                .push(button(
                    decrement_button,
                    "-",
                    Message::HitPoint(HitPointMessage::DecrementHealthDelta),
                    style::Button::Decrement,
                    1,
                ))
                .push(button(
                    increment_button,
                    "+",
                    Message::HitPoint(HitPointMessage::IncrementHealthDelta),
                    style::Button::Increment,
                    1,
                ))
                .push(button(
                    heal_button,
                    "Heal",
                    Message::HitPoint(HitPointMessage::Heal),
                    style::Button::Heal,
                    2,
                ))
                .push(button(
                    reset_button,
                    "Reset",
                    Message::HitPoint(HitPointMessage::ResetDelta),
                    style::Button::Reset,
                    1,
                ))
                .push(button(
                    full_health_button,
                    "Full_Health",
                    Message::HitPoint(HitPointMessage::FullHealth),
                    style::Button::Reset,
                    2,
                )),
        )
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Increment,
        Decrement,
        Reset,
        Heal,
        Damage,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Increment => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.7, 0.2))),
                    border_radius: 10.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                },
                Button::Decrement => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.7, 0.2, 0.2))),
                    border_radius: 10.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                },
                Button::Reset => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.7))),
                    border_radius: 10.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                },
                Button::Heal => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.7, 0.1))),
                    border_radius: 10.0,
                    text_color: Color::BLACK,
                    ..button::Style::default()
                },
                Button::Damage => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.7, 0.1, 0.1))),
                    border_radius: 10.0,
                    text_color: Color::BLACK,
                    ..button::Style::default()
                },
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                text_color: match self {
                    Button::Increment => Color::from_rgb(0.2, 0.2, 0.2),
                    Button::Decrement => Color::from_rgb(0.2, 0.2, 0.2),
                    Button::Reset => Color::from_rgb(0.2, 0.2, 0.7),
                    Button::Heal => Color::from_rgb(0.2, 0.7, 0.2),
                    Button::Damage => Color::from_rgb(0.7, 0.2, 0.2),
                },
                shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                ..active
            }
        }
    }
}
