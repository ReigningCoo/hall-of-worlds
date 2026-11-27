//! Spells' popups: resting the pointer (or a finger) on a spell's picture
//! (the action bar's spell slots, the Spells tab, the spell design panel's
//! preview) opens a `hover` popup with its name, cost and what it does step
//! by step, with numbers for the player as they are now, before the target
//! resists (`shared::spell::tip_lines`). See
//! `docs/context/client/spell_popup.md`.

mod place;
mod popup;
mod subject;

use bevy::prelude::*;

use crate::dom::{BuildDocuments, ClosePopups};

pub struct SpellPopupPlugin;

impl Plugin for SpellPopupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<popup::SpellPopup>()
            .add_systems(Update, place::keep_on_screen)
            .add_systems(
                PostUpdate,
                (popup::forget_closed, popup::follow_pointer)
                    .chain()
                    .after(ClosePopups)
                    .before(BuildDocuments),
            );
    }
}

#[cfg(test)]
mod tests;
