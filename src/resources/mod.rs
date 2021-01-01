use crate::character::ability_score::Ability;
use crate::resources::equipment::Equipment;
use crate::resources::skill::Skill;
use serde::{Deserialize, Serialize};

pub mod equipment;
mod persistence;
pub mod skill;

#[derive(Debug, Clone, Default)]
pub struct Resources {
    skills: Vec<Skill>,
    equipment: Vec<Equipment>,
}

impl Resources {
    pub fn skills(&self) -> Vec<Skill> {
        self.skills.clone()
    }
    pub fn equipment(&self) -> Vec<Equipment> {
        self.equipment.clone()
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum ResourceError {
    Store(crate::store::StoreError),
    Serialize(String),
}

pub async fn load(storage_root: String) -> Result<Resources, ResourceError> {
    let persistence = persistence::ResourcePersistence::load(
        persistence::ResourcePersistenceConfig::of(storage_root),
    )
    .await?;

    Ok(persistence.resources())
}
