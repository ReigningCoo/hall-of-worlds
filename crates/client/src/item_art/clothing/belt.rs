//! Belts: a strap with a buckle or tied, studs, plates or a chain; a cord wound twice
//! and knotted; a sash with hanging ends, perhaps pleated; the obi, wide, with a cord
//! round it and a bow behind. Worn it goes round the waist over whatever it cinches
//! (`legs.rs`), the long end hanging at the front; alone a strap or cord lies
//! folded in three runs down its picture, and a sash or obi keeps its round. How it
//! fastens and ends is in `belt_ends.rs`.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::clothing::{Belt, BeltEnd, BeltForm, BeltWidth, Fittings};

use super::legs::BELT_EASE;
use super::shape::{FITTED, FRONT, Wear};
use crate::item_art::armour::surface::grid;
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::{blob, limb};

/// How tall a belt is.
pub(super) fn height(belt: &Belt) -> f32 {
    match (belt.form, belt.width) {
        (BeltForm::Cord, _) => 14.0,
        (BeltForm::Obi, _) => 110.0,
        (_, BeltWidth::Narrow) => 22.0,
        (_, BeltWidth::Wide) => 45.0,
        (_, BeltWidth::VeryWide) => 80.0,
    }
}

/// Where a belt lies: round the waist, or folded down its picture in three runs.
pub(super) enum Lay {
    Round { middle: f32, ease: f32, round: f32 },
    Straight { long: f32, high: f32 },
}

/// Runs of a belt folded for its picture.
const RUNS: usize = 3;

/// A point along a belt folded back and forth down its picture, and its way along.
fn folded(along: f32, long: f32, high: f32) -> (Vec3, Vec3) {
    let radius = high / 2.0 + 10.0;
    let turn = std::f32::consts::PI * radius;
    let run = (long - turn * (RUNS - 1) as f32) / RUNS as f32;
    let mut left = along;
    for at in 0..RUNS {
        let x = (at as f32 - 1.0) * 2.0 * radius;
        let down = at % 2 == 0;
        if left <= run || at == RUNS - 1 {
            return match down {
                true => (Vec3::new(x, -left, 0.0), Vec3::NEG_Y),
                false => (Vec3::new(x, -run + left, 0.0), Vec3::Y),
            };
        }
        left -= run;
        if left <= turn {
            let angle = left / radius;
            let (sin, cos) = angle.sin_cos();
            return match down {
                true => (
                    Vec3::new(x + radius - cos * radius, -run - sin * radius, 0.0),
                    Vec3::new(sin, -cos, 0.0),
                ),
                false => (
                    Vec3::new(x + radius - cos * radius, sin * radius, 0.0),
                    Vec3::new(sin, cos, 0.0),
                ),
            };
        }
        left -= turn;
    }
    (Vec3::ZERO, Vec3::NEG_Y)
}

impl Wear<'_> {
    pub(super) fn belt(&self, model: &mut Model, belt: &Belt) {
        let high = height(belt);
        let straight =
            self.wearer().is_none() && !matches!(belt.form, BeltForm::Sash | BeltForm::Obi);
        let lay = if straight {
            Lay::Straight {
                long: 700.0
                    + if belt.end == BeltEnd::Long {
                        250.0
                    } else {
                        0.0
                    },
                high,
            }
        } else {
            let middle = self.belt_height();
            let ease = match self.wearer() {
                Some(_) => BELT_EASE,
                None => FITTED,
            };
            // How far round, measured on the cloth.
            let round = (0..40)
                .map(|at| {
                    let a = |at: usize| self.hips().at(middle, at as f32 / 40.0 * TAU, ease).0;
                    a(at).distance(a(at + 1))
                })
                .sum();
            Lay::Round {
                middle,
                ease,
                round,
            }
        };
        let length = match &lay {
            Lay::Round { round, .. } => *round,
            Lay::Straight { long, .. } => *long,
        };
        let thick = self.thick;
        let at = |along: f32, across: f32, lift: f32| self.belt_at(&lay, along, across, lift);
        if belt.form == BeltForm::Cord {
            // Wound twice, one turn over the other.
            for (turn, lift) in [(-5.0, 0.0), (5.0, 3.0)] {
                let path: Vec<(Vec3, f32)> = (0..=48)
                    .map(|step| (at(step as f32 / 48.0 * length, turn, lift).0, 4.0))
                    .collect();
                limb(model.of(Stuff::Main), &path);
            }
        } else {
            let surface = |u: f32, v: f32| at(u * length, (v - 0.5) * high, 0.0);
            let columns = (length / 20.0) as usize;
            plate(
                model.of(Stuff::Main),
                &grid(&surface, 2, columns),
                thick,
                false,
            );
        }
        if belt.pleated {
            for row in [-0.25, 0.0, 0.25] {
                let path: Vec<(Vec3, f32)> = (0..=40)
                    .map(|step| {
                        (
                            at(step as f32 / 40.0 * length, row * high, thick / 2.0).0,
                            1.0,
                        )
                    })
                    .collect();
                limb(model.of(Stuff::Shade), &path);
            }
        }
        self.belt_fittings(model, belt, &at, length, high);
        self.belt_fastening(model, belt, &lay, &at, high);
    }

    /// A point `along` the belt from its front (or top), `across` it, `lift` out.
    pub(super) fn belt_at(&self, lay: &Lay, along: f32, across: f32, lift: f32) -> (Vec3, Vec3) {
        match lay {
            Lay::Straight { long, high } => {
                let (point, way) = folded(along, *long, *high);
                let side = Vec3::new(-way.y, way.x, 0.0);
                (point + side * across + Vec3::Z * lift, Vec3::Z)
            }
            Lay::Round {
                middle,
                ease,
                round,
            } => {
                let angle = FRONT - along / round * TAU;
                self.hips().at(middle + across, angle, ease + lift)
            }
        }
    }

    /// Studs, plates or a chain along it.
    fn belt_fittings(
        &self,
        model: &mut Model,
        belt: &Belt,
        at: &dyn Fn(f32, f32, f32) -> (Vec3, Vec3),
        length: f32,
        high: f32,
    ) {
        let lift = self.thick / 2.0;
        match belt.fittings {
            Fittings::None => {}
            Fittings::Studs => {
                let count = (length / 32.0) as usize;
                for n in 1..count {
                    let (point, out) = at(n as f32 * 32.0, 0.0, lift);
                    blob(
                        model.of(Stuff::Rivet),
                        point + out * 1.5,
                        Vec3::splat(high * 0.15),
                    );
                }
            }
            Fittings::Plates => {
                let count = (length / 45.0) as usize;
                for n in 1..count {
                    let middle = n as f32 * 45.0;
                    let surface = |u: f32, v: f32| {
                        at(
                            middle + (u - 0.5) * 30.0,
                            (v - 0.5) * high * 0.8,
                            lift + 1.0,
                        )
                    };
                    plate(model.of(Stuff::Field), &grid(&surface, 1, 3), 1.6, false);
                }
            }
            Fittings::Chain => {
                // Hanging in a loop below the belt, from the front round to the left.
                let path: Vec<(Vec3, f32)> = (0..=16)
                    .map(|step| {
                        let t = step as f32 / 16.0;
                        let sag = (t * std::f32::consts::PI).sin() * 45.0;
                        let (point, out) = at(t * length * 0.3, -high / 2.0 - sag, lift + 1.0);
                        (point + out, 1.6)
                    })
                    .collect();
                limb(model.of(Stuff::Rivet), &path);
            }
        }
    }

    pub(super) fn belt_length(&self, lay: &Lay) -> f32 {
        match lay {
            Lay::Round { round, .. } => *round,
            Lay::Straight { long, .. } => *long,
        }
    }
}
