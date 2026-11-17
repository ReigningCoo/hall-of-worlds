//! Building a plant's model by its herb's `PlantShape`, from its `Plan`. Every
//! shape is a handful of stems, leaves and blobs (`Pieces`), placed from the
//! plant's own random numbers. In plant space: the foot at the origin, up `y`.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::herb::PlantShape;
use shared::rng::Rng;

use super::pieces::Pieces;
use super::plan::{Plan, between};

mod creeper;
mod fern;
mod grass;
mod rosette;
mod umbel;
mod upright;
mod vine;
mod woody;

/// The model of one plant shaped as `shape`.
pub fn build(shape: PlantShape, plan: &Plan, rng: &mut Rng) -> Pieces {
    let mut pieces = Pieces::default();
    let build = match shape {
        PlantShape::Rosette => rosette::rosette,
        PlantShape::Succulent => rosette::succulent,
        PlantShape::Spike => upright::spike,
        PlantShape::Clump => upright::clump,
        PlantShape::Bush => woody::bush,
        PlantShape::Tree => woody::tree,
        PlantShape::Umbel => umbel::umbel,
        PlantShape::Vine => vine::vine,
        PlantShape::Grass => grass::grass,
        PlantShape::Aquatic => grass::aquatic,
        PlantShape::Fern => fern::fern,
        PlantShape::Creeper => creeper::creeper,
    };
    build(&mut pieces, plan, rng);
    pieces
}

/// The `i`th of `n` ways round, a little off even.
fn around(i: usize, n: usize, rng: &mut Rng) -> f32 {
    (i as f32 + between(rng, -0.3, 0.3)) / n as f32 * TAU
}

/// Outwards `angle` round, rising `rise` for each unit out.
fn out(angle: f32, rise: f32) -> Vec3 {
    Vec3::new(angle.cos(), rise, angle.sin())
}

/// How thick a stem `height` tall is at its foot.
fn stem_radius(height: f32) -> f32 {
    (height * 0.025).clamp(0.2, 0.6)
}

/// A flower head on top of `at`: a flattened blob in the flower's colour.
fn flower_head(pieces: &mut Pieces, plan: &Plan, at: Vec3, rng: &mut Rng) {
    if let Some(colour) = plan.flower {
        let size = plan.flower_size * between(rng, 0.9, 1.2);
        let turn = between(rng, 0.0, TAU);
        let middle = at + Vec3::Y * size * 0.4;
        pieces.blob_turned(middle, Vec3::new(size, size * 0.55, size), turn, colour);
    }
}

/// A closed bud on top of `at`, for a stage before flowering.
fn bud(pieces: &mut Pieces, plan: &Plan, at: Vec3) {
    let size = plan.flower_size * 0.6;
    pieces.blob(
        at + Vec3::Y * size,
        Vec3::new(size, size * 1.4, size),
        plan.leaf_colour,
    );
}

/// Flowers if it is flowering, a bud if it is full grown, else nothing.
fn top(pieces: &mut Pieces, plan: &Plan, at: Vec3, rng: &mut Rng) {
    if plan.growth.flowers {
        flower_head(pieces, plan, at, rng);
    } else if plan.growth.size >= 1.0 && plan.flower.is_some() {
        bud(pieces, plan, at);
    }
}
