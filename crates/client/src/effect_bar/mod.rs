//! The bar of timed effects at the top middle of the window: an icon for each
//! effect on the character that has an end (eating, poison, the debug menu's
//! bonus), shaded more and more from the top as it runs out, with a `hover`
//! popup saying what it does and when it ends. Effects for good (none yet)
//! never show. See `docs/context/client/effect_bar.md`.

mod bar;
mod looks;
mod popup;
mod words;

use bevy::prelude::*;

use crate::common::{ServerTick, take_clock};
use crate::dom::{BuildDocuments, ClosePopups};

pub struct EffectBarPlugin;

impl Plugin for EffectBarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ServerTick>()
            .init_resource::<popup::EffectPopup>()
            .add_systems(Startup, bar::spawn_bar)
            .add_systems(
                Update,
                (
                    take_clock,
                    bar::bind_canvas,
                    bar::draw_icons,
                    bar::show_shades,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                (
                    popup::forget_closed,
                    popup::follow_pointer,
                    popup::count_down,
                )
                    .chain()
                    .after(ClosePopups)
                    .before(BuildDocuments),
            );
    }
}

#[cfg(test)]
mod tests;
