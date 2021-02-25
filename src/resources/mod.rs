use crate::core::ability_score::Ability;
use crate::resources::skill::Skill;
use crate::resources::template::Templates;
use serde::{Deserialize, Serialize};

mod persistence;
pub mod skill;
pub mod template;

#[derive(Debug, Clone, Default)]
pub struct Resources {
    skills: Vec<Skill>,
    templates: Templates,
}

impl Resources {
    pub fn skills(&self) -> Vec<Skill> {
        self.skills.clone()
    }
    pub fn templates(&self) -> &Templates {
        &self.templates
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
