//! `Plan`: one plant's measures in world units, from its herb's `PlantLook`,
//! the stage it has grown to and its own small differences.
//!
//! Sizes are exaggerated (owner's): small herbs are drawn larger than life and
//! tall ones a little smaller, so all can be seen and none hides a cell.

use bevy::prelude::*;
use shared::herb::{PartSize, PlantLook, Rgb};
use shared::rng::Rng;

/// World units in a metre: a cell is a metre and a half.
pub const METRE: f32 = crate::dungeon_map::CELL_SIZE / 1.5;

/// The tallest any plant is drawn, in metres.
const TALLEST: f32 = 2.2;

/// The widest one plant is drawn across, in metres.
const WIDEST: f32 = 1.3;

/// How tall or wide something `cm` across in life is drawn, in metres.
pub fn shown_metres(cm: u16) -> f32 {
    let metres = f32::from(cm) / 100.0;
    0.35 + 0.6 * metres.powf(0.7)
}

/// How many plants a patch of this herb has: fewer the wider each grows.
pub fn plants_in_patch(look: &PlantLook) -> usize {
    match shown_metres(look.spread) {
        wide if wide >= 1.1 => 1,
        wide if wide >= 0.85 => 2,
        wide if wide >= 0.65 => 3,
        _ => 4,
    }
}

/// How a stage looks: how big the plant is, how many of its leaves it has,
/// and whether it is flowering (or fruiting).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Growth {
    pub size: f32,
    pub leaves: f32,
    pub flowers: bool,
}

impl Growth {
    /// Stages 1 (a shoot) to 4 (flowering).
    pub fn of(stage: u8) -> Self {
        let (size, leaves) = match stage {
            0 | 1 => (0.4, 0.4),
            2 => (0.65, 0.65),
            _ => (1.0, 1.0),
        };
        Self {
            size,
            leaves,
            flowers: stage >= 4,
        }
    }
}

/// One plant's measures, in world units.
#[derive(Clone, Debug)]
pub struct Plan {
    pub height: f32,
    /// Half its width.
    pub radius: f32,
    /// A leaf's length.
    pub leaf: f32,
    pub leaf_colour: Vec3,
    /// Its flowers' colour, if they show.
    pub flower: Option<Vec3>,
    /// A flower's half width.
    pub flower_size: f32,
    pub growth: Growth,
    /// How far it leans from upright, in radians, and which way.
    pub lean: Vec2,
}

impl Plan {
    /// A plant of `look` at `stage` among `count` in its patch, varied by `rng`.
    pub fn new(look: &PlantLook, stage: u8, count: usize, rng: &mut Rng) -> Self {
        let growth = Growth::of(stage);
        let crowd = if count > 1 { 0.75 } else { 1.0 };
        let height = shown_metres(look.height).min(TALLEST) * METRE * vary(rng, 0.15);
        let radius = shown_metres(look.spread).min(WIDEST) / 2.0 * METRE * crowd * vary(rng, 0.1);
        let leaf = radius
            * match look.leaf_size {
                PartSize::Tiny => 0.3,
                PartSize::Small => 0.45,
                PartSize::Medium => 0.65,
                PartSize::Large => 0.9,
            };
        let flower_size = match look.flower_size {
            PartSize::Tiny => 0.45,
            PartSize::Small => 0.7,
            PartSize::Medium => 1.0,
            PartSize::Large => 1.5,
        };
        let angle = rng.range_f64(0.0, std::f64::consts::TAU) as f32;
        let lean = Vec2::from_angle(angle) * rng.range_f64(0.0, 0.14) as f32;
        Self {
            height: height * growth.size,
            radius: radius * growth.size,
            leaf: (leaf * growth.size).max(1.0),
            leaf_colour: hue(colour(look.leaf), rng),
            flower: look.flower.map(|flower| hue(colour(flower), rng)),
            flower_size,
            growth,
            lean,
        }
    }

    /// `base` leaves at this stage: at least one.
    pub fn count(&self, base: usize) -> usize {
        ((base as f32 * self.growth.leaves).round() as usize).max(1)
    }

    /// Where a point `up` world units above the foot is, as the plant leans.
    pub fn leaned(&self, up: f32) -> Vec3 {
        Vec3::new(self.lean.x * up, up, self.lean.y * up)
    }

    /// Stems' and twigs' colour: the leaves', darker and browner.
    pub fn stem_colour(&self) -> Vec3 {
        self.leaf_colour * Vec3::new(0.85, 0.75, 0.6)
    }
}

/// Bark's colour.
pub const WOOD: Vec3 = Vec3::new(0.36, 0.26, 0.17);

/// 1, more or less by up to `by`.
fn vary(rng: &mut Rng, by: f64) -> f32 {
    rng.range_f64(1.0 - by, 1.0 + by) as f32
}

/// A colour as 0 to 1, in the light the maps' vertex colours use.
pub fn colour(Rgb(r, g, b): Rgb) -> Vec3 {
    let linear = |c: u8| (f32::from(c) / 255.0).powf(2.2);
    Vec3::new(linear(r), linear(g), linear(b))
}

/// A colour shifted a little, each channel on its own, so no two plants are
/// quite the same green.
fn hue(colour: Vec3, rng: &mut Rng) -> Vec3 {
    let mut shift = || rng.range_f64(0.88, 1.12) as f32;
    (colour * Vec3::new(shift(), shift(), shift())).min(Vec3::ONE)
}

/// A random number from `low` to `high`.
pub fn between(rng: &mut Rng, low: f32, high: f32) -> f32 {
    rng.range_f64(f64::from(low), f64::from(high)) as f32
}
