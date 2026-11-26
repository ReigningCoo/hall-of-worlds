//! The debug menu: whether it is open, and what is typed in each layer.

use bevy::prelude::*;

use super::{ACTIONS, SUBMENU};

/// What has been typed in one layer's list, which entry the arrow keys are on
/// (counted among those on show), and the first column drawn.
#[derive(Default)]
pub(super) struct Level {
    pub(super) filter: String,
    pub(super) highlight: Option<usize>,
    pub(super) first_column: usize,
}

/// What the submenu lists: Generate POI's kinds, or Play Tune's tunes (their
/// titles, in the order they play).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(in crate::debug_menu) enum Sublist {
    #[default]
    Kinds,
    Tunes(Vec<String>),
}

/// The debug menu: whether it is open, and its lists.
#[derive(Resource, Default)]
pub struct DebugMenu {
    pub open: bool,
    pub(super) actions: Level,
    /// The submenu, if it is open, and what it lists.
    pub(super) submenu: Option<Level>,
    pub(super) sublist: Sublist,
    /// Counting down to closing, after a toggle.
    pub(super) closing: Option<Timer>,
}

impl DebugMenu {
    /// Open, with this already typed (dev tools).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_with(filter: String) -> Self {
        Self {
            open: true,
            actions: Level {
                filter,
                ..default()
            },
            ..default()
        }
    }

    pub(super) fn set_open(&mut self, open: bool) {
        *self = Self { open, ..default() };
    }

    /// Opens the menu at its submenu, listing this.
    pub(in crate::debug_menu) fn open_submenu(&mut self, sublist: Sublist) {
        self.open = true;
        self.submenu = Some(Level::default());
        self.sublist = sublist;
    }

    pub(super) fn top_layer(&self) -> usize {
        if self.submenu.is_some() {
            SUBMENU
        } else {
            ACTIONS
        }
    }

    pub(super) fn level(&self, layer: usize) -> Option<&Level> {
        if layer == ACTIONS {
            Some(&self.actions)
        } else {
            self.submenu.as_ref()
        }
    }
}
