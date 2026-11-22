//! Opening and closing the reading panel: `ReadRecipe` (the Recipes tab,
//! PR 5) or a scroll just read into the book (`RecipeInserted`) opens it on
//! that recipe, or shows that one if it is open already; its close button,
//! or leaving the game, closes it.

use bevy::prelude::*;
use shared::protocol::ServerMessage;
use shared::recipe::Entry;

use super::super::ReadRecipe;
use super::draw::RecipeGrid;
use super::markup::READING;
use crate::common::Session;
use crate::connection::FromServer;
use crate::dom::{Dom, UiEvent, UiValue, document, top_strip};
use crate::widgets::UiId;

/// The reading panel: its document while open, and the recipe on it.
#[derive(Resource, Default)]
pub(in crate::cooking) struct Reading {
    pub(super) document: Option<Entity>,
    pub(super) entry: Option<Entry>,
}

impl Reading {
    pub(in crate::cooking) fn is_open(&self) -> bool {
        self.document.is_some()
    }
}

/// Opens the panel on a recipe asked for or just read from a scroll.
pub(super) fn open_reading(
    mut asked: MessageReader<ReadRecipe>,
    mut from_server: MessageReader<FromServer>,
    mut reading: ResMut<Reading>,
    mut dom: Dom,
    mut commands: Commands,
) {
    let inserted = (from_server.read()).filter_map(|FromServer(message)| match message {
        ServerMessage::RecipeInserted(entry) => Some(entry.clone()),
        _ => None,
    });
    let asked: Vec<Entry> = (asked.read().map(|ReadRecipe(entry)| entry.clone()))
        .chain(inserted)
        .collect();
    let Some(entry) = asked.into_iter().last() else {
        return;
    };
    reading.entry = Some(entry);
    match reading.document {
        Some(_) => dom.raise("recipe/panel"),
        None => {
            let document = commands.spawn(document(&READING)).insert(top_strip()).id();
            reading.document = Some(document);
        }
    }
}

/// Marks the grid's canvas once built.
pub(super) fn bind_reading(mut commands: Commands, new: Query<(Entity, &UiId), Added<UiId>>) {
    for (entity, id) in &new {
        if id.parts() == ("recipe", "grid") {
            commands.entity(entity).insert(RecipeGrid);
        }
    }
}

/// Forgets the panel once its close button has closed it, and closes it on
/// leaving the game.
pub(super) fn notice_closed(
    mut events: MessageReader<UiEvent>,
    mut reading: ResMut<Reading>,
    session: Res<Session>,
    mut commands: Commands,
) {
    let closed = (events.read())
        .any(|event| event.id == UiId::new("recipe", "panel") && event.value == UiValue::Closed);
    let Some(document) = reading.document else {
        return;
    };
    if closed || !session.is_playing() {
        commands.entity(document).despawn();
        reading.document = None;
        reading.entry = None;
    }
}
