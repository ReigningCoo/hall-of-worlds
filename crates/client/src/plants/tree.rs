//! Trees' models (woodcutting plan PR 3): every wild kind (`shared::tree`) in
//! the fruit trees' style, by its crown's shape, bark and leaves
//! (`docs/context/research-trees.md`), at each stage a felled one grows back
//! through (sapling, young, half grown) and grown; stumps for wild and fruit
//! trees alike, and felled fruit trees growing back (their own models, smaller).
//! A few variants each, kept from stage to stage, so a tree grows rather than
//! changes. Black outlines as every model has, and a sepia copy for memory.
//! Built once.

use std::collections::HashMap;
use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::fruit::{FruitDef, FruitStage};
use shared::rng::{Rng, Seed};
use shared::tree::{Tree, TreeKind, TreeStage};

use super::fruit::{build_sized, fruit_plant_size};
use super::models::VARIANTS;
use super::pieces::Pieces;
use super::plan::{WOOD, between, colour};
use super::triangles::Triangles;
use crate::common::{lightness, sepia_linear};

mod broadleaf;
mod conifer;
mod kinds;
mod look;
mod stump;

use kinds::{Crown, kind_look};
use look::{Look, drawn_metres, stage_size, trunk_metres};

/// For the trees' own random numbers: the same everywhere, so every client
/// draws a tree alike.
const LOOKS: Seed = Seed::new(0x7472_6565_7321);

/// What a tree's model is of.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TreeModel {
    /// A wild tree of a kind, as grown back to a stage (`Grown` standing), in
    /// a variant.
    Wild(TreeKind, TreeStage, u8),
    /// A felled fruit tree (by its fruit's key) grown back to a stage (stump
    /// to half grown), in a variant.
    Fruit(String, TreeStage, u8),
}

/// A tree model's meshes: the tree, its black outline (drawn inside out), and
/// the tree in sepia for where it is remembered.
#[derive(Clone, Debug)]
pub struct TreeMeshes {
    pub fill: Handle<Mesh>,
    pub outline: Handle<Mesh>,
    pub memory: Handle<Mesh>,
}

/// Tree models built so far; `None` for nothing to draw (cleared ground).
#[derive(Resource, Default)]
pub struct TreeModels {
    built: HashMap<TreeModel, Option<TreeMeshes>>,
}

impl TreeModels {
    /// `model`, built if it hasn't been; `None` if there is nothing to draw.
    pub fn get(&mut self, model: &TreeModel, meshes: &mut Assets<Mesh>) -> Option<TreeMeshes> {
        self.built
            .entry(model.clone())
            .or_insert_with(|| {
                let pieces = build(model)?;
                Some(TreeMeshes {
                    memory: meshes.add(sepia_copy(&pieces.fill).into_mesh()),
                    fill: meshes.add(pieces.fill.into_mesh()),
                    outline: meshes.add(pieces.outline.into_mesh()),
                })
            })
            .clone()
    }
}

/// How tall a wild tree of `kind` grown back to `stage` is drawn at most,
/// and how far it reaches from its middle, in world units: what a click on it
/// may land on. `None` while cleared.
pub fn tree_size(kind: TreeKind, stage: TreeStage) -> Option<(f32, f32)> {
    let looks = kind_look(kind);
    let metre = super::plan::METRE;
    if stage == TreeStage::Stump {
        return Some(stump::stump_size(trunk_metres(&looks) * metre));
    }
    let size = stage_size(stage);
    let (height, width) = drawn_metres(&looks);
    // Crowns' masses and the trees' own differences reach past the drawn
    // measures: by up to about 10% up and 27% out.
    let reach = width / 2.0 * metre * size.sqrt().min(1.0) * 1.3;
    (size > 0.0).then_some((height * metre * size * 1.15, reach))
}

/// Builds `model`, standing on the origin; `None` for nothing.
fn build(model: &TreeModel) -> Option<Pieces> {
    match model {
        TreeModel::Wild(kind, stage, variant) => wild(*kind, *stage, *variant),
        TreeModel::Fruit(key, stage, variant) => {
            let def = shared::fruit::Fruits::built_in().get(key)?;
            fruit(def, *stage, *variant)
        }
    }
}

/// A wild tree of `kind` at `stage`.
fn wild(kind: TreeKind, stage: TreeStage, variant: u8) -> Option<Pieces> {
    let seed = LOOKS.derive(&format!("{kind:?}"));
    let mut rng = Seed::new(seed.at(&[i64::from(variant % VARIANTS)])).rng();
    let looks = kind_look(kind);
    let mut pieces = Pieces::default();
    match stage {
        TreeStage::Cleared => return None,
        TreeStage::Stump => {
            let thick = trunk_metres(&looks) * super::plan::METRE;
            let wood = heartwood(&Tree::Wild(kind));
            stump::stump(&mut pieces, thick, (colour(looks.bark), wood), &mut rng);
            return Some(pieces);
        }
        _ => {}
    }
    let look = Look::new(&looks, stage, &mut rng);
    let build = match look.crown {
        Crown::Spire => conifer::spire,
        Crown::OpenCone => conifer::open_cone,
        Crown::PineTop => conifer::pine_top,
        Crown::Tiers => conifer::tiers,
        Crown::Column => conifer::column,
        Crown::Oval => broadleaf::oval,
        Crown::Dome => broadleaf::dome,
        Crown::Spreading => broadleaf::spreading,
        Crown::Weeping => broadleaf::weeping,
        Crown::Umbrella => broadleaf::umbrella,
        Crown::Shrubby => broadleaf::shrubby,
    };
    build(&mut pieces, &look, &mut rng);
    Some(pieces)
}

/// A felled fruit tree grown back to `stage`: its stump, then its own model
/// smaller (in leaf, without flowers or fruit), then as it grows by itself.
fn fruit(def: &FruitDef, stage: TreeStage, variant: u8) -> Option<Pieces> {
    let (size, shown) = match stage {
        TreeStage::Cleared => return None,
        TreeStage::Stump => {
            let mut rng = Seed::new(LOOKS.derive(&def.key).at(&[0])).rng();
            // As thick as the fruit trees' own trunks (`fruit::tree`).
            let thick = (fruit_plant_size(def).0 / 1.15 * 0.045).clamp(0.3, 1.6);
            let wood = heartwood(&Tree::Fruit(def.key.clone()));
            let mut pieces = Pieces::default();
            stump::stump(&mut pieces, thick, (WOOD, wood), &mut rng);
            return Some(pieces);
        }
        TreeStage::Sapling => (0.22, FruitStage::Young),
        TreeStage::Young => (0.42, FruitStage::Young),
        TreeStage::HalfGrown => (0.7, FruitStage::Leafy),
        TreeStage::Grown => (1.0, FruitStage::Leafy),
    };
    Some(build_sized(def, shown, variant, Some(size)))
}

/// The heartwood colour of `tree`'s wood, for a stump's cut.
fn heartwood(tree: &Tree) -> Vec3 {
    let wood = tree.wood().and_then(|material| material.wood());
    let [r, g, b] = wood.map_or([200, 160, 110], |wood| wood.heartwood);
    colour(shared::herb::Rgb(r, g, b))
}

/// `fill` in sepia for memory (`common::sepia_linear`): each triangle's
/// lightness kept, so the shape still shows, around the maps' shade of memory.
fn sepia_copy(fill: &Triangles) -> Triangles {
    let mut toned = fill.clone();
    for c in &mut toned.colours {
        let shade = (0.2 + lightness([c[0], c[1], c[2]]) * 0.6).min(0.45);
        let [r, g, b] = sepia_linear(shade);
        *c = [r, g, b, 1.0];
    }
    toned
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
mod tests;
