//! Opening and closing the panel: the Alchemy ability opens it (with the
//! Character panel on its Inventory, where the ingredients are dragged from)
//! or closes it; so does its close button; leaving the game closes it. The
//! process on it is kept for the next time.

use bevy::prelude::*;
use shared::ability::Ability;

use super::OpenAlchemy;
use super::bench::Bench;
use super::markup::ALCHEMY;
use super::rows::{ApparatusRow, ProcessRow};
use super::side::ProductSlot;
use crate::common::{OpenPanels, Session};
use crate::dom::{SlotShown, UiEvent, UiValue, document, top_strip};
use crate::inventory::OpenCharacter;
use crate::widgets::UiId;

pub(super) fn open_panel(
    mut asked: MessageReader<OpenAlchemy>,
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
    let document = commands.spawn(document(&ALCHEMY)).insert(top_strip()).id();
    bench.document = Some(document);
    open_character.write(OpenCharacter);
}

/// Marks the built canvases and the product's frame.
pub(super) fn bind_panel(mut commands: Commands, new: Query<(Entity, &UiId), Added<UiId>>) {
    for (entity, id) in &new {
        let ("alchemy", name) = id.parts() else {
            continue;
        };
        match name {
            "apparatus" => commands.entity(entity).insert(ApparatusRow),
            "process" => commands.entity(entity).insert(ProcessRow),
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
        .any(|event| event.id == UiId::new("alchemy", "panel") && event.value == UiValue::Closed);
    let Some(document) = bench.document else {
        return;
    };
    if closed || !session.is_playing() {
        commands.entity(document).despawn();
        bench.document = None;
    }
}

/// Says whether the panel is open, for the ability's look on the action bar
/// and the Abilities page.
pub(super) fn tell_open(bench: Res<Bench>, mut panels: ResMut<OpenPanels>) {
    OpenPanels::tell(&mut panels, Ability::Alchemy, bench.document.is_some());
}
