//! `Look`: one fruit plant's measures in world units and its colours, from its
//! fruit's `FruitPlantLook`, the stage it has grown to and its own small
//! differences.
//!
//! Sizes are exaggerated, as herbs' are: small plants drawn larger than life,
//! trees much smaller (at most `TALLEST` metres, so a camera looking down
//! still sees round them) and fruit larger, so it shows on its plant.

use bevy::prelude::*;
use shared::fruit::{FruitDef, FruitPosition, FruitShape, FruitStage, PlantFamily};
use shared::herb::PartSize;
use shared::rng::Rng;

use super::super::plan::{METRE, between, colour};

/// The tallest a fruit plant is drawn, and the widest, in metres. Trees stand
/// three cells (4.5 m) apart in orchards.
const TALLEST: f32 = 4.4;
const WIDEST: f32 = 3.0;

/// The widest a plant without a trunk is drawn, in metres: they stand side
/// by side, a cell apart.
const WIDEST_LOW: f32 = 1.6;

/// How big a young plant is against a grown one.
const YOUNG: f32 = 0.45;

/// What shows on the plant besides its leaves.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Show {
    Leaves,
    Flowers,
    /// Fruit, in this colour.
    Fruit(Vec3),
}

/// One fruit plant's measures, in world units, and its colours.
#[derive(Clone, Debug)]
pub(super) struct Look {
    pub(super) height: f32,
    /// Half its width.
    pub(super) radius: f32,
    pub(super) leaf: Vec3,
    /// A leaf's length.
    pub(super) leaf_size: f32,
    pub(super) flower: Vec3,
    /// A flower's half width.
    pub(super) flower_size: f32,
    /// A fruit's half width, and its shape.
    pub(super) fruit_size: f32,
    pub(super) fruit_shape: FruitShape,
    pub(super) fruits: FruitPosition,
    pub(super) show: Show,
    /// How wide it grows against how tall, in life: a crown's shape.
    pub(super) wide: f32,
}

impl Look {
    /// `def`'s plant at `stage`, `size` as big as grown if given (else as big
    /// as `stage` is), varied by `rng`.
    pub(super) fn new(def: &FruitDef, stage: FruitStage, size: Option<f32>, rng: &mut Rng) -> Self {
        let plant = &def.plant_look;
        let (height, width) = drawn_metres(def);
        let grown = size.unwrap_or(if stage == FruitStage::Young {
            YOUNG
        } else {
            1.0
        });
        let height = height * METRE * grown * between(rng, 0.88, 1.12);
        let radius = width / 2.0 * METRE * grown * between(rng, 0.9, 1.1);
        let show = match stage {
            FruitStage::Young | FruitStage::Leafy => Show::Leaves,
            FruitStage::Flowering => Show::Flowers,
            FruitStage::Fruiting { ripe } => {
                Show::Fruit(colour(if ripe { def.look.ripe } else { def.look.unripe }))
            }
        };
        // A fruit's half width: from 5 cm, growing more slowly than the fruit;
        // on trees at least 10 cm, so it shows from afar.
        let fruit_size = (0.05 + 0.002 * f32::from(def.look.size).powf(0.8)).min(0.3);
        let least = if def.plant.has_trunk() { 0.1 } else { 0.0 };
        let fruit_size = fruit_size.max(least) * METRE;
        Self {
            height,
            radius,
            leaf: hue(colour(plant.leaf), rng),
            leaf_size: part(plant.leaf_size, 0.12, 0.2, 0.32, 0.5) * METRE * grown,
            flower: colour(plant.flower),
            flower_size: part(plant.flower_size, 0.035, 0.05, 0.07, 0.1) * METRE,
            fruit_size,
            fruit_shape: def.look.shape,
            fruits: plant.fruits,
            show,
            wide: f32::from(plant.spread) / f32::from(plant.height.max(1)),
        }
    }
}

/// How tall and wide `def`'s plant is drawn grown, in metres.
pub(super) fn drawn_metres(def: &FruitDef) -> (f32, f32) {
    let metres = |cm: u16| f32::from(cm) / 100.0;
    let (height, spread) = (metres(def.plant_look.height), metres(def.plant_look.spread));
    if def.plant.has_trunk() {
        let height = (0.6 + 1.1 * height.sqrt()).min(TALLEST);
        (height, (0.5 + 0.55 * spread.powf(0.6)).min(WIDEST))
    } else {
        let shown = |m: f32| 0.35 + 0.6 * m.powf(0.7);
        let low = matches!(
            def.plant,
            PlantFamily::LowMat | PlantFamily::LowHerb | PlantFamily::GroundVine
        );
        let height = if low {
            shown(height).min(0.6)
        } else {
            shown(height).min(2.4)
        };
        (height, shown(spread).min(WIDEST_LOW))
    }
}

/// A size for each of `PartSize`'s, in metres.
fn part(size: PartSize, tiny: f32, small: f32, medium: f32, large: f32) -> f32 {
    match size {
        PartSize::Tiny => tiny,
        PartSize::Small => small,
        PartSize::Medium => medium,
        PartSize::Large => large,
    }
}

/// A colour shifted a little, each channel on its own, so no two plants are
/// quite the same green.
fn hue(colour: Vec3, rng: &mut Rng) -> Vec3 {
    let mut shift = || between(rng, 0.9, 1.1);
    (colour * Vec3::new(shift(), shift(), shift())).min(Vec3::ONE)
}
