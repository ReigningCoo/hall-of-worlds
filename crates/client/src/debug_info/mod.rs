//! Debug Info: the bug button at the top right (just left of the system
//! menu's gear) opens a moving panel where the player describes a problem,
//! with a debug string beside it to copy into a bug report. The string
//! (`shared::debug_report`) says where they are and what they see: enough for
//! staff to go there. Shown to everyone while playing.

use bevy::prelude::*;

mod button;
#[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
mod debug_api;
mod gather;
mod known;
mod markup;
mod panel;
mod platform;
mod refresh;
mod state;

pub struct DebugInfoPlugin;

impl Plugin for DebugInfoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<state::Panel>()
            .init_resource::<known::Known>()
            .add_systems(Startup, button::spawn_button)
            .add_systems(
                Update,
                (
                    known::note_sent,
                    known::note_told,
                    button::bind_button,
                    button::show_button,
                    panel::notice_closed,
                    panel::open_panel,
                    panel::bind_panel,
                    refresh::read_events,
                    refresh::refresh,
                )
                    .chain(),
            );
        #[cfg(all(feature = "debug-api", not(target_arch = "wasm32")))]
        debug_api::add(app);
    }
}

#[cfg(test)]
mod tests;
