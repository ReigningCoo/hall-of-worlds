//! Fruit plants' models (fruit plan PR 4): one builder per plant family, from
//! each fruit's `FruitPlantLook` (its height, spread, leaf and flower colours
//! and sizes, and where its fruit hangs), at each stage: small while young,
//! leafy, flowering, then fruiting in the fruit's unripe and ripe colours.
//! Several variants each, from their own random numbers, which a plant keeps
//! from stage to stage. Black outlines as every model has. Built once.

use std::collections::HashMap;
use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::fruit::{FruitDef, FruitStage, PlantFamily};
use shared::rng::{Rng, Seed};

use super::models::{PlantMeshes, VARIANTS};
use super::pieces::Pieces;
use super::plan::{METRE, between};

mod cactus;
mod conifer;
mod dress;
mod giant;
mod look;
mod low;
mod palm;
mod rosette;
mod shrub;
mod tree;
mod vine;

use dress::Dress;
use look::{Look, Show, drawn_metres};

/// For the plants' own random numbers: the same everywhere, so every client
/// draws a plant alike.
const LOOKS: Seed = Seed::new(0x6672_7569_7473);

/// Fruit plant models built so far, by fruit, stage and variant.
#[derive(Resource, Default)]
pub struct FruitModels {
    built: HashMap<(String, FruitStage, u8), PlantMeshes>,
}

impl FruitModels {
    /// Variant `variant` of `def`'s plant at `stage`, built if it hasn't been.
    pub fn get(
        &mut self,
        def: &FruitDef,
        stage: FruitStage,
        variant: u8,
        meshes: &mut Assets<Mesh>,
    ) -> PlantMeshes {
        let variant = variant % VARIANTS;
        self.built
            .entry((def.key.clone(), stage, variant))
            .or_insert_with(|| {
                let pieces = build(def, stage, variant);
                PlantMeshes {
                    fill: meshes.add(pieces.fill.into_mesh()),
                    outline: meshes.add(pieces.outline.into_mesh()),
                }
            })
            .clone()
    }
}

/// How tall `def`'s plant is drawn grown, and how far it reaches from its
/// middle, in world units: what a click on it may land on.
pub fn fruit_plant_size(def: &FruitDef) -> (f32, f32) {
    let (height, width) = drawn_metres(def);
    (height * METRE * 1.15, width / 2.0 * METRE * 1.1)
}

/// Builds variant `variant` of `def`'s plant at `stage`, standing on the
/// origin.
fn build(def: &FruitDef, stage: FruitStage, variant: u8) -> Pieces {
    let seed = LOOKS.derive(&def.key);
    let mut rng = Seed::new(seed.at(&[i64::from(variant)])).rng();
    let look = Look::new(def, stage, &mut rng);
    let mut dress = Dress {
        look: &look,
        rng: Seed::new(seed.derive("dress").at(&[i64::from(variant)])).rng(),
        sits: false,
    };
    let build = match def.plant {
        PlantFamily::BroadleafTree => tree::tree,
        PlantFamily::Conifer => conifer::conifer,
        PlantFamily::Palm => palm::palm,
        PlantFamily::Shrub => shrub::shrub,
        PlantFamily::Canes => shrub::canes,
        PlantFamily::Vine => vine::vine,
        PlantFamily::GroundVine => vine::ground_vine,
        PlantFamily::LowMat => low::mat,
        PlantFamily::LowHerb => low::herb,
        PlantFamily::GiantHerb => giant::giant,
        PlantFamily::Rosette => rosette::rosette,
        PlantFamily::PadCactus => cactus::pads,
        PlantFamily::ColumnCactus => cactus::column,
    };
    let mut pieces = Pieces::default();
    build(&mut pieces, &look, &mut rng, &mut dress);
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
mod fruit_tests;
