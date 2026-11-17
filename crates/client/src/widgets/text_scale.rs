//! Lays text out again whenever the scale it is drawn at changes, so all of it
//! follows the interface's size (`UiScale`) and the window's.
//!
//! Bevy measures text at the scale its entity holds
//! (`ComputedUiRenderTargetInfo`), but text spawned under an interface already
//! on show (rows copied from a template, labels set at run time) gets the
//! scale passed down to it a frame late: its first measure is at 1. Bevy only
//! measures again when the scale differs from its node's, and the node takes
//! its scale from the root, which was right all along, so such text stayed
//! drawn at its unscaled size for good. Marking the text changed when its
//! scale arrives (or changes later) has Bevy measure it again.

use bevy::prelude::*;
use bevy::ui::ComputedUiRenderTargetInfo;
use bevy::ui::UiSystems;
use bevy::ui::widget::measure_text_system;

pub(super) fn add(app: &mut App) {
    app.add_systems(
        PostUpdate,
        follow_scale
            .in_set(UiSystems::Content)
            .after(UiSystems::Propagate)
            .before(measure_text_system),
    );
}

/// Marks text whose scale has changed since it was last looked at as changed,
/// so it is measured again at the new one.
pub(super) fn follow_scale(mut texts: Query<&mut Text, Changed<ComputedUiRenderTargetInfo>>) {
    for mut text in &mut texts {
        text.set_changed();
    }
}

#[cfg(test)]
mod tests;
