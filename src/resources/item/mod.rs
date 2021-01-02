use crate::character::feature::{Feature, FeaturesState};
use crate::character::Message;
use crate::dimensions::Weight;
use crate::resources::item::weapon::Weapon;
use crate::util::two_column_row;
use iced::{Column, Row, Text};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod weapon;

type ItemType = String;
type ItemSubType = String;

//TODO add feature

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    name: String,
    weight: Weight,
    item_type: ItemType,
    sub_type: ItemType,
    weapon: Option<Weapon>,
}

impl Item {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn weapon(name: String, weight: Weight, sub_type: ItemSubType, weapon: Weapon) -> Item {
        let weapon = Some(weapon);
        let item_type = "Weapon".to_string();
        Item {
            name,
            weight,
            item_type,
            sub_type,
            weapon,
        }
    }
    pub fn view<'a>(self) -> Column<'a, Message> {
        let Item {
            name,
            weight,
            item_type,
            sub_type,
            weapon,
        } = self;

        let mut column = Column::new()
            .push(Row::new().push(Text::new(name.clone()).size(16)))
            .push(weight.view())
            .push(two_column_row(
                Text::new("Type"),
                Text::new(format!("{} ({})", sub_type, item_type)),
            ));

        match weapon {
            Some(weapon) => column = column.push(Row::new().push(weapon.view())),
            None => {}
        }

        column
    }
}

#[cfg(test)]
mod test {
    use crate::core::{Damage, Dice};
    use crate::dimensions::Weight;
    use crate::resources::item::weapon::{Weapon, WeaponProperty};
    use crate::resources::item::Item;

    fn lbs(lbs: isize) -> Weight {
        Weight::new(lbs, 0)
    }

    fn ndn(count: isize, sides: isize) -> Dice {
        Dice::new(count, sides)
    }

    fn d8(count: isize) -> Dice {
        ndn(count, 8)
    }

    fn d10(count: isize) -> Dice {
        ndn(count, 10)
    }
    fn slashing(dice: Dice) -> Damage {
        Damage::new(dice, None, "Slashing".to_string())
    }

    fn piercing(dice: Dice) -> Damage {
        Damage::new(dice, None, "Piercing".to_string())
    }
    fn versatile(damage: Vec<Damage>) -> WeaponProperty {
        WeaponProperty::Versatile { damage: damage }
    }

    fn ammunition() -> WeaponProperty {
        WeaponProperty::Ammunition
    }

    fn loading() -> WeaponProperty {
        WeaponProperty::Loading
    }

    fn range(normal: isize, long: isize) -> WeaponProperty {
        WeaponProperty::Range { normal, long }
    }
    fn twoHanded() -> WeaponProperty {
        WeaponProperty::TwoHanded
    }

    fn weapon(damage: Vec<Damage>, properties: Vec<WeaponProperty>) -> Weapon {
        Weapon::new(damage, properties)
    }

    fn item<T: Into<String>>(name: T, weight: Weight, item_type: T, weapon: Weapon) -> Item {
        Item::weapon(name.into(), weight, item_type.into(), weapon)
    }

    #[test]
    fn test_item() {
        let longsword = item(
            "Longsword",
            lbs(3),
            "Martial Melee Weapon",
            weapon(
                vec![slashing(d8(1))],
                vec![versatile(vec![slashing(d10(1))])],
            ),
        );

        let lightCrossbow = item(
            "Crossbow, Light",
            lbs(5),
            "Simple Ranged Weapon",
            weapon(
                vec![piercing(d8(1))],
                vec![ammunition(), loading(), range(80, 320), twoHanded()],
            ),
        );

        let weapons = vec![longsword, lightCrossbow];

        let result = serde_json::to_string_pretty(&weapons);

        println!("{}", result.unwrap_or("".to_string()));
    }
}
