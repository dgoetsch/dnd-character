use crate::core::roll::Advantage;
use crate::core::roll::{Dice, Roll};
use crate::util::format_modifier;
use iced::{Element, HorizontalAlignment, Length, Text, VerticalAlignment};
use std::collections::HashSet;

pub struct Rollable {
    dice: Vec<Dice>,
    advantage_counter: isize,
    reroll: HashSet<isize>,
    bonus: isize,
}

impl Rollable {
    pub fn from(
        dice: Vec<Dice>,
        reroll: HashSet<isize>,
        advantage_counter: isize,
        bonus: isize,
    ) -> Rollable {
        Rollable {
            dice,
            advantage_counter,
            reroll,
            bonus,
        }
    }

    pub fn merge(&mut self, other: Rollable) {
        self.dice.extend(other.dice);
        self.reroll.extend(other.reroll);
        self.advantage_counter = other.advantage_counter + self.advantage_counter;
        self.bonus = self.bonus + other.bonus;
    }

    pub fn add_bonus(&mut self, bonus: isize) {
        self.bonus = self.bonus + bonus;
    }

    pub fn advantage(&self) -> Option<Advantage> {
        if self.advantage_counter == 0 {
            None
        } else if self.advantage_counter > 0 {
            Some(Advantage::Advantage)
        } else {
            Some(Advantage::Disadvantage)
        }
    }

    pub fn dice(&self) -> Vec<Dice> {
        self.dice.clone()
    }

    pub fn bonus(&self) -> isize {
        self.bonus
    }

    pub fn view<'a, T>(&self) -> Element<'a, T> {
        let dice: Option<String> = Some(
            self.dice()
                .into_iter()
                .map(|d| d.to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
                .join("+"),
        )
        .filter(|s| !s.is_empty());

        let bonus = Some(format_modifier(self.bonus())).filter(|s| !s.is_empty());
        let advantage = self.advantage().map(|a| format!("({})", a.to_string()));
        let bonus_dice_and_modifier = Some(
            vec![dice, bonus]
                .into_iter()
                .flatten()
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
                .join(""),
        )
        .filter(|b| !b.is_empty());

        let text = vec![bonus_dice_and_modifier, advantage]
            .into_iter()
            .flatten()
            .collect::<Vec<String>>()
            .join(" ");

        Text::new(text)
            .size(16)
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Bottom)
            .width(Length::FillPortion(1))
            .into()
    }
}
