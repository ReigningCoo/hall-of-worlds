//! `Look`: one vegetable plant's measures in world units and its colours,
//! from its `VegetablePlantLook`, the look of the part picked, the stage it
//! has grown to and its own small differences.
//!
//! Sizes are exaggerated, as herbs' are: small plants drawn larger than life,
//! tall ones smaller (the bamboo and the thorny tree as fruit trees are), and
//! what is picked larger still, so it shows on its plant.

use bevy::prelude::*;
use shared::herb::PartSize;
use shared::rng::Rng;
use shared::vegetable::{Part, PlantShape, VegetableDef, VegetableSet};

use super::super::plan::{METRE, between, colour};

/// How big a plant is at each stage against full grown.
const GROWN: [f32; 4] = [0.35, 0.65, 1.0, 1.0];

/// Straw: grain's leaves turn towards it as the ears ripen, and dry pods are it.
pub(super) const STRAW: Vec3 = Vec3::new(0.52, 0.42, 0.17);

/// Vegetables whose pods grow underground: nothing of them shows.
const UNDERGROUND: [&str; 2] = ["peanut", "bambara_groundnut"];

/// What shows on the plant besides its leaves and stems.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Show {
    Leaves,
    Flowers,
    /// What is picked, green: ears and heads before they ripen.
    Unripe,
    /// What is picked, ready.
    Ripe,
}

/// What a plant bears that may show: how its picked part is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Bears {
    /// Fruit hanging, or lying by a ground vine.
    Fruit,
    /// Pods: green ones picked, or dry ones holding the seeds picked.
    Pods { dry: bool },
    /// Ears of grain, seed heads, a flower head.
    Heads,
    /// A root's or bulb's shoulder at the ground.
    Shoulder,
    /// A head or heart of leaves, or stalks.
    Heart,
    /// Shoots coming up round its foot.
    Shoots,
    /// Nothing: it is all underground (tubers, groundnuts).
    Hidden,
}

/// One vegetable plant's measures, in world units, and its colours.
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
    /// What is picked: its colour ripe, and half its length and width.
    pub(super) part: Vec3,
    pub(super) part_size: Vec2,
    /// Its second colour, if it has one (a cob's husk).
    pub(super) accent: Option<Vec3>,
    pub(super) bears: Bears,
    pub(super) show: Show,
    /// How grown it is against full grown, 0.35 to 1.
    pub(super) grown: f32,
    pub(super) shape: PlantShape,
    /// Ears on the stalk's side, wrapped in husks, under a tassel (maize).
    pub(super) cob: bool,
}

impl Look {
    /// `def`'s plant at `stage` (1 to 4) with `slot` metres of room, varied by
    /// `rng`.
    pub(super) fn new(def: &VegetableDef, stage: u8, slot: f32, rng: &mut Rng) -> Self {
        let plant = &def.plant_look;
        let (height, width) = drawn_metres(def);
        let grown = GROWN[usize::from(stage.clamp(1, 4) - 1)];
        let bears = bears(def);
        let show = show(bears, stage);
        let mut leaf = hue(colour(plant.leaf), rng);
        if def.set == VegetableSet::Grain && show == Show::Ripe {
            leaf = leaf.lerp(STRAW, 0.6);
        }
        let radius = (width / 2.0).min(slot * 0.6);
        Self {
            height: height * METRE * grown * between(rng, 0.88, 1.12),
            radius: radius * METRE * grown * between(rng, 0.9, 1.1),
            leaf,
            leaf_size: part(plant.leaf_size, 0.1, 0.16, 0.26, 0.4) * METRE * grown,
            flower: colour(plant.flower),
            flower_size: part(plant.flower_size, 0.025, 0.04, 0.06, 0.09) * METRE,
            part: colour(def.look.colours[0]),
            part_size: part_size(def, bears) * METRE,
            accent: def.look.accent.map(colour),
            bears,
            show,
            grown,
            shape: def.plant,
            cob: def.key == "maize",
        }
    }

    /// Stems' colour: the leaves', darker and browner.
    pub(super) fn stem(&self) -> Vec3 {
        self.leaf * Vec3::new(0.85, 0.75, 0.6)
    }
}

/// How tall and wide `def`'s plant is drawn full grown, in metres.
pub(super) fn drawn_metres(def: &VegetableDef) -> (f32, f32) {
    let metres = |cm: u16| f32::from(cm) / 100.0;
    let (height, spread) = (metres(def.plant_look.height), metres(def.plant_look.spread));
    if matches!(def.plant, PlantShape::BambooGrove | PlantShape::ThornyTree) {
        let height = (0.6 + 1.1 * height.sqrt()).min(4.4);
        (height, (0.5 + 0.55 * spread.powf(0.6)).min(3.0))
    } else {
        let shown = |m: f32| 0.35 + 0.6 * m.powf(0.7);
        (shown(height).min(2.4), shown(spread).min(1.5))
    }
}

/// What `def`'s plant bears that may show.
fn bears(def: &VegetableDef) -> Bears {
    if UNDERGROUND.contains(&def.key.as_str()) {
        return Bears::Hidden;
    }
    match def.part {
        Part::Fruit => Bears::Fruit,
        Part::Pods => Bears::Pods { dry: false },
        Part::Seeds if def.set == VegetableSet::Pulse => Bears::Pods { dry: true },
        Part::Seeds | Part::EarOfGrain | Part::FlowerHead => Bears::Heads,
        Part::Root | Part::Bulb => Bears::Shoulder,
        Part::LeafBunch | Part::StalkBunch => Bears::Heart,
        Part::Shoots => Bears::Shoots,
        Part::Tuber => Bears::Hidden,
    }
}

/// What shows at `stage`: flowers before fruit and pods, green heads before
/// ripe ones; a root's shoulder, a heart or shoots only full grown; and a
/// plant bearing nothing to see flowers full grown.
fn show(bears: Bears, stage: u8) -> Show {
    match (bears, stage) {
        (_, 0..=2) => Show::Leaves,
        (Bears::Fruit | Bears::Pods { .. }, 3) => Show::Flowers,
        (Bears::Heads, 3) => Show::Unripe,
        (_, 3) => Show::Leaves,
        (Bears::Hidden, _) => Show::Flowers,
        _ => Show::Ripe,
    }
}

/// Half the length and width of what is picked, in metres: larger than life,
/// so it shows. A dry pod is drawn round its seeds.
fn part_size(def: &VegetableDef, bears: Bears) -> Vec2 {
    let (mut length, mut width) = (f32::from(def.look.length), f32::from(def.look.width));
    if bears == (Bears::Pods { dry: true }) {
        (length, width) = ((length * 5.0).max(120.0), (width * 1.5).max(20.0));
    }
    let half = |mm: f32| (mm / 1000.0 * 0.75 + 0.015).clamp(0.015, 0.3);
    Vec2::new(half(length), half(width))
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
