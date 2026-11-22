//! The lantern on screen (lantern task PR 6; rules in `shared::lantern`, the
//! server's side in `connections/lantern.rs`): the light round the player
//! while one burns, its oil counted down here between the server's word, and
//! its fuel panel, opened by a double click or double tap on it, with a space
//! to drop a bottle of oil on. Its model and fuel bar are `item_art`'s and
//! `widgets`'; others' light is `entities`'. See
//! `docs/context/client/lantern.md`.

use bevy::prelude::*;

use crate::common::PointerSystems;
use crate::dom::{DomInput, HoldObjects};
use crate::player::PlayerSystems;

// Split into files (`cargo xtask lint` keeps files small).
mod burn;
mod fill;
mod light;
mod markup;
mod panel;
mod words;

use panel::LanternPanel;

/// To the right of where the Character panel opens, so bottles can be dragged
/// across, leaving the player in view.
const AT: Vec2 = Vec2::new(680.0, 120.0);

pub struct LanternPlugin;

impl Plugin for LanternPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LanternPanel>()
            .init_resource::<burn::Activity>()
            .add_systems(
                PreUpdate,
                (
                    panel::close_on_escape
                        .after(PointerSystems)
                        .before(DomInput::Front),
                    fill::drop_oil.after(HoldObjects),
                ),
            )
            .add_systems(
                Update,
                (
                    (burn::note_activity, burn::burn_locally).chain(),
                    light::follow_lantern.after(burn::burn_locally),
                    (
                        panel::open_panel,
                        panel::bind_panel,
                        panel::close_panel,
                        panel::show_panel,
                    )
                        .chain()
                        .after(PlayerSystems::Travel),
                ),
            );
    }
}

#[cfg(test)]
mod tests;
