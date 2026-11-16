//! What Load (staff reading someone's debug string) keeps: what is pasted, the
//! checkbox, the reported problem, how it went, and the view to set once there.

use bevy::prelude::*;
use shared::debug_report::Camera;

/// Load's side of the panel. What is pasted is kept when the panel closes; the
/// reported problem until the next Load or the panel closes.
#[derive(Resource, Default)]
pub(super) struct Load {
    /// What is in the paste box.
    pub pasted: String,
    /// Also set the world's time (only with World Time's permission).
    pub set_time: bool,
    /// The words of the report last loaded.
    pub reported: Option<String>,
    /// How the last Load went, in words.
    pub status: String,
    /// The camera of the report on its way, set once the server says the
    /// player is there.
    pub camera: Option<Camera>,
    /// What the panel was last made to show, to change it only when it differs.
    pub shown: Option<Shown>,
}

/// What the panel shows of Load.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Shown {
    pub allowed: bool,
    pub may_set_time: bool,
    pub reported: Option<String>,
    pub status: String,
}
