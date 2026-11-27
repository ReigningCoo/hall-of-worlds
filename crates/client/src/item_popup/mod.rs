//! Item popups: resting the pointer (or a finger) on an item anywhere it is
//! drawn opens a `hover` popup (`dom`) with its name in its quality's colour,
//! what it gives (`shared::bonus`, worded through `shared::lang`), its set's
//! pieces and bonuses (`shared::item_set`), what it
//! asks of its wearer (`shared::requirement`), what a key remembers, its weight,
//! its value in coins (`shared::value`), and its id
//! for staff with debug tools. It stays while
//! the pointer is on the item or on it, and goes when the pointer leaves both;
//! its padlock keeps it open. Whoever draws items says which one the pointer
//! is on through `widgets::PointedItem`.

mod about;
mod cask;
mod copy;
mod food;
mod fungus;
mod herb;
mod keys;
mod needs;
mod ointment;
mod open;
mod place;
mod recipe;
mod set;
mod show;
mod size;
mod value;
mod words;
mod yeast;

use bevy::prelude::*;

use crate::dom::BuildDocuments;
use crate::widgets::ItemsPointed;

pub struct ItemPopupPlugin;

impl Plugin for ItemPopupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<open::ItemPopups>()
            .add_systems(Update, (place::keep_on_screen, copy::copy_pressed))
            // Once everything has said what the pointer is on, in time for a
            // new popup to be built this frame.
            .add_systems(
                PostUpdate,
                (open::forget_closed, open::follow_pointed)
                    .chain()
                    .after(ItemsPointed)
                    .before(BuildDocuments),
            )
            .add_systems(
                PostUpdate,
                (show::bind_names, copy::bind_copy)
                    .after(BuildDocuments)
                    .before(bevy::ui::UiSystems::Layout),
            );
    }
}

#[cfg(test)]
mod tests;
