//! Fruit plants' stand-in models (fruit plan PR 3; PR 4 builds the real ones):
//! one simple shape per plant family (a trunk and a crown for trees, a low
//! mound for the rest), small while young, with pale flowers when flowering
//! and the fruit's own colour, unripe or ripe, when fruiting. Black outlines
//! as every model has. Built once per fruit and stage.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::fruit::{FruitDef, FruitStage, PlantFamily};

use super::models::PlantMeshes;
use super::pieces::Pieces;
use super::plan::{METRE, colour};

/// Leaves, bark, flowers and a cactus's skin.
const LEAF: Vec3 = Vec3::new(0.06, 0.25, 0.05);
const BARK: Vec3 = Vec3::new(0.18, 0.1, 0.05);
const FLOWER: Vec3 = Vec3::new(0.95, 0.85, 0.88);
const CACTUS: Vec3 = Vec3::new(0.1, 0.3, 0.12);

/// A family's measures in metres: its height, its crown's (or mound's) width,
/// and how far up the crown starts (0 for no trunk).
fn measures(family: PlantFamily) -> (f32, f32, f32) {
    match family {
        PlantFamily::BroadleafTree => (2.5, 1.9, 1.0),
        PlantFamily::Conifer => (2.7, 1.4, 0.6),
        PlantFamily::Palm => (2.7, 1.9, 2.2),
        PlantFamily::GiantHerb => (2.3, 1.5, 1.2),
        PlantFamily::ColumnCactus => (2.3, 0.5, 0.0),
        PlantFamily::Shrub => (1.4, 1.2, 0.0),
        PlantFamily::Canes | PlantFamily::Vine => (1.5, 1.0, 0.0),
        PlantFamily::PadCactus => (1.1, 1.0, 0.0),
        PlantFamily::Rosette => (0.7, 0.9, 0.0),
        PlantFamily::GroundVine => (0.4, 1.2, 0.0),
        PlantFamily::LowMat | PlantFamily::LowHerb => (0.35, 0.8, 0.0),
    }
}

/// Fruit plant models built so far, by fruit, stage and ripeness.
#[derive(Resource, Default)]
pub struct FruitModels {
    built: HashMap<(String, FruitStage), PlantMeshes>,
}

impl FruitModels {
    /// `def`'s plant at `stage`, built if it hasn't been.
    pub fn get(
        &mut self,
        def: &FruitDef,
        stage: FruitStage,
        meshes: &mut Assets<Mesh>,
    ) -> PlantMeshes {
        self.built
            .entry((def.key.clone(), stage))
            .or_insert_with(|| {
                let pieces = build(def, stage);
                PlantMeshes {
                    fill: meshes.add(pieces.fill.into_mesh()),
                    outline: meshes.add(pieces.outline.into_mesh()),
                }
            })
            .clone()
    }
}

/// Builds `def`'s stand-in at `stage`, standing on the origin.
fn build(def: &FruitDef, stage: FruitStage) -> Pieces {
    let (height, width, trunk) = measures(def.plant);
    let grown = if stage == FruitStage::Young {
        0.45
    } else {
        1.0
    };
    let (height, width, trunk) = (
        height * grown * METRE,
        width * grown * METRE,
        trunk * grown * METRE,
    );
    let mut pieces = Pieces::default();
    let cactus = matches!(
        def.plant,
        PlantFamily::ColumnCactus | PlantFamily::PadCactus
    );
    let leaf = if cactus { CACTUS } else { LEAF };
    // The crown (or the whole plant), and where its outside is.
    let middle = Vec3::Y * (trunk + (height - trunk) / 2.0);
    let half = Vec3::new(width / 2.0, (height - trunk) / 2.0, width / 2.0);
    if trunk > 0.0 {
        let radius = 0.06 * height.max(METRE);
        pieces.stem(Vec3::ZERO, middle, (radius, radius * 0.6), BARK);
    }
    pieces.blob(middle, half, leaf);
    let dots = match stage {
        FruitStage::Young | FruitStage::Leafy => return pieces,
        FruitStage::Flowering => FLOWER,
        FruitStage::Fruiting { ripe: false } => colour(def.look.unripe),
        FruitStage::Fruiting { ripe: true } => colour(def.look.ripe),
    };
    // Dots round the crown's outside, a little above its middle.
    let size = Vec3::splat((width * 0.12).max(0.08 * METRE));
    for i in 0..6 {
        let turn = i as f32 * std::f32::consts::TAU / 6.0;
        let out = Vec3::new(turn.cos() * half.x, half.y * 0.3, turn.sin() * half.z) * 0.95;
        pieces.blob(middle + out, size, dots);
    }
    pieces
}

#[cfg(test)]
mod fruit_tests;
