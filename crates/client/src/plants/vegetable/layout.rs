//! Where a cell's vegetable plants stand: a square of rows, 1 to 3 a side by
//! how wide each grows (grain always 3, a sprawling or climbing one alone),
//! each a little off its place and turned, from the variant's own seed.

use bevy::prelude::*;
use shared::rng::Seed;
use shared::vegetable::{PlantShape, VegetableDef};

use super::super::plan::{METRE, between};
use super::LOOKS;
use super::look::drawn_metres;
use crate::dungeon_map::CELL_SIZE;

/// How far a plant may stand off its place, as a share of its room.
const JITTER: f32 = 0.12;

/// One plant of a cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Spot {
    /// From the cell's middle, in world units.
    pub(super) offset: Vec3,
    /// Turned about the upright, in radians.
    pub(super) turn: f32,
}

/// How many plants stand along each side of a cell.
pub(super) fn per_side(def: &VegetableDef) -> usize {
    match def.plant {
        PlantShape::GrainGrass | PlantShape::WaterGrass | PlantShape::PaddyGrass => 3,
        PlantShape::GroundVine
        | PlantShape::ClimberOnAPole
        | PlantShape::BambooGrove
        | PlantShape::ThornyTree
        | PlantShape::AgaveRosette => 1,
        PlantShape::ClimberOnSticks => 2,
        _ => match drawn_metres(def).1 {
            wide if wide <= 0.55 => 3,
            wide if wide <= 0.85 => 2,
            _ => 1,
        },
    }
}

/// How much room each plant has, in metres across.
pub(super) fn slot_metres(def: &VegetableDef) -> f32 {
    CELL_SIZE / METRE / per_side(def) as f32
}

/// The plants of a cell of `def` in `variant`.
pub(super) fn spots(def: &VegetableDef, variant: u8) -> Vec<Spot> {
    let mut rng = Seed::new(
        LOOKS
            .derive(&def.key)
            .derive("layout")
            .at(&[i64::from(variant)]),
    )
    .rng();
    let side = per_side(def);
    let room = CELL_SIZE / side as f32;
    let place = |i: usize| (i as f32 + 0.5) * room - CELL_SIZE / 2.0;
    let mut spots = Vec::with_capacity(side * side);
    for row in 0..side {
        for column in 0..side {
            let mut jitter = || between(&mut rng, -JITTER, JITTER) * room;
            let offset = Vec3::new(place(column) + jitter(), 0.0, place(row) + jitter());
            spots.push(Spot {
                offset: if side == 1 { Vec3::ZERO } else { offset },
                turn: between(&mut rng, 0.0, std::f32::consts::TAU),
            });
        }
    }
    spots
}
