//! How other floors of a building are drawn from an upper one (`dungeon_map`'s
//! other floors): numbers the debug menu's Other Floors panel can change while
//! playing, to tune them by eye.

use bevy::prelude::*;

/// How the floors below are drawn. Starts with the tuned values; the debug
/// menu's Other Floors panel changes it for this run only.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct OtherFloorLooks {
    /// How much darker each storey down is in full daylight (0 none, 1 black).
    pub day_dark: f32,
    /// The same at night, with no light outside.
    pub night_dark: f32,
    /// How far from the player (cells, straight line) the ground floor's
    /// outside is drawn. The server sends it no further than
    /// `shared::floor_sight::GROUND_REACH`.
    pub ground_range: f32,
    /// How tall a storey is, in walls' heights: a wall and a floor's thickness.
    pub storey: f32,
    /// Whether remembered slices (seen before, out of sight now) are drawn, in
    /// grey; off only to compare.
    pub remembered: bool,
}

impl OtherFloorLooks {
    pub const DAY_DARK: f32 = 0.15;
    pub const NIGHT_DARK: f32 = 0.45;
    pub const GROUND_RANGE: f32 = 24.0;
    pub const STOREY: f32 = 1.2;
}

impl Default for OtherFloorLooks {
    fn default() -> Self {
        Self {
            day_dark: Self::DAY_DARK,
            night_dark: Self::NIGHT_DARK,
            ground_range: Self::GROUND_RANGE,
            storey: Self::STOREY,
            remembered: true,
        }
    }
}
