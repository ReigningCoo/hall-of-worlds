//! Personal map marks (owner's design, `shared::map_mark`): keeps
//! `common::MapMarks` to what the server says, and opens the full map's popups
//! for placing a mark (icons and colours) and for one of the player's marks
//! (Travel Here, Remove) from `minimap::FullMapClick`. `minimap` draws them.

use bevy::prelude::*;

#[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
mod debug_api;
mod pictures;
mod popup;
mod press;
mod read;
mod words;

pub use popup::MarkPopup;

use crate::common::MapMarks;

pub struct MapMarksPlugin;

impl Plugin for MapMarksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapMarks>()
            .init_resource::<MarkPopup>()
            .init_resource::<press::JustClosed>()
            .init_resource::<pictures::MarkPictures>()
            .add_systems(Startup, pictures::make_pictures)
            .add_systems(
                Update,
                (
                    read::read_server,
                    pictures::recolour_icons,
                    press::press_popup,
                    press::open_on_click,
                    popup::show_popup,
                )
                    .chain(),
            );
        #[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
        debug_api::add(app);
    }
}

#[cfg(test)]
mod tests;
