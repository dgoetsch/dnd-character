use crate::character::feature::{Feature, FeaturesState};
use crate::character::Message;
use crate::dimensions::Weight;
use crate::resources::equipment::weapon::Attack;
use crate::util::two_column_row;
use iced::{Column, Row, Text};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod weapon;

type EquipmentType = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    name: String,
    weight: Weight,
    equipment_type: EquipmentType,
    attack: Option<Attack>,
}

impl Equipment {
    pub fn weapon(
        name: String,
        weight: Weight,
        equipment_type: EquipmentType,
        attack: Attack,
    ) -> Equipment {
        let attack = Some(attack);
        Equipment {
            name,
            weight,
            equipment_type,
            attack,
        }
    }
    pub fn view(&mut self) -> Column<Message> {
        let Equipment {
            name,
            weight,
            equipment_type,
            attack,
        } = self;

        let mut column = Column::new()
            .push(Row::new().push(Text::new(name.clone())))
            .push(weight.view())
            .push(two_column_row(
                Text::new("Type"),
                Text::new(equipment_type.to_string()),
            ));

        match attack {
            Some(attack) => column = column.push(Row::new().push(attack.view())),
            None => {}
        }

        column
    }
}

#[cfg(test)]
mod test {
    use crate::core::{Damage, Dice};
    use crate::dimensions::Weight;
    use crate::resources::equipment::weapon::{Attack, WeaponProperty};
    use crate::resources::equipment::Equipment;

    fn lbs(lbs: isize) -> Weight {
        Weight::new(lbs, 0)
    }

    fn ndn(count: isize, sides: isize) -> Dice {
        Dice::new(count, sides)
    }

    fn d8(count: isize) -> Dice {
        ndn(count, 8)
    }

    fn d8(count: isize) -> Dice {
        ndn(count, 10)
    }
    fn slashing(dice: Dice) -> Damage {
        Damage::new(dice, "Slashing".to_string())
    }

    fn versatile(damage: Vec<Damage>) -> WeaponProperty {
        WeaponProperty::Versatile { damage: damage }
    }

    fn attack(damage: Vec<Damage>, properties: Vec<WeaponProperty>) -> Attack {
        Attack::new(damage, properties)
    }

    fn weapon<T: Into<String>>(
        name: T,
        weight: Weight,
        equipment_type: T,
        attack: Attack,
    ) -> Equipment {
        Equipment::weapon(name.into(), weight, equipment_type.into(), attack)
    }

    #[test]
    fn test_equipment() {
        let longsword = weapon(
            "Longsword",
            lbs(3),
            "Martial Melee Weapon",
            attack(
                vec![slashing(d8(1))],
                vec![versatile(vec![slashing(d10(1))])],
            ),
        );

        let result = serde_json::to_string_pretty(&longsword);

        println!("{}", result.unwrap_or("".to_string()));

        assert!(false)
    }
}
