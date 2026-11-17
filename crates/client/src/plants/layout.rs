//! Where a patch's plants stand in its cell, and how each is turned, sized
//! and which variant it is: from the patch's own seed, so patches side by
//! side differ.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::herb::PlantLook;
use shared::rng::Seed;

use super::models::VARIANTS;
use super::plan::{between, plants_in_patch};
use crate::dungeon_map::CELL_SIZE;

/// How far from the cell's middle a patch's plants stand, as a share of a cell.
const RING: f32 = 0.22;

/// How far a plant may stand off its place, as a share of a cell.
const JITTER: f32 = 0.06;

/// One plant of a patch.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlantSpot {
    /// From the cell's middle, in world units east and south.
    pub offset: Vec2,
    /// Turned about the upright, in radians.
    pub turn: f32,
    /// Its size against its model's.
    pub scale: f32,
    pub variant: u8,
}

/// The plants of a patch of `look`, from its `seed`.
pub fn patch_layout(look: &PlantLook, seed: u64) -> Vec<PlantSpot> {
    let mut rng = Seed::new(seed).rng();
    let count = plants_in_patch(look);
    let start = between(&mut rng, 0.0, TAU);
    (0..count)
        .map(|i| {
            let ring = if count == 1 { 0.0 } else { RING };
            let angle = start + i as f32 / count as f32 * TAU;
            let jitter = Vec2::new(
                between(&mut rng, -JITTER, JITTER),
                between(&mut rng, -JITTER, JITTER),
            );
            PlantSpot {
                offset: (Vec2::from_angle(angle) * ring + jitter) * CELL_SIZE,
                turn: between(&mut rng, 0.0, TAU),
                scale: between(&mut rng, 0.9, 1.1),
                variant: (rng.next_u64() % u64::from(VARIANTS)) as u8,
            }
        })
        .collect()
}
