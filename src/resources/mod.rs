use crate::character::ability_score::Ability;
use serde::{Deserialize, Serialize};

mod persistence;

#[derive(Debug, Clone, Default)]
pub struct Resources {
    skills: Vec<Skill>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    name: String,
    ability: Ability,
}

impl Skill {
    pub fn of(name: &str, ability: Ability) -> Skill {
        Skill {
            name: name.to_string(),
            ability: ability,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn ability(&self) -> Ability {
        self.ability.clone()
    }
}
