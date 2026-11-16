//! The panel's state: its document, what is typed, and the string shown.

use bevy::prelude::*;
use shared::debug_report::DebugReport;

/// How long typing must stop before the description goes into the string, in
/// seconds (as Connect waits before checking an address).
pub(super) const TYPING_PAUSE: f64 = 0.6;

/// The open panel, if any, and what it shows. What was typed is kept when it
/// closes, so it opens again as it was left.
#[derive(Resource, Default)]
pub(super) struct Panel {
    /// Its document, from when it is opened.
    pub document: Option<Entity>,
    /// The panel itself, once built.
    pub root: Option<Entity>,
    /// What is in the description box.
    pub typed: String,
    /// When it was last typed in, by the real clock: `None` once it is in the
    /// string.
    pub typed_at: Option<f64>,
    /// The description the string holds.
    pub problem: String,
    /// The report the string was made from, with its time left out, to tell
    /// when anything else has changed.
    pub made_from: Option<DebugReport>,
    /// The string shown.
    pub string: String,
}

impl Panel {
    pub fn is_open(&self) -> bool {
        self.document.is_some()
    }

    /// Takes what is typed into the string straight away.
    pub fn take_typed(&mut self) {
        self.typed_at = None;
        self.problem.clone_from(&self.typed);
    }
}
