use crate::resources::template::Templates;
use crate::resources::{ResourceError, Resources};
use crate::store::Store;

pub struct ResourcePersistence {
    templates: Templates,
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
    pub fn from(templates: Templates) -> ResourcePersistence {
        ResourcePersistence { templates }
    }

    pub async fn load(
        config: ResourcePersistenceConfig,
    ) -> Result<ResourcePersistence, ResourceError> {
        let store = config.store()?;

        let template_key = "template.json".to_string();
        let templates: Templates = store
            .load(template_key)
            .await
            .map_err(ResourceError::Store)
            .and_then(|content| {
                serde_json::from_str(content.as_str())
                    .map_err(|e| ResourceError::Serialize(e.to_string()))
            })?;

        Ok(ResourcePersistence { templates })
    }

    pub fn resources(self) -> Resources {
        Resources {
            templates: self.templates,
        }
    }
}
