//! A line one screen pixel wide just outside every `object-slot`, in the grid
//! lines' colour. Slots side by side (the clothes) are laid out one by one,
//! and at most interface sizes rounding their places to whole pixels leaves
//! a pixel between some of them where the panel shows through: this covers
//! it. A whole pixel, not a whole point of the interface, so it is never
//! blurred.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::SlotShown;
use crate::widgets::GRID_LINE;

/// Gives each slot its edge, and fits them all again when the interface size
/// or the screen's scale changes.
pub(super) fn fit_slot_edges(
    mut commands: Commands,
    slots: Query<(Entity, Option<&Outline>), With<SlotShown>>,
    ui_scale: Res<UiScale>,
    window: Option<Single<&Window, With<PrimaryWindow>>>,
) {
    let scale = window.map_or(1.0, |window| window.scale_factor());
    let width = px(one_pixel(ui_scale.0, scale));
    for (slot, edge) in &slots {
        if edge.is_none_or(|edge| edge.width != width) {
            commands
                .entity(slot)
                .insert(Outline::new(width, px(0.0), GRID_LINE));
        }
    }
}

/// One screen pixel in the interface's points.
pub(super) fn one_pixel(ui_scale: f32, scale_factor: f32) -> f32 {
    1.0 / (ui_scale * scale_factor).max(0.01)
}

#[cfg(test)]
mod tests;
