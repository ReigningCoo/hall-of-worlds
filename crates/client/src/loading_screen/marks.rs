//! Finer marks of the start-up for the web page's figures (`web/loader.js`):
//! around every plugin's `finish` and `cleanup`, around `Startup`, when the
//! first frame has been drawn, and the longest frames while loading. Natively
//! `StartupMarks` does nothing (a `bevy/trace_chrome` trace tells more there).

use bevy::prelude::*;

/// Added to `main`'s app first and again last: plugins' `finish` and `cleanup`
/// run in the order they were added, so these mark when they begin and end.
/// The last one also adds the marking systems.
pub enum StartupMarks {
    First,
    Last,
}

impl Plugin for StartupMarks {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, _app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        match self {
            Self::First => super::page::mark("how-finish-start"),
            Self::Last => {
                super::page::mark("how-finish-done");
                web::add_systems(_app);
            }
        }
    }

    fn cleanup(&self, _app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        if let Self::Last = self {
            super::page::mark("how-cleanup-done");
        }
    }

    // There are two of it.
    fn is_unique(&self) -> bool {
        false
    }
}

#[cfg(target_arch = "wasm32")]
mod web;
