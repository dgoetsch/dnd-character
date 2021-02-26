use crate::resources::template::Templates;

mod persistence;
pub mod template;

#[derive(Debug, Clone, Default)]
pub struct Resources {
    templates: Templates,
}

impl Resources {
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
