use super::spell_slot::SpellSlot;
use super::State;
use crate::character::class::{Class, Classes};
use crate::character::description::Description;
use crate::character::hitpoints::HitPoints;
use crate::character::inventory::Inventory;
use crate::character::name::Name;
use crate::character::proficiencies::Proficiencies;
use crate::character::saving_throw::SavingThrows;
use crate::character::spell_slot::SpellSlotsState;
use crate::character::spellcasting::Spellcasting;
use crate::core::ability_score::AbilityScores;
use crate::core::feature::{Feature, FeatureState, FeaturesState};
use crate::resources::{ResourceError, Resources};
use crate::store::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum LoadError {
    Store(crate::store::StoreError),
    Serialize(String),
    Resource(ResourceError),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialOrd, PartialEq)]
pub struct CharacterPersistenceConfig {
    storage_root: String,
    character_id: String,
}

#[derive(Debug, Default, Clone)]
pub struct LoadData {
    resources: Resources,
    character: CharacterPersistence,
}

impl LoadData {
    pub fn to_state(self) -> State {
        let CharacterPersistence {
            name,
            description,
            ability_scores,
            config,
            classes,
            hit_points,
            proficiencies,
            saving_throws,
            spellcasting,
            spell_slots,
            inventory,
            features,
        } = self.character;

        State {
            name: name,
            description: description,
            ability_scores: ability_scores,
            config: config,
            classes: Classes::from(classes),
            hit_points: hit_points.to_state(),
            saving_throws: saving_throws,
            proficiencies: proficiencies,
            spellcasting: spellcasting,
            spell_slots: SpellSlotsState::from(spell_slots),
            inventory: inventory.to_state(),
            features: FeaturesState::from(features),
            resources: self.resources,
            ..State::default()
        }
    }
}

impl CharacterPersistenceConfig {
    pub fn storage_root(&self) -> String {
        self.storage_root.clone()
    }

    pub async fn load(self) -> Result<LoadData, LoadError> {
        let resource = crate::resources::load(self.storage_root());
        let character = CharacterPersistence::load(self);
        let resources = resource.await.map_err(LoadError::Resource)?;
        let character = character.await?;
        Ok(LoadData {
            resources,
            character,
        })
    }
}

impl CharacterPersistenceConfig {
    pub fn new(storage_root: String, character_id: String) -> CharacterPersistenceConfig {
        CharacterPersistenceConfig {
            storage_root,
            character_id,
        }
    }

    fn store(&self) -> Result<Store, LoadError> {
        Store::new(self.storage_root.clone()).map_err(|e| LoadError::Store(e))
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
    spellcasting: Vec<Spellcasting>,
    spell_slots: Vec<SpellSlot>,
    inventory: Inventory,
    features: Vec<Feature>,
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
        spellcasting: Vec<Spellcasting>,
        spell_slots: Vec<SpellSlot>,
        inventory: Inventory,
        features: Vec<Feature>,
        config: CharacterPersistenceConfig,
    ) -> CharacterPersistence {
        CharacterPersistence {
            name,
            description,
            ability_scores,
            classes,
            hit_points,
            saving_throws,
            proficiencies,
            spellcasting,
            spell_slots,
            inventory,
            features,
            config,
        }
    }

    fn default_from(config: CharacterPersistenceConfig) -> CharacterPersistence {
        CharacterPersistence {
            config: config,
            ..CharacterPersistence::default()
        }
    }

    pub async fn load(
        config: CharacterPersistenceConfig,
    ) -> Result<CharacterPersistence, LoadError> {
        let store = config.store()?;
        let key = CharacterPersistence::key(config.character_id.clone());
        match store
            .load(key)
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
        let key = CharacterPersistence::key(self.config.character_id.clone());
        let json =
            serde_json::to_string_pretty(&self).map_err(|e| LoadError::Serialize(e.to_string()))?;

        let store = self.config.store()?;

        store.save(key, json).await.map_err(|e| LoadError::Store(e))
    }

    fn key(character_id: String) -> String {
        format!("characters/{}.json", character_id)
    }
}
