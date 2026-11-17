//! `PlantModels`: every plant model built so far, by herb, stage and variant,
//! as meshes, so each is built once however many patches show it.

use std::collections::HashMap;

use bevy::prelude::*;
use shared::herb::HerbDef;
use shared::rng::Seed;

use super::plan::{Plan, plants_in_patch};
use super::shapes::build;
use crate::common::HerbTable;

/// How many different plants each herb has at each stage: patches pick one
/// for each of their plants.
pub const VARIANTS: u8 = 4;

/// For the plants' own random numbers: the same everywhere, so every client
/// draws a patch alike.
const LOOKS: Seed = Seed::new(0x706c_616e_7473);

/// A plant model's two meshes: the plant, and its black outline (drawn
/// inside out).
#[derive(Clone, Debug)]
pub struct PlantMeshes {
    pub fill: Handle<Mesh>,
    pub outline: Handle<Mesh>,
}

/// Plant models built so far.
#[derive(Resource, Default)]
pub struct PlantModels {
    built: HashMap<(String, u8, u8), PlantMeshes>,
}

impl PlantModels {
    /// Variant `variant` of `def`'s plant at `stage` (1 to 4), built if it
    /// hasn't been. A variant keeps its shape from stage to stage, growing.
    pub fn get(
        &mut self,
        def: &HerbDef,
        stage: u8,
        variant: u8,
        meshes: &mut Assets<Mesh>,
    ) -> PlantMeshes {
        let key = (def.key.clone(), stage, variant % VARIANTS);
        self.built
            .entry(key)
            .or_insert_with(|| {
                let mut rng = Seed::new(LOOKS.derive(&def.key).at(&[i64::from(variant)])).rng();
                let count = plants_in_patch(&def.plant);
                let plan = Plan::new(&def.plant, stage, count, &mut rng);
                let pieces = build(def.plant.shape, &plan, &mut rng);
                PlantMeshes {
                    fill: meshes.add(pieces.fill.into_mesh()),
                    outline: meshes.add(pieces.outline.into_mesh()),
                }
            })
            .clone()
    }
}

/// Forgets every model when the server's herbs change: one may look
/// different now.
pub(super) fn forget_on_new_table(table: Res<HerbTable>, mut models: ResMut<PlantModels>) {
    if table.is_changed() && !table.is_added() {
        models.built.clear();
    }
}
