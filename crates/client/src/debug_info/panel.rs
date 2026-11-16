//! Opening the panel (the bug button), giving it its copy button once built,
//! and forgetting it once closed or no longer playing.

use bevy::prelude::*;

use super::markup::PANEL;
use super::state::Panel;
use crate::common::Session;
use crate::dom::{Dom, UiEvent, UiValue, document};
use crate::widgets::{Pressable, RaisePanel, TextSize, UiId, spawn_copy_button};

/// The bug button's id.
pub(super) const BUTTON: UiId = UiId::new("hud", "debug_info");

/// The copy button's id.
pub(super) const COPY: UiId = UiId::new("debug_info", "copy");

/// Marks the copy button.
#[derive(Component)]
pub(super) struct CopyButton;

/// Opens the panel when the bug button is pressed, or brings it to the front.
pub(super) fn open_panel(
    mut events: MessageReader<UiEvent>,
    mut commands: Commands,
    mut panel: ResMut<Panel>,
    mut raise: MessageWriter<RaisePanel>,
    mut dom: Dom,
) {
    let pressed = |event: &UiEvent| event.id == BUTTON && event.value == UiValue::Pressed;
    if events.read().filter(|event| pressed(event)).count() == 0 {
        return;
    }
    open(&mut commands, &mut panel, &mut raise, &mut dom);
}

/// Opens the panel, or brings it to the front if open.
pub(super) fn open(
    commands: &mut Commands,
    panel: &mut Panel,
    raise: &mut MessageWriter<RaisePanel>,
    dom: &mut Dom,
) {
    if panel.is_open() {
        if let Some(root) = panel.root {
            raise.write(RaisePanel(root));
        }
        return;
    }
    panel.document = Some(commands.spawn(document(&PANEL)).id());
    // What was typed before, as it was left; the string is made afresh.
    dom.set("debug_info/problem", "value", &panel.typed);
    panel.take_typed();
    panel.made_from = None;
}

/// Notes the built panel, to bring it to the front, and puts in the copy button.
pub(super) fn bind_panel(
    new: Query<(Entity, &UiId), Added<UiId>>,
    mut commands: Commands,
    mut panel: ResMut<Panel>,
) {
    for (entity, id) in &new {
        match id.parts() {
            ("debug_info", "panel") => panel.root = Some(entity),
            ("debug_info", "copy_holder") => {
                commands.entity(entity).with_children(|holder| {
                    let node = Node {
                        width: px(34),
                        height: px(34),
                        flex_shrink: 0.0,
                        ..default()
                    };
                    let what = "Copy the debug string";
                    spawn_copy_button(holder, what, TextSize::Medium, node)
                        .insert((CopyButton, COPY, Pressable));
                });
            }
            _ => {}
        }
    }
}

/// Forgets the panel once its close button has closed it, and closes it on
/// leaving the game.
pub(super) fn notice_closed(
    mut events: MessageReader<UiEvent>,
    mut commands: Commands,
    mut panel: ResMut<Panel>,
    session: Res<Session>,
) {
    let closed = |event: &UiEvent| {
        event.id == UiId::new("debug_info", "panel") && event.value == UiValue::Closed
    };
    let closed = events.read().filter(|event| closed(event)).count() > 0;
    let Some(document) = panel.document else {
        return;
    };
    if closed || !session.is_playing() {
        // The rest of the document goes with its panel.
        commands.entity(document).despawn();
        panel.document = None;
        panel.root = None;
    }
}
