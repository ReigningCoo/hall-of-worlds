//! A miller's mill in a town's model: a round tower with a tall cone of a
//! roof, and the lower end of one sail before its door.

use bevy::prelude::*;

use super::{PoiModel, Rgb, WOOD, circle, rectangle};

/// The tower's half width and height, and how far its roof rises: far
/// taller than a house's.
const TOWER_HALF: f32 = 0.75;
const TOWER_HEIGHT: f32 = 1.8;
const ROOF_RISE: f32 = 2.8;

/// The sail's slats: how many, the widest's half width and how high the
/// first is, each one up narrower, so the sail fades out towards the roof.
const SLATS: usize = 4;
const SLAT_HALF: f32 = 0.45;
const FIRST_SLAT: f32 = 1.0;
const SLAT_STEP: f32 = 0.45;

/// A mill standing at `at`, its door to the south, as houses' are.
pub(super) fn mill(model: &mut PoiModel, at: Vec2, walls: Rgb, roof: Rgb) {
    let round = circle(Vec2::splat(TOWER_HALF), 10);
    model.prism(at, &round, (0.0, TOWER_HEIGHT), walls);
    let eaves = circle(Vec2::splat(TOWER_HALF + 0.25), 10);
    model.cone(at, &eaves, TOWER_HEIGHT, ROOF_RISE, roof);
    model.doorway(at.x, at.y + TOWER_HALF, 0.25, 0.9);
    // The sail stands out from the tower, above the door: its stock (the
    // whip) and slats across it.
    let front = at + Vec2::new(0.0, TOWER_HALF + 0.35);
    let top = FIRST_SLAT + SLAT_STEP * SLATS as f32;
    model.prism(
        front,
        &rectangle(Vec2::new(0.06, 0.06)),
        (FIRST_SLAT - 0.3, top),
        WOOD,
    );
    for slat in 0..SLATS {
        let height = FIRST_SLAT + SLAT_STEP * slat as f32;
        let half = SLAT_HALF * (1.0 - slat as f32 / SLATS as f32);
        let shape = rectangle(Vec2::new(half, 0.04));
        model.prism(front, &shape, (height, height + 0.08), WOOD);
    }
}
