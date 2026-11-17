//! The copy button beside an item's id in its popup, for staff with debug
//! tools (`show.rs` shows the row): put in once the popup is built, as the
//! markup has no copy button, and copying the id when pressed, as
//! `debug_info` copies its string.

use bevy::prelude::*;
use shared::id::Id;

use super::open::FORM;
use crate::widgets::{
    ButtonPressed, Pressable, TextSize, UiId, copy_to_clipboard, spawn_copy_button,
};

/// The copy button in the popup about this item.
#[derive(Component, Clone, Copy, Debug)]
pub(super) struct CopyId(pub(super) Id);

/// Puts a copy button into each new popup's holder beside its id.
pub(super) fn bind_copy(new: Query<(Entity, &UiId), Added<UiId>>, mut commands: Commands) {
    for (entity, id) in &new {
        if id.parts() != (FORM, "copy_holder") {
            continue;
        }
        let Some(of) = id.which() else {
            continue;
        };
        commands.entity(entity).with_children(|holder| {
            let node = Node {
                width: px(28),
                height: px(28),
                flex_shrink: 0.0,
                ..default()
            };
            spawn_copy_button(holder, "Copy the item's id", TextSize::Small, node).insert((
                CopyId(of),
                UiId::of(FORM, of, "copy"),
                Pressable,
            ));
        });
    }
}

/// Copies the id when its button is pressed.
pub(super) fn copy_pressed(mut pressed: MessageReader<ButtonPressed>, buttons: Query<&CopyId>) {
    for press in pressed.read() {
        if let Ok(CopyId(of)) = buttons.get(press.0) {
            copy_to_clipboard(&of.to_string());
        }
    }
}
