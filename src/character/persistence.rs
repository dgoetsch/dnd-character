use super::spell_slot::SpellSlot;
use super::State;
use crate::character::ability_score::AbilityScores;
use crate::character::class::{Class, Classes};
use crate::character::description::Description;
use crate::character::hitpoints::HitPoints;
use crate::character::name::Name;
use crate::character::proficiencies::Proficiencies;
use crate::character::saving_throw::SavingThrows;
use crate::character::spell_slot::SpellSlotsState;
use crate::store::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum LoadError {
    Store(crate::store::StoreError),
    Serialize(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialOrd, PartialEq)]
pub struct CharacterPersistenceConfig {
    storage_root: String,
}

impl CharacterPersistenceConfig {
    pub fn new(storage_root: String) -> CharacterPersistenceConfig {
        CharacterPersistenceConfig { storage_root }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CharacterPersistence {
    name: Name,
    description: Description,
    ability_scores: AbilityScores,
    classes: Vec<Class>,
    hit_points: HitPoints,
    saving_throws: SavingThrows,
    proficiencies: Proficiencies,
    spell_slots: Vec<SpellSlot>,
    config: CharacterPersistenceConfig,
}

impl CharacterPersistence {
    pub fn from(
        name: Name,
        description: Description,
        ability_scores: AbilityScores,
        classes: Vec<Class>,
        hit_points: HitPoints,
        saving_throws: SavingThrows,
        proficiencies: Proficiencies,
        spell_slots: Vec<SpellSlot>,
        config: CharacterPersistenceConfig,
    ) -> CharacterPersistence {
        CharacterPersistence {
            name: name,
            description: description,
            ability_scores: ability_scores,
            classes: classes,
            hit_points: hit_points,
            saving_throws: saving_throws,
            proficiencies: proficiencies,
            spell_slots: spell_slots,
            config: config,
        }
    }

    fn store(config: &CharacterPersistenceConfig) -> Result<Store, LoadError> {
        Store::new(config.storage_root.clone()).map_err(|e| LoadError::Store(e))
    }

    fn default_from(config: CharacterPersistenceConfig) -> CharacterPersistence {
        CharacterPersistence {
            config: config,
            ..CharacterPersistence::default()
        }
    }

    pub fn to_state(self) -> State {
        State {
            name: self.name,
            description: self.description,
            ability_scores: self.ability_scores,
            config: self.config,
            classes: Classes::from(self.classes),
            hit_points: self.hit_points.to_state(),
            saving_throws: self.saving_throws,
            proficiencies: self.proficiencies,
            spell_slots: SpellSlotsState::from(self.spell_slots),
            ..State::default()
        }
    }

    pub async fn load(
        config: CharacterPersistenceConfig,
    ) -> Result<CharacterPersistence, LoadError> {
        let store = CharacterPersistence::store(&config)?;

        match store
            .load("character.json".to_string())
            .await
            .map_err(|e| LoadError::Store(e))
            .and_then(|content| {
                serde_json::from_str(content.as_str())
                    .map_err(|e| LoadError::Serialize(e.to_string()))
            }) {
            Ok(r) => Ok(r),
            Err(_) => {
                let default = CharacterPersistence::default_from(config);
                default.clone().save().await.map(|_| default)
            }
        }
    }

    pub async fn save(self) -> Result<(), LoadError> {
        let json =
            serde_json::to_string_pretty(&self).map_err(|e| LoadError::Serialize(e.to_string()))?;

        let store = CharacterPersistence::store(&self.config)?;

        store
            .save("character.json".to_string(), json)
            .await
            .map_err(|e| LoadError::Store(e))
    }
}
