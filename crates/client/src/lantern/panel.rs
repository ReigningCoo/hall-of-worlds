//! The fuel panel (owner): opened on a lantern carried, by a double click or
//! double tap on it (`common::OpenLantern`, from `inventory`); it shows the
//! lantern, its oil and how long that lasts, and a space to drop a bottle of
//! oil on (`fill.rs`). One at a time; it closes by its button, Esc, or the
//! lantern no longer being carried.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::{Item, ItemKind};
use shared::potion::Oil;

use super::AT;
use super::markup::LANTERN;
use super::words;
use crate::common::OpenLantern;
use crate::dom::{Dom, SlotShown, SlotThing, UiEvent, UiValue, document, top_strip};
use crate::inventory::Carried;
use crate::widgets::UiId;

/// The panel open now, if any.
#[derive(Resource, Default)]
pub(super) struct LanternPanel(pub(super) Option<Open>);

pub(super) struct Open {
    /// The lantern it is for.
    pub(super) lantern: Id,
    /// Its document.
    document: Entity,
    /// To be drawn again even if `Carried` hasn't changed (just opened, or
    /// its slots just built).
    stale: bool,
}

/// The slot showing the lantern.
#[derive(Component)]
pub(super) struct LampSlot;

/// The space to drop a bottle of oil on.
#[derive(Component)]
pub(super) struct FillSlot;

/// Opens the panel on the lantern asked for, or raises it if open on it.
pub(super) fn open_panel(
    mut asked: MessageReader<OpenLantern>,
    mut panel: ResMut<LanternPanel>,
    mut dom: Dom,
    mut commands: Commands,
) {
    let Some(&OpenLantern(lantern)) = asked.read().last() else {
        return;
    };
    if let Some(open) = &panel.0 {
        if open.lantern == lantern {
            dom.raise("lantern/panel");
            return;
        }
        commands.entity(open.document).try_despawn();
    }
    let document = commands.spawn(document(&LANTERN)).insert(top_strip()).id();
    dom.start_at("lantern/panel", AT);
    panel.0 = Some(Open {
        lantern,
        document,
        stale: true,
    });
}

/// Marks the slots once built.
pub(super) fn bind_panel(
    new: Query<(Entity, &UiId), Added<UiId>>,
    mut panel: ResMut<LanternPanel>,
    mut commands: Commands,
) {
    for (entity, id) in &new {
        let mut slot = commands.entity(entity);
        match id.parts() {
            ("lantern", "lamp") => slot.insert((LampSlot, SlotShown::default())),
            ("lantern", "fill") => slot.insert((FillSlot, SlotShown::default())),
            _ => continue,
        };
        if let Some(open) = &mut panel.0 {
            open.stale = true;
        }
    }
}

/// Closes it by its button, or once the lantern isn't carried.
pub(super) fn close_panel(
    mut events: MessageReader<UiEvent>,
    carried: Res<Carried>,
    mut panel: ResMut<LanternPanel>,
    mut commands: Commands,
) {
    let closed = (events.read())
        .any(|event| event.id.parts() == ("lantern", "panel") && event.value == UiValue::Closed);
    let Some(open) = &panel.0 else {
        return;
    };
    let gone = (carried.0.find(open.lantern)).is_none_or(|item| item.lantern().is_none());
    if closed || gone {
        close(&mut panel, &mut commands);
    }
}

/// Esc closes the panel rather than opening the system menu.
pub(super) fn close_on_escape(
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut panel: ResMut<LanternPanel>,
    mut commands: Commands,
) {
    if panel.0.is_some() && keys.clear_just_pressed(KeyCode::Escape) {
        close(&mut panel, &mut commands);
    }
}

fn close(panel: &mut LanternPanel, commands: &mut Commands) {
    if let Some(open) = panel.0.take() {
        commands.entity(open.document).try_despawn();
    }
}

/// Shows the lantern, its oil and the space to fill it, as `Carried` says.
pub(super) fn show_panel(
    carried: Res<Carried>,
    mut panel: ResMut<LanternPanel>,
    mut lamps: Query<&mut SlotShown, (With<LampSlot>, Without<FillSlot>)>,
    mut fills: Query<&mut SlotShown, With<FillSlot>>,
    mut dom: Dom,
) {
    let Some(open) = &mut panel.0 else {
        return;
    };
    if !open.stale && !carried.is_changed() {
        return;
    }
    let Some(item) = carried.0.find(open.lantern) else {
        return;
    };
    let Some(fount) = item.lantern() else {
        return;
    };
    open.stale = false;
    dom.set_text("lantern/oil", words::oil(fount));
    dom.set_text("lantern/left", words::left(fount));
    dom.set_text("lantern/fill_words", words::fill(fount));
    for mut slot in &mut lamps {
        let thing = Some(shown(item.clone(), false));
        if slot.thing != thing {
            slot.thing = thing;
        }
    }
    for mut slot in &mut fills {
        // A faint bottle of oil, saying what goes here.
        let bottle =
            shared::potion::pressed(Item::new(Id::stand_in(0), ItemKind::Bottle), Oil::Neutral);
        let thing = Some(shown(bottle, true));
        if slot.thing != thing {
            slot.thing = thing;
        }
    }
}

fn shown(item: Item, faded: bool) -> SlotThing {
    SlotThing {
        item,
        count: 1,
        faded,
        pressable: false,
    }
}
