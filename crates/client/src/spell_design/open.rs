//! Opening and closing the panel: the Magic ability opens it (with the
//! Character panel, where the spellbook is) or closes it; so does its close
//! button; leaving the game closes it. A spell taken out of the book to
//! change (`EditSpell`) is put on it, opening it if closed. The design on it
//! is kept for the next time.

use bevy::prelude::*;
use shared::ability::Ability;

use super::draft::Draft;
use super::markup::SPELL_DESIGN;
use super::rows::{PiecesRow, StepsRow};
use super::{EditSpell, OpenSpellDesign};
use crate::common::{OpenPanels, Session};
use crate::dom::{UiEvent, UiValue, document, top_strip};
use crate::inventory::OpenCharacter;
use crate::widgets::UiId;

pub(super) fn open_panel(
    (mut asked, mut edits): (MessageReader<OpenSpellDesign>, MessageReader<EditSpell>),
    mut draft: ResMut<Draft>,
    mut open_character: MessageWriter<OpenCharacter>,
    mut commands: Commands,
) {
    let mut toggle = asked.read().count() % 2 == 1;
    if let Some(EditSpell(kept)) = edits.read().last() {
        draft.load(kept.clone());
        // Opened whatever else was asked.
        toggle = !draft.is_open();
    }
    if !toggle {
        return;
    }
    if let Some(document) = draft.document.take() {
        commands.entity(document).despawn();
        return;
    }
    let document = commands
        .spawn(document(&SPELL_DESIGN))
        .insert(top_strip())
        .id();
    draft.document = Some(document);
    open_character.write(OpenCharacter);
}

/// Marks the built canvases.
pub(super) fn bind_panel(mut commands: Commands, new: Query<(Entity, &UiId), Added<UiId>>) {
    for (entity, id) in &new {
        let ("spell_design", name) = id.parts() else {
            continue;
        };
        match name {
            "pieces" => commands.entity(entity).insert(PiecesRow),
            "steps" => commands.entity(entity).insert(StepsRow),
            _ => continue,
        };
    }
}

/// Forgets the panel once its close button has closed it, and closes it on
/// leaving the game.
pub(super) fn notice_closed(
    mut events: MessageReader<UiEvent>,
    mut draft: ResMut<Draft>,
    session: Res<Session>,
    mut commands: Commands,
) {
    let closed = (events.read()).any(|event| {
        event.id == UiId::new("spell_design", "panel") && event.value == UiValue::Closed
    });
    let Some(document) = draft.document else {
        return;
    };
    if closed || !session.is_playing() {
        commands.entity(document).despawn();
        draft.document = None;
        draft.naming = false;
    }
}

/// Says whether the panel is open, for Magic's look on the action bar and the
/// Abilities page.
pub(super) fn tell_open(draft: Res<Draft>, mut panels: ResMut<OpenPanels>) {
    OpenPanels::tell(&mut panels, Ability::Magic, draft.document.is_some());
}
