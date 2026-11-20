//! Herb patches: each a few plants of its herb (`plants`' models), at the
//! stage it has grown to by the world's clock (`HerbPatch::stage`), in the
//! wilderness and in herbalists' beds, standing on the ground; and picking
//! them by clicking (or tapping) them.
//!
//! Only patches in sight are drawn, never from memory: they will have grown
//! since. Each patch is an entity under the scene with a child per plant, so
//! a plant model is built once and drawn wherever it grows.

use bevy::prelude::*;
use shared::herb::STAGES;

use super::sight_lines::SurfaceMaterial;

mod draw;
mod pick;
mod showcase;

pub(super) use draw::{DrawnPatches, update_herbs};
pub(super) use pick::{HerbHeading, aim_at_herbs, beside_trunk, pick_on_arrival};

/// How many stages the Drawing States panel's shift goes through before
/// everything looks as it is again.
pub const HERB_STAGES: u32 = STAGES as u32;

/// The plants' materials: their colours (tinted by the outside light), and
/// their black outlines, drawn inside out; and glowing fungi's colours, never
/// tinted, so they shine in the dark.
#[derive(Resource)]
pub(super) struct HerbMaterials {
    pub(super) fill: Handle<SurfaceMaterial>,
    pub(super) glow: Handle<SurfaceMaterial>,
    pub(super) outline: Handle<SurfaceMaterial>,
}

/// The stage drawn for one grown to `stage` (1 to `STAGES`), moved on `shift`
/// stages to look at them, going round.
pub(super) fn shown_stage(stage: u8, shift: u32) -> u8 {
    let moved = (u32::from(stage) - 1 + shift) % HERB_STAGES;
    moved as u8 + 1
}

#[cfg(test)]
mod tests;
