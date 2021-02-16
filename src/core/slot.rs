use crate::core::feature_path::FeaturePath;
use iced::{button, Button, Column, Element, Length, Row, Text};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Slot {
    current: isize,
    max: Option<isize>,
}

impl Slot {
    pub fn new(current: isize, max: Option<isize>) -> Slot {
        Slot { current, max }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SlotState {
    control: SlotControl,
    slot: Slot,
}

#[derive(Debug, Clone)]
pub enum SlotCommand {
    Use,
    Reset,
}

impl SlotState {
    pub fn persistable(&self) -> Slot {
        let SlotState { control, slot } = self;
        slot.clone()
    }

    pub fn from(slot: Slot) -> SlotState {
        SlotState {
            control: SlotControl::default(),
            slot,
        }
    }

    pub fn use_slot(&mut self) -> bool {
        let SlotState { control, slot } = self;
        slot.current = slot.current - 1;
        true
    }

    pub fn reset(&mut self) -> bool {
        let SlotState { control, slot } = self;
        if (slot.max == Some(slot.current)) {
            false
        } else {
            slot.current = slot.max.unwrap_or(0);
            true
        }
    }

    pub fn update(&mut self, command: SlotCommand) -> bool {
        match command {
            SlotCommand::Use => self.use_slot(),
            SlotCommand::Reset => self.reset(),
        }
    }

    pub fn view<'a, 'b, T, F>(&'a mut self, f: &'b F) -> Column<'a, T>
    where
        'a: 'b,
        T: Debug + Clone + 'a,
        F: Fn(SlotCommand) -> T + 'b,
    {
        let SlotState { slot, control } = self;

        let Slot { current, max } = slot;
        let mut row = Row::new().spacing(20);

        row = match max {
            Some(max) => {
                row.push(Text::new(format!("{} / {}", current.clone(), max.clone())).size(32))
            }
            None => row.push(Text::new(format!("{}", current.clone())).size(32)),
        };

        let SlotControl {
            use_slot,
            reset,
            reset_all,
        } = control;

        let button = Button::new(use_slot, Text::new("Use").size(16))
            .on_press(f(SlotCommand::Use))
            .padding(8);
        row = row.push(button);

        let button = Button::new(reset, Text::new("Reset").size(16))
            .on_press(f(SlotCommand::Reset))
            .padding(8);
        row = row.push(button);

        Column::new().push(row)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SlotControl {
    use_slot: button::State,
    reset: button::State,
    reset_all: button::State,
}

pub trait FromSlotCommand<T> {
    fn from(command: SlotCommand) -> T;
}
