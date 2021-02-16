use crate::character::class::Classes;
use crate::character::proficiencies::{Proficiencies, Proficiency};
use crate::character::Message;
use crate::core::ability_score::ModifiedAbilityScores;
use crate::core::effect::Effect;
use crate::core::feature::{Feature, FeaturesState};
use crate::core::feature_path::FeaturePath;
use crate::core::roll::{CheckBonus, CheckRoll, CheckRollType, DamageRollScope};
use crate::core::Damage;
use crate::resources::item::Item;
use crate::util::two_column_row;
use iced::{Column, Row, Text};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inventory {
    equipped: Vec<InventoryItem>,
    on_person: Vec<InventoryItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryItem {
    name: String,
    item_name: String,
    features: Option<Vec<Feature>>,
}

impl Inventory {
    pub fn to_state(self) -> InventoryState {
        InventoryState {
            equipped: self
                .equipped
                .clone()
                .into_iter()
                .map(|i| i.to_state())
                .collect(),
            on_person: self.on_person.into_iter().map(|i| i.to_state()).collect(),
        }
    }
}

impl InventoryItem {
    fn to_state(self) -> InventoryItemState {
        let features = self.features.clone().map(FeaturesState::from);
        InventoryItemState {
            item: self,
            features: features,
            ..InventoryItemState::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InventoryState {
    equipped: Vec<InventoryItemState>,
    on_person: Vec<InventoryItemState>,
}

#[derive(Debug, Clone, Default)]
pub struct InventoryItemState {
    item: InventoryItem,
    features: Option<FeaturesState>,
    attack_bonus: Vec<(FeaturePath, CheckBonus)>,
    damage_bonus: Vec<(FeaturePath, Damage)>,
}

// features: Vec<Feature>,
//TODO inventory (number available)
//TODO modifiers
impl InventoryState {
    pub fn apply_all(&mut self, effects: &Vec<Effect>) {
        let InventoryState {
            equipped,
            on_person,
        } = self;

        for state in equipped {
            state.apply_all(effects)
        }

        for state in on_person {
            state.apply_all(effects)
        }
    }
    pub fn effects_from_equipped(&self) -> Vec<Effect> {
        let InventoryState {
            equipped,
            on_person,
        } = self;

        let mut result = vec![];
        for item in equipped {
            result.extend(item.effects());
        }
        result
    }
    pub fn persistable(&self) -> Inventory {
        Inventory {
            equipped: self
                .equipped
                .clone()
                .into_iter()
                .map(|i| i.persistable())
                .collect(),
            on_person: self
                .on_person
                .clone()
                .into_iter()
                .map(|i| i.persistable())
                .collect(),
        }
    }

    pub fn view(
        &mut self,
        items: Vec<Item>,
        ability_scores: ModifiedAbilityScores,
        proficiencies: &Proficiencies,
        classes: &Classes,
    ) -> Column<Message> {
        let items: HashMap<String, Item> = items.into_iter().map(|e| (e.name(), e)).collect();
        let InventoryState {
            equipped,
            on_person,
        } = self;

        let bonus = classes.proficiency_bonus();
        let weapon_proficiencies = proficiencies.weapons();

        let mut column = Column::new()
            .push(Text::new("Inventory").size(32))
            .push(Text::new("Equipped").size(24));

        for item in equipped {
            let inventory_item = items.get(&item.item.item_name).map(|e| e.clone());

            let proficiency_modifier = match inventory_item.clone() {
                Some(item) => item
                    .proficiency_for(&weapon_proficiencies)
                    .modifier_for_bonus(bonus),
                _ => 0,
            };
            column = column.padding(8).push(item.view(
                inventory_item,
                ability_scores.clone(),
                proficiency_modifier,
            ))
        }

        column = column.push(Text::new("On Person").size(24));

        for item in on_person {
            let inventory_item = items.get(&item.item.item_name).map(|e| e.clone());

            let proficiency_modifier = match inventory_item.clone() {
                Some(item) => item
                    .proficiency_for(&weapon_proficiencies)
                    .modifier_for_bonus(bonus),
                _ => 0,
            };
            column = column.padding(8).push(item.view(
                inventory_item,
                ability_scores.clone(),
                proficiency_modifier,
            ))
        }

        column
    }
}

impl InventoryItemState {
    pub fn apply_all(&mut self, effects: &Vec<Effect>) {
        for effect in effects {
            self.apply(effect.clone())
        }
    }

    pub fn apply(&mut self, effect: Effect) {
        match effect {
            Effect::Damage { damage, scope } => match scope {
                DamageRollScope::Attack => self
                    .damage_bonus
                    .push((FeaturePath::empty().with_include_children(true), damage)),
                DamageRollScope::Feature(path) => match path.matches(self.item.name.clone()) {
                    (true, remaining_path) => self.damage_bonus.push((remaining_path, damage)),
                    (false, _) => {}
                },
                _ => {}
            },
            Effect::Check { bonus, roll } => match roll {
                CheckRollType::Attack => self
                    .attack_bonus
                    .push((FeaturePath::empty().with_include_children(true), bonus)),
                CheckRollType::Feature(path) => match path.matches(self.item.name.clone()) {
                    (true, remaining_path) => self.attack_bonus.push((remaining_path, bonus)),
                    (false, _) => {}
                },
                _ => {}
            },
            _ => {}
        }
    }

    pub fn effects(&self) -> Vec<Effect> {
        let InventoryItemState { features, .. } = self;

        match features {
            Some(f) => f.effects(),
            None => vec![],
        }
    }

    fn persistable(&self) -> InventoryItem {
        self.item.clone()
    }

    fn view<'a>(
        &'a mut self,
        item: Option<Item>,
        ability_scores: ModifiedAbilityScores,
        proficiency_modifier: isize,
    ) -> Column<'a, Message> {
        let item_resource = item;
        let InventoryItemState {
            item,
            features,
            damage_bonus,
            attack_bonus,
        } = self;
        let feature_state = features;
        let InventoryItem {
            name,
            item_name,
            features,
        } = item;
        let mut column = Column::new();

        column = column.push(Row::new().push(Text::new(name.clone())));

        match item_resource {
            Some(item) => {
                column = column.push(item.clone().view());
                for attack in item.attacks(ability_scores).unwrap_or(vec![]) {
                    let mut attack = attack.clone();
                    for (path, damage) in damage_bonus.clone() {
                        match attack.matches(path) {
                            (true, _) => attack = attack.with_extra_damage(damage),
                            _ => {}
                        }
                    }

                    let mut check_bonuses = vec![];

                    for (path, check) in attack_bonus.clone() {
                        match attack.matches(path) {
                            (true, _) => check_bonuses.push(check),
                            _ => {}
                        }
                    }
                    check_bonuses.push(CheckBonus::Modifier(proficiency_modifier));
                    attack = attack.with_extra_check(CheckRoll::from(check_bonuses));
                    column = column.push(attack.view())
                }
            }
            None => {}
        }
        match feature_state {
            Some(features) => {
                column = column.push(
                    features.view(FeaturePath::of(vec![item.name.clone()]), &Message::Feature),
                )
            }
            None => {}
        }

        column
    }
}
