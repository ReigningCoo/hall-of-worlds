//! `Look`: one wild tree's drawn measures in world units and its colours, from
//! its kind's looks, the stage it has grown back to and its own differences.
//!
//! Sizes are squeezed as fruit trees' are (owner's: a camera looking down must
//! still see round them): from 2.2 m for the smallest to 4.6 m for the
//! tallest, crowns about a cell or two across, so neighbours in a wood touch.

use bevy::prelude::*;
use shared::rng::Rng;
use shared::tree::TreeStage;

use super::super::plan::{METRE, between, colour};
use super::kinds::{Crown, KindLook};

/// The shortest and tallest a grown tree is drawn, in metres.
const SHORTEST: f32 = 2.2;
const TALLEST: f32 = 4.6;

/// One tree's measures in world units, and its colours.
#[derive(Clone, Debug)]
pub(super) struct Look {
    pub(super) crown: Crown,
    pub(super) height: f32,
    /// Half its width.
    pub(super) radius: f32,
    /// Half its trunk's width at the foot.
    pub(super) thick: f32,
    pub(super) bark: Vec3,
    pub(super) leaf: Vec3,
}

impl Look {
    /// A tree of `kind` grown to `stage`, varied by `rng`.
    pub(super) fn new(kind: &KindLook, stage: TreeStage, rng: &mut Rng) -> Self {
        let (height, width) = drawn_metres(kind);
        let size = stage_size(stage);
        let leaf = colour(kind.leaf) * Vec3::new(vary(rng), vary(rng), vary(rng));
        Self {
            crown: kind.crown,
            height: height * METRE * size * between(rng, 0.9, 1.1),
            radius: width / 2.0 * METRE * size.sqrt().min(1.0) * between(rng, 0.9, 1.1),
            thick: trunk_metres(kind) * METRE * size.max(0.3),
            bark: colour(kind.bark),
            leaf: leaf.min(Vec3::ONE),
        }
    }
}

/// How big a tree is at `stage` against a grown one.
pub(super) fn stage_size(stage: TreeStage) -> f32 {
    match stage {
        TreeStage::Stump | TreeStage::Cleared => 0.0,
        TreeStage::Sapling => 0.22,
        TreeStage::Young => 0.42,
        TreeStage::HalfGrown => 0.7,
        TreeStage::Grown => 1.0,
    }
}

/// How tall and wide a grown tree of `kind` is drawn, in metres.
pub(super) fn drawn_metres(kind: &KindLook) -> (f32, f32) {
    let height = (1.4 + 0.5 * kind.height.sqrt()).clamp(SHORTEST, TALLEST);
    let width = match kind.crown {
        Crown::Column => 0.9,
        Crown::Spire | Crown::OpenCone => 1.6,
        Crown::Oval => 1.8,
        Crown::PineTop => 2.0,
        Crown::Dome => 2.3,
        Crown::Weeping | Crown::Shrubby => 2.5,
        Crown::Spreading | Crown::Tiers => 2.7,
        Crown::Umbrella => 3.0,
    };
    (height, width)
}

/// Half a grown trunk's width at the foot, in metres: wider in life, wider
/// drawn, though far less.
pub(super) fn trunk_metres(kind: &KindLook) -> f32 {
    (0.07 + 0.06 * kind.trunk).min(0.22)
}

/// 1, more or less: each colour channel on its own.
fn vary(rng: &mut Rng) -> f32 {
    between(rng, 0.9, 1.1)
}
