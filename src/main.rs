use iced::{Application, Settings};

mod character;
mod core;
mod dimensions;
mod resources;
mod store;
mod util;
use character::persistence::CharacterPersistenceConfig;
use character::Character;

fn main() {
    let character_name = match std::env::var("CHARACTER_NAME") {
        Ok(name) => name,
        _ => "vynne".to_string(),
    };

    match Character::run(Settings::with_flags(CharacterPersistenceConfig::new(
        ".store/".to_string(),
        character_name,
    ))) {
        Ok(_) => println!("Exited Successfully"),
        Err(e) => panic!("An error caused the application to crash {}", e),
    }
}
