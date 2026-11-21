//! Opening and closing the panel: the Cook ability opens it (with the
//! Character panel on its Inventory, where the ingredients are dragged from)
//! or closes it; so does its close button; leaving the game closes it. The
//! process on it is kept for the next time.

use bevy::prelude::*;

use super::OpenCooking;
use super::bench::Bench;
use super::grid::{CultureCanvas, GridCanvas, ToolRow};
use super::markup::COOKING;
use super::side::ProductSlot;
use crate::common::Session;
use crate::dom::{SlotShown, UiEvent, UiValue, document, top_strip};
use crate::inventory::OpenCharacter;
use crate::widgets::UiId;

pub(super) fn open_panel(
    mut asked: MessageReader<OpenCooking>,
    mut bench: ResMut<Bench>,
    mut open_character: MessageWriter<OpenCharacter>,
    mut commands: Commands,
) {
    if asked.read().count() == 0 {
        return;
    }
    if let Some(document) = bench.document.take() {
        commands.entity(document).despawn();
        return;
    }
    let document = commands.spawn(document(&COOKING)).insert(top_strip()).id();
    bench.document = Some(document);
    open_character.write(OpenCharacter);
}

/// Marks the built canvases and the product's frame.
pub(super) fn bind_panel(mut commands: Commands, new: Query<(Entity, &UiId), Added<UiId>>) {
    for (entity, id) in &new {
        let ("cooking", name) = id.parts() else {
            continue;
        };
        match name {
            "tools" => commands.entity(entity).insert(ToolRow),
            "grid" => commands.entity(entity).insert(GridCanvas),
            "culture_space" => commands.entity(entity).insert(CultureCanvas),
            "product" => commands
                .entity(entity)
                .insert((ProductSlot, SlotShown::default())),
            _ => continue,
        };
    }
}

/// Forgets the panel once its close button has closed it, and closes it on
/// leaving the game.
pub(super) fn notice_closed(
    mut events: MessageReader<UiEvent>,
    mut bench: ResMut<Bench>,
    session: Res<Session>,
    mut commands: Commands,
) {
    let closed = (events.read())
        .any(|event| event.id == UiId::new("cooking", "panel") && event.value == UiValue::Closed);
    let Some(document) = bench.document else {
        return;
    };
    if closed || !session.is_playing() {
        commands.entity(document).despawn();
        bench.document = None;
    }
}
