use iced::{Application, Settings};

mod character;
mod store;
pub mod util;

use character::persistence::CharacterPersistenceConfig;
use character::Character;

fn main() {
    match Character::run(Settings::with_flags(CharacterPersistenceConfig::new(
        "store/.characters/.vynne".to_string(),
    ))) {
        Ok(_) => println!("Exited Successfully"),
        Err(e) => panic!("An error caused the application to crash {}", e),
    }
}
