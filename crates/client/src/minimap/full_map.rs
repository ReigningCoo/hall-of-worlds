//! The full map: whether it is open, where it is dragged to and its zoom.

use bevy::prelude::*;

use super::OPENING_ZOOM;

/// The full map: whether it is open, where it has been dragged to, and how
/// far it is zoomed in.
#[derive(Resource)]
pub struct FullMap {
    pub open: bool,
    /// The grid position in its middle; `None` follows the player.
    pub centre: Option<(f64, f64)>,
    /// Its scale against the minimap's.
    pub zoom: f32,
    /// Where the cursor was last frame, while dragging.
    pub(super) drag_from: Option<Vec2>,
    /// The floor just changed to, keeping the map's offset from the player
    /// even if nothing known shows there.
    pub(super) kept_on: Option<u32>,
    /// Where a left press on the picture began, and whether it has moved far
    /// enough to be a drag rather than a click.
    pub(super) press: Option<(Vec2, bool)>,
    /// A click on the picture `take_input` noticed, for `report_click`.
    pub(super) clicked: Option<Vec2>,
}

impl Default for FullMap {
    fn default() -> Self {
        Self {
            open: false,
            centre: None,
            zoom: OPENING_ZOOM,
            drag_from: None,
            kept_on: None,
            press: None,
            clicked: None,
        }
    }
}

impl FullMap {
    /// Opens (or closes) it as it first looks: on the player, at the opening
    /// zoom.
    pub(super) fn set_open(&mut self, open: bool) {
        *self = Self { open, ..default() };
    }
}
