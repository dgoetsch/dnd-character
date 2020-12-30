use super::Message;
use iced::{button, Align, Button, Column, Row, Text};
use serde::{Deserialize, Serialize};
pub type Level = usize;
type IsDirty = bool;

#[derive(Debug, Clone, Default)]
pub struct SpellSlotsState {
    spell_slots: Vec<SpellSlotState>,
    reset_all: button::State,
}

#[derive(Debug, Clone)]
pub enum SpellSlotMessage {
    UseSpell(Level),
    ResetSlots(Option<Level>),
}

impl SpellSlotsState {
    pub fn from(spell_slots: Vec<SpellSlot>) -> SpellSlotsState {
        SpellSlotsState {
            spell_slots: spell_slots.into_iter().map(|s| s.to_state()).collect(),
            ..SpellSlotsState::default()
        }
    }
    pub fn persistable(&self) -> Vec<SpellSlot> {
        self.spell_slots
            .clone()
            .into_iter()
            .map(|s| s.persistable())
            .collect()
    }

    pub fn update(&mut self, message: SpellSlotMessage) -> IsDirty {
        match message {
            SpellSlotMessage::UseSpell(level) => {
                self.use_spell_slot(level);
                true
            }
            SpellSlotMessage::ResetSlots(maybe_level) => {
                self.reset_spell_slot(maybe_level);
                true
            }
        }
    }
    pub fn view(&mut self) -> Column<Message> {
        let SpellSlotsState {
            spell_slots,
            reset_all,
        } = self;

        let mut column = Column::new().push(Text::new("Spells"));

        for spell_slot in spell_slots {
            column = column.push(spell_slot.view());
        }

        column = column.push(
            Row::new().push(
                Button::new(reset_all, Text::new("Reset All").size(32))
                    .on_press(Message::SpellSlot(SpellSlotMessage::ResetSlots(None))),
            ),
        );
        column
    }

    fn use_spell_slot(&mut self, level: Level) {
        let SpellSlotsState {
            spell_slots,
            reset_all,
        } = self;
        for state in spell_slots {
            if state.spell_slot.level == level {
                state.spell_slot.available = state.spell_slot.available - 1
            }
        }
    }

    pub fn reset_spell_slot(&mut self, level: Option<Level>) {
        let SpellSlotsState {
            spell_slots,
            reset_all,
        } = self;

        for state in spell_slots {
            if level.is_none() || level == Some(state.spell_slot.level) {
                state.spell_slot.available = state.spell_slot.max
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpellSlot {
    level: Level,
    available: usize,
    max: usize,
}

impl SpellSlot {
    pub fn to_state(self) -> SpellSlotState {
        SpellSlotState {
            spell_slot: self,
            ..SpellSlotState::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpellSlotControl {
    use_slot: button::State,
    reset: button::State,
}

#[derive(Debug, Clone, Default)]
pub struct SpellSlotState {
    spell_slot: SpellSlot,
    controls: SpellSlotControl,
}

impl SpellSlotState {
    pub fn persistable(&self) -> SpellSlot {
        self.spell_slot.clone()
    }

    fn view(&mut self) -> Row<Message> {
        let SpellSlotState {
            spell_slot,
            controls,
        } = self;

        let SpellSlot {
            level,
            available,
            max,
        } = spell_slot;

        let SpellSlotControl { use_slot, reset } = controls;

        Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new(format!("Level: {}", level.clone())).size(24))
            .push(Text::new(format!("{} / {}", available.clone(), max.clone())).size(32))
            .push(SpellSlotState::use_slot(level.clone(), use_slot))
            .push(SpellSlotState::reset_slot(level.clone(), reset))
    }

    fn use_slot(level: Level, use_slot: &mut button::State) -> Button<Message> {
        let label = Text::new("Cast").size(24);
        let button = Button::new(use_slot, label);
        button
            .on_press(Message::SpellSlot(SpellSlotMessage::UseSpell(level)))
            .padding(8)
    }

    fn reset_slot(level: Level, reset: &mut button::State) -> Button<Message> {
        let label = Text::new("Reset").size(24);
        let button = Button::new(reset, label);
        button
            .on_press(Message::SpellSlot(SpellSlotMessage::ResetSlots(Some(
                level,
            ))))
            .padding(8)
    }
}
