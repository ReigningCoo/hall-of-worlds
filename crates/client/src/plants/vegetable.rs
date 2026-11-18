//! Vegetable plants' models (vegetable plan PR 4): a cell's worth of one
//! vegetable's plants (several small ones in rows, or one big one), each
//! built by its plant shape (`PlantShape`) from its `VegetablePlantLook` and
//! the look of the part picked, at each of the four stages: small, growing,
//! grown (in flower, or with green ears or heads) and full grown with what is
//! picked showing (fruit, pods, ripe ears, a root's shoulder). Several
//! variants each, from their own random numbers, which a plant keeps from
//! stage to stage. Black outlines as every model has. Built once.

use std::collections::HashMap;
use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::rng::{Rng, Seed};
use shared::vegetable::{PlantShape, VegetableDef};

use super::models::{PlantMeshes, VARIANTS};
use super::pieces::Pieces;
use super::plan::between;

mod climber;
mod crop;
mod grass;
mod layout;
mod look;
mod rooted;
mod rosette;
mod tall;
mod upright;
mod water;

use crop::Crop;
use look::Look;

/// For the plants' own random numbers: the same everywhere, so every client
/// draws a field alike.
const LOOKS: Seed = Seed::new(0x7665_6765_7461);

/// Vegetable cell models built so far, by vegetable, stage and variant.
#[derive(Resource, Default)]
pub struct VegetableModels {
    built: HashMap<(String, u8, u8), PlantMeshes>,
}

impl VegetableModels {
    /// Variant `variant` of a cell of `def`'s plants at `stage` (1 to
    /// `herb::STAGES`), built if it hasn't been; `None` at stage 0 (dug up).
    pub fn get(
        &mut self,
        def: &VegetableDef,
        stage: u8,
        variant: u8,
        meshes: &mut Assets<Mesh>,
    ) -> Option<PlantMeshes> {
        if stage == 0 {
            return None;
        }
        let variant = variant % VARIANTS;
        let model = self
            .built
            .entry((def.key.clone(), stage, variant))
            .or_insert_with(|| {
                let pieces = build_cell(def, stage, variant);
                PlantMeshes {
                    fill: meshes.add(pieces.fill.into_mesh()),
                    outline: meshes.add(pieces.outline.into_mesh()),
                }
            });
        Some(model.clone())
    }
}

/// A cell of `def`'s plants at `stage`, in variant `variant`, its middle on
/// the origin.
fn build_cell(def: &VegetableDef, stage: u8, variant: u8) -> Pieces {
    let mut cell = Pieces::default();
    let spots = layout::spots(def, variant);
    let slot = layout::slot_metres(def);
    for (index, spot) in spots.iter().enumerate() {
        let plant = build(def, (stage, slot), (variant, index), true);
        cell.fill.append_moved(&plant.fill, spot.offset, spot.turn);
        cell.outline
            .append_moved(&plant.outline, spot.offset, spot.turn);
    }
    cell
}

/// One plant of `def` at `stage` given `slot` metres of room, the `index`th
/// of its cell in `variant`, standing on the origin; with what shows besides
/// its leaves and stems when `dressed`.
fn build(
    def: &VegetableDef,
    (stage, slot): (u8, f32),
    (variant, index): (u8, usize),
    dressed: bool,
) -> Pieces {
    let seed = LOOKS.derive(&def.key);
    let at = [i64::from(variant), index as i64];
    let mut rng = Seed::new(seed.at(&at)).rng();
    let look = Look::new(def, stage, slot, &mut rng);
    let mut crop = Crop {
        look: &look,
        rng: Seed::new(seed.derive("crop").at(&at)).rng(),
        dressed,
    };
    let build = match def.plant {
        PlantShape::LeavesOverARoot => rooted::root,
        PlantShape::BulbClump => rooted::bulbs,
        PlantShape::LeafyRosette => rosette::rosette,
        PlantShape::Succulent => rosette::succulent,
        PlantShape::AgaveRosette => rosette::agave,
        PlantShape::UprightLeafyHerb => upright::upright,
        PlantShape::Bush => upright::bush,
        PlantShape::GroundVine => climber::ground_vine,
        PlantShape::ClimberOnAPole => climber::pole,
        PlantShape::ClimberOnSticks => climber::sticks,
        PlantShape::GrainGrass | PlantShape::WaterGrass | PlantShape::PaddyGrass => grass::tuft,
        PlantShape::TallGrass => grass::tall,
        PlantShape::WaterPlant => water::water,
        PlantShape::Fern => water::fern,
        PlantShape::BambooGrove => tall::bamboo,
        PlantShape::ThornyTree => tall::thorny_tree,
    };
    let mut pieces = Pieces::default();
    build(&mut pieces, &look, &mut rng, &mut crop);
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

#[cfg(test)]
mod vegetable_tests;
