use crate::core::ability_score::Ability;
use serde::{Deserialize, Serialize};

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
