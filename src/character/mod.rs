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
use spell_slot::SpellSlotsState;

use crate::character::inventory::InventoryState;
use crate::character::persistence::LoadData;
use crate::character::spellcasting::Spellcasting;
use crate::core::ability_score::AbilityScores;
use crate::core::feature;
use crate::core::feature::{FeatureMessage, FeatureState, FeaturesState};
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
pub mod spell_slot;
pub mod spellcasting;
//TODO experience, ac

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
    ability_scores: AbilityScores,
    classes: Classes,
    hit_points: HitPointState,
    saving_throws: SavingThrows,
    proficiencies: Proficiencies,
    spellcasting: Vec<Spellcasting>,
    spell_slots: SpellSlotsState,
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
            self.ability_scores.clone(),
            self.classes.persistable(),
            self.hit_points.persistable(),
            self.saving_throws.clone(),
            self.proficiencies.clone(),
            self.spellcasting.clone(),
            self.spell_slots.persistable(),
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
    SpellSlot(spell_slot::SpellSlotMessage),
    Feature(feature::FeatureMessage),
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
            Character::Loading(_) => {
                match message {
                    Message::Loaded(Ok(loaded)) => {
                        *self = Character::Loaded(loaded.to_state());
                    }
                    Message::Loaded(Err(e)) => {
                        println!("Encountered error {:?}", e);
                        *self = Character::Loaded(State::default());
                    }
                    unexpected => {
                        println!(
                            "Encountered unexpected message while loading {:?}",
                            unexpected
                        );
                    }
                }
                Command::none()
            }
            Character::Loaded(state) => {
                match message {
                    Message::Loaded(_) => (),
                    Message::Saved(_) => {
                        state.saving = false;
                    }
                    Message::HitPoint(hit_point_message) => {
                        state.dirty = state.hit_points.update(hit_point_message)
                    }
                    Message::SpellSlot(spell_slot_message) => {
                        state.dirty = state.spell_slots.update(spell_slot_message);
                    }
                    Message::Feature(feature_message) => {
                        state.dirty = state.features.update(feature_message);
                    }
                }
                println!(
                    "Performed message, dirty?:{}, saving?:{}",
                    state.dirty, state.saving
                );

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
                spellcasting,
                spell_slots,
                inventory,
                features,
                saving,
                dirty,
                scroll,
            }) => {
                let name = name.view().padding(4);

                let description = description.view().padding(4);
                let saving_throws = saving_throws
                    .view(
                        ability_scores,
                        proficiencies.saving_throws(),
                        classes.clone(),
                    )
                    .padding(4);

                let skill_view = skill::view(
                    resources.skills(),
                    proficiencies.skills(),
                    classes.clone(),
                    ability_scores.clone(),
                );

                let spellcasting =
                    spellcasting::view(spellcasting.clone(), classes, ability_scores.clone());

                let ability_scores = ability_scores.view().padding(4);
                let proficiencies = proficiencies.view().padding(4);
                let classes = classes.view().padding(4);

                let hp_view = hit_points
                    .view()
                    .max_width(800)
                    .spacing(20)
                    .padding(20)
                    .width(Length::FillPortion(1));

                let spell_slot_view = spell_slots
                    .view()
                    .max_width(800)
                    .spacing(20)
                    .padding(20)
                    .width(Length::FillPortion(2));

                let inventory = inventory.view(resources.items().clone());

                let features = features.view(vec![], Message::Feature);

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
                            .push(
                                Column::new()
                                    .push(spellcasting)
                                    .push(spell_slot_view)
                                    .width(Length::FillPortion(1)),
                            )
                            .push(skill_view.width(Length::FillPortion(1))),
                    )
                    .push(inventory)
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
