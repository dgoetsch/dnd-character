use crate::core::feature::Feature;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Templates {
    #[serde(default)]
    features: HashMap<String, Feature>,
}

impl Templates {
    pub fn new(features: HashMap<String, Feature>) -> Templates {
        Templates { features }
    }

    pub fn features(&self) -> &HashMap<String, Feature> {
        &self.features
    }
}
