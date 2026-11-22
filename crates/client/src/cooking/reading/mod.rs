//! The recipe reading panel (recipes plan PR 4, `docs/context/recipes.md`):
//! a moving panel drawn as an unrolled scroll, opened by `ReadRecipe` or on
//! reading a scroll into the book, showing one recipe: the dish's name and
//! where it is from (or a quill and "Your recipe" for the reader's own), its
//! grid as the Cooking panel draws one, read only, the note when it is beyond
//! the reader's Cooking skill, and its steps written out from `recipe.ftl`.
//! What isn't carried, and tools not to hand, are red in the grid and the
//! words.

#[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
mod debug_api;
mod draw;
mod entry;
mod have;
mod markup;
mod open;
mod show;
mod written;

use bevy::prelude::*;

pub(super) use entry::Shown;
pub(super) use have::{foods_named, input_token};
pub(super) use open::Reading;

/// The reading panel's systems, in order.
pub(super) fn add(app: &mut App) {
    app.init_resource::<Reading>().add_systems(
        Update,
        (
            open::open_reading,
            open::bind_reading,
            open::notice_closed,
            show::show_reading.after(super::kitchen::keep_kitchen),
        )
            .chain(),
    );
    #[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
    debug_api::add(app);
}

#[cfg(test)]
mod tests;
