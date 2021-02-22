use crate::resources::{ResourceError, Resources, Skill};
use crate::store::Store;

pub struct ResourcePersistence {
    skills: Vec<Skill>,
}
pub struct ResourcePersistenceConfig {
    storage_root: String,
}

impl ResourcePersistenceConfig {
    pub fn of(storage_root: String) -> ResourcePersistenceConfig {
        ResourcePersistenceConfig { storage_root }
    }
    fn store(&self) -> Result<Store, ResourceError> {
        Store::new(self.storage_root.clone()).map_err(|e| ResourceError::Store(e))
    }
}

impl ResourcePersistence {
    pub fn from(skills: Vec<Skill>) -> ResourcePersistence {
        ResourcePersistence { skills }
    }

    pub async fn load(
        config: ResourcePersistenceConfig,
    ) -> Result<ResourcePersistence, ResourceError> {
        let store = config.store()?;
        let skill_key = "skills.json".to_string();
        let skills: Vec<Skill> = store
            .load(skill_key)
            .await
            .map_err(ResourceError::Store)
            .and_then(|content| {
                serde_json::from_str(content.as_str())
                    .map_err(|e| ResourceError::Serialize(e.to_string()))
            })?;

        Ok(ResourcePersistence { skills })
    }

    pub fn resources(self) -> Resources {
        Resources {
            skills: self.skills,
        }
    }
}
