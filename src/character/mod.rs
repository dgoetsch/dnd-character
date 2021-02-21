use iced::{
    scrollable, Align, Application, Column, Command, Container, Element, HorizontalAlignment,
    Length, Row, Scrollable, Text,
};

use class::Classes;
use description::Description;
use hitpoints::HitPointState;
use name::Name;
use persistence::{CharacterPersistence, CharacterPersistenceConfig, LoadError};
use proficiencies::Proficiencies;
use saving_throw::SavingThrows;

use crate::character::inventory::InventoryState;
use crate::character::persistence::LoadData;
use crate::core::ability_score::{AbilityScores, AbilityScoresState};
use crate::core::feature;
use crate::core::feature::{FeatureMessage, FeatureState, FeaturesState};
use crate::core::feature_path::FeaturePath;
use crate::resources::Resources;

pub mod class;
pub mod description;
pub mod hitpoints;
pub mod inventory;
pub mod name;
pub mod persistence;
pub mod proficiencies;
pub mod saving_throw;
pub mod skill;
//TODO experience, ac, attack

#[derive(Debug)]
pub enum Character {
    Loading(CharacterPersistenceConfig),
    Loaded(State),
}

#[derive(Debug, Clone, Default)]
pub struct State {
    config: CharacterPersistenceConfig,
    resources: Resources,
    name: Name,
    description: Description,
    ability_scores: AbilityScoresState,
    classes: Classes,
    hit_points: HitPointState,
    saving_throws: SavingThrows,
    proficiencies: Proficiencies,
    inventory: InventoryState,
    features: FeaturesState,
    saving: bool,
    dirty: bool,
    scroll: scrollable::State,
}

impl State {
    fn persistable(&self) -> CharacterPersistence {
        CharacterPersistence::from(
            self.name.clone(),
            self.description.clone(),
            self.ability_scores.persistable(),
            self.classes.persistable(),
            self.hit_points.persistable(),
            self.saving_throws.clone(),
            self.proficiencies.clone(),
            self.inventory.persistable(),
            self.features.persistable(),
            self.config.clone(),
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<LoadData, LoadError>),
    Saved(Result<(), LoadError>),
    HitPoint(hitpoints::HitPointMessage),
    Feature(feature::FeatureMessage),
    ResetEffects,
}

impl Application for Character {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = CharacterPersistenceConfig;

    fn new(flags: CharacterPersistenceConfig) -> (Character, Command<Message>) {
        (
            Character::Loading(flags.clone()),
            Command::perform(flags.load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        match self {
            Character::Loaded(state) => "Character".to_string(),
            _ => "Loading...".to_string(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            Character::Loading(_) => match message {
                Message::Loaded(Ok(loaded)) => {
                    *self = Character::Loaded(loaded.to_state());
                    self.update(Message::ResetEffects)
                }
                Message::Loaded(Err(e)) => {
                    println!("Encountered error {:?}", e);
                    *self = Character::Loaded(State::default());
                    Command::none()
                }
                unexpected => {
                    println!(
                        "Encountered unexpected message while loading {:?}",
                        unexpected
                    );
                    Command::none()
                }
            },
            Character::Loaded(state) => {
                match message {
                    Message::ResetEffects => {
                        let mut active_effects = state.features.effects();
                        active_effects.extend(state.inventory.effects_from_equipped());

                        state.ability_scores.apply_all(&active_effects);
                        state.inventory.apply_all(&active_effects);
                        state.features.apply_effects(&active_effects);
                    }
                    Message::Loaded(_) => {}
                    Message::Saved(_) => {
                        state.saving = false;
                    }
                    Message::HitPoint(hit_point_message) => {
                        state.dirty = state.hit_points.update(hit_point_message)
                    }
                    Message::Feature(feature_message) => {
                        state.dirty = state.features.update(feature_message);
                    }
                }

                if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;
                    let persistant_data = state.persistable();
                    Command::perform(persistant_data.save(), Message::Saved)
                } else {
                    Command::none()
                }
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        match self {
            Character::Loading(_) => loading(),
            Character::Loaded(State {
                config,
                resources,
                name,
                description,
                ability_scores,
                classes,
                hit_points,
                saving_throws,
                proficiencies,
                inventory,
                features,
                saving,
                dirty,
                scroll,
            }) => {
                let modified_ability_scores = &ability_scores.modified();

                let name = name.view().padding(4);
                let description = description.view().padding(4);

                let saving_throws = saving_throws
                    .view(
                        &modified_ability_scores,
                        proficiencies.saving_throws(),
                        classes.clone(),
                    )
                    .padding(4);

                let skill_view = skill::view(
                    resources.skills(),
                    proficiencies.skills(),
                    classes.clone(),
                    modified_ability_scores.clone(),
                );

                let features = features.view(
                    FeaturePath::empty(),
                    &modified_ability_scores.ability_scores(),
                    classes,
                    &Message::Feature,
                );

                let ability_scores = ability_scores.view().padding(4);

                let inventory = inventory.view(
                    resources.items().clone(),
                    modified_ability_scores,
                    proficiencies,
                    classes,
                );

                let proficiencies = proficiencies.view().padding(4);
                let classes = classes.view().padding(4);

                let hp_view = hit_points
                    .view()
                    .max_width(800)
                    .spacing(20)
                    .padding(20)
                    .width(Length::FillPortion(1));

                let layout = Column::new()
                    .align_items(Align::Start)
                    .push(Row::new().push(name))
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(
                                Column::new()
                                    .push(description)
                                    .push(classes)
                                    .push(ability_scores)
                                    .width(Length::FillPortion(1)),
                            )
                            .push(
                                Column::new()
                                    .push(proficiencies)
                                    .push(saving_throws)
                                    .width(Length::FillPortion(1)),
                            ),
                    )
                    .push(Row::new().push(hp_view))
                    .push(
                        Row::new()
                            .spacing(8)
                            .push(inventory.width(Length::FillPortion(1)))
                            .push(skill_view.width(Length::FillPortion(1))),
                    )
                    .push(features);

                Scrollable::new(scroll)
                    .padding(40)
                    .push(Container::new(layout).width(Length::Fill).center_x())
                    .into()
            }
        }
    }
}

fn loading<'a>() -> Element<'a, Message> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}
