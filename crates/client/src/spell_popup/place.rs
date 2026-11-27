//! Keeping the spell popup on the screen once its size is known: past the
//! bottom (the action bar's), it goes above the pointer instead, so as not
//! to cover the picture; past the right edge, it moves in.

use bevy::prelude::*;
use bevy::ui::UiGlobalTransform;
use bevy::window::PrimaryWindow;

use super::popup::{BOX, OFFSET, SpellPopup};
use crate::common::interface_area;
use crate::dom::{Dom, Popup};
use crate::widgets::{UiId, logical_rect};

pub(super) fn keep_on_screen(
    popups: Query<(&UiId, &ComputedNode, &UiGlobalTransform), With<Popup>>,
    popup: Res<SpellPopup>,
    window: Single<&Window, With<PrimaryWindow>>,
    ui_scale: Res<UiScale>,
    mut dom: Dom,
) {
    let Some(pointer) = popup.opened_at() else {
        return;
    };
    let area = interface_area(&window, &ui_scale);
    for (id, node, transform) in &popups {
        if *id != BOX {
            continue;
        }
        let (corner, size) = logical_rect(node, transform);
        if size == Vec2::ZERO {
            continue;
        }
        let mut inside = corner;
        if corner.y + size.y > area.y {
            inside.y = pointer.y - OFFSET.y - size.y;
        }
        let inside = inside.min(area - size).max(Vec2::ZERO).round();
        if inside != corner.round() {
            dom.set("spell_popup/box", "left", &inside.x.to_string());
            dom.set("spell_popup/box", "top", &inside.y.to_string());
        }
    }
}
