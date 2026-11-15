//! Soft brimless caps: round caps over the skull, berets puffed out flat, cones and the
//! Phrygian cap bent forward, fezzes and kalpaks as short cylinders and tall fur caps,
//! with ear flaps tied under the chin, a turned-up band or a peak, and a button,
//! tassel or feather on top.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::clothing::{Cap, CapBrim, CapMake, CapShape, CapTop};

use super::lofted::{oval, oval_by, roll, solid};
use super::shape::Wear;
use super::skull::OFF_HEAD;
use crate::item_art::armour::{Model, Stuff};
use crate::item_art::body::{blob, limb};

/// How far down from the crown a cap's edge comes: above the brow and ears.
const EDGE: f32 = 0.42;

impl Wear<'_> {
    pub fn cap(&self, model: &mut Model, cap: &Cap) {
        let off = match cap.make {
            CapMake::Fur => OFF_HEAD + 6.0,
            _ => OFF_HEAD,
        };
        let y = self.head_y(EDGE);
        let half = self.head_half(y, off);
        // Under a helmet only what fits inside it is drawn: a close cap.
        let top = if self.helmeted() {
            self.over_head(model, Stuff::Main, (0.0, EDGE), &|_, _| off, &|_| 0.0);
            None
        } else {
            self.crown_of(model, cap, y, half, off)
        };
        if cap.ear_flaps {
            self.ear_flaps(model, off);
        }
        match cap.brim {
            CapBrim::None => {}
            CapBrim::Band => {
                let ring = oval(Vec3::Y * (y + 12.0), (half.0 + 4.0, half.1 + 4.0), 0.0);
                roll(model, Stuff::Main, &ring, 10.0);
            }
            CapBrim::Peak => {
                // A stiff half-disc out over the brow.
                let surface = |u: f32, v: f32| {
                    let angle = (u - 0.5) * 2.2;
                    let reach = 1.0 + v * 0.45;
                    let point = Vec3::new(
                        half.0 * reach * angle.sin(),
                        y - 2.0 - v * 10.0,
                        half.1 * reach * angle.cos(),
                    );
                    (point, Vec3::Y)
                };
                let grid = crate::item_art::armour::surface::grid(&surface, 3, 12);
                crate::item_art::armour::plate(model.of(Stuff::Shade), &grid, 2.5, false);
            }
        }
        if let Some(top) = top {
            self.cap_top(model, cap.top, top, half);
        }
        self.horn_slits(model, off);
    }

    /// The cap's crown by its shape, from its edge at height `y`; its top.
    fn crown_of(
        &self,
        model: &mut Model,
        cap: &Cap,
        y: f32,
        half: (f32, f32),
        off: f32,
    ) -> Option<Vec3> {
        let crown = self.head_y(0.0) + off;
        // Never inside the head and what is under the cap.
        let ring = |rise: f32, scale: f32, forward: f32, tip: f32| {
            let level = y + rise;
            let (across, deep) = if level < self.head_y(0.0) {
                self.head_half(level, off + 6.0)
            } else {
                (0.0, 0.0)
            };
            oval(
                Vec3::new(0.0, level, forward),
                (
                    (half.0 * scale).max(across),
                    (half.1 * scale).max(deep + forward.abs()),
                ),
                tip,
            )
        };
        let rings = match cap.shape {
            CapShape::Round => {
                self.over_head(model, Stuff::Main, (0.0, EDGE), &|_, _| off, &|_| 0.0);
                // Its seams from the crown.
                for seam in 0..4 {
                    let angle = seam as f32 * TAU / 4.0;
                    let points: Vec<(Vec3, f32)> = (0..=8)
                        .map(|step| {
                            let down = EDGE * step as f32 / 8.0;
                            (self.on_head(angle, down, off + 1.0).0, 1.3)
                        })
                        .collect();
                    limb(model.of(Stuff::Shade), &points);
                }
                return Some(Vec3::Y * crown);
            }
            CapShape::Flat => {
                // Puffed out flat and wide, slouched to the left.
                let high = crown - y;
                let swell = |angle: f32| 18.0 + 14.0 * angle.sin();
                let puff = |rise: f32, more: f32| {
                    oval_by(Vec3::new(8.0, y + rise, 0.0), half, -0.06, &|angle| {
                        swell(angle) * more
                    })
                };
                vec![
                    ring(0.0, 1.0, 0.0, 0.0),
                    puff(high * 0.55, 1.6),
                    puff(high * 0.85, 1.7),
                    puff(high * 1.0, 1.0),
                    ring(high * 1.05, 0.2, 0.0, -0.06),
                ]
            }
            CapShape::Cone => {
                let high = crown - y + 90.0;
                (0..=12)
                    .map(|step| {
                        let t = step as f32 / 12.0;
                        let scale = (1.0 - t).max(0.04) * if t < 0.5 { 1.0 } else { 0.95 };
                        ring(high * t, scale, 0.0, 0.0)
                    })
                    .collect()
            }
            CapShape::Phrygian => {
                // A cone bent forward over the brow, its point falling in front.
                let high = crown - y + 50.0;
                (0..=12)
                    .map(|step| {
                        let t = step as f32 / 12.0;
                        let rise = high * (t * 1.25).min(1.0) - 30.0 * (t - 0.8).max(0.0) * 5.0;
                        // Bent only above the head.
                        let bent = ((t - 0.45) / 0.55).max(0.0);
                        let forward = 90.0 * bent * bent;
                        ring(rise, (1.0 - t * 0.85).max(0.08), forward, -0.9 * bent)
                    })
                    .collect()
            }
            CapShape::Cylinder | CapShape::Tall => {
                let tall = cap.shape == CapShape::Tall;
                let high = crown - y + if tall { 110.0 } else { 45.0 };
                let (bottom, top) = match (tall, cap.top) {
                    (true, _) => (1.08, 1.12),
                    (false, CapTop::Tassel) => (1.0, 0.86),
                    (false, _) => (1.02, 0.96),
                };
                vec![
                    ring(0.0, bottom, 0.0, 0.0),
                    ring(high * 0.5, (bottom + top) / 2.0, 0.0, 0.0),
                    ring(high, top, 0.0, 0.0),
                    ring(high + 4.0, top * 0.92, 0.0, 0.0),
                ]
            }
        };
        let tip = rings.last()?.iter().copied().sum::<Vec3>() / rings.last()?.len() as f32;
        solid(model, Stuff::Main, &rings);
        if cap.make == CapMake::Knitted {
            // Ribbing round it.
            for ring in rings.iter().skip(1).take(rings.len().saturating_sub(2)) {
                roll(model, Stuff::Shade, ring, 1.5);
            }
        }
        Some(tip)
    }

    /// Flaps down over the ears, tied under the chin.
    fn ear_flaps(&self, model: &mut Model, off: f32) {
        let wide = 0.5;
        for side in [-1.0f32, 1.0] {
            let middle = side * std::f32::consts::FRAC_PI_2;
            let surface = |u: f32, v: f32| {
                let angle = middle + (u - 0.5) * 2.0 * wide;
                let down = EDGE - 0.03 + v * (0.92 - EDGE);
                self.on_head(angle, down, off + 1.0)
            };
            let grid = crate::item_art::armour::surface::grid(&surface, 6, 8);
            crate::item_art::armour::plate(model.of(Stuff::Main), &grid, self.thick, false);
        }
        let left = self.on_head(1.5, 0.92, off).0;
        let right = self.on_head(-1.5, 0.92, off).0;
        let knot = Vec3::new(0.0, self.chin() - 14.0, self.frame.head.1 as f32 * 0.35);
        limb(
            model.of(Stuff::Shade),
            &[(left, 2.5), (knot, 3.0), (right, 2.5)],
        );
    }

    /// A button, tassel or feather on top.
    fn cap_top(&self, model: &mut Model, top: CapTop, at: Vec3, half: (f32, f32)) {
        match top {
            CapTop::Plain => {}
            CapTop::Button => blob(
                model.of(Stuff::Shade),
                at + Vec3::Y * 5.0,
                Vec3::new(9.0, 6.0, 9.0),
            ),
            CapTop::Tassel => {
                // Over the outside of the crown, clear of it.
                let hang = Vec3::new(half.0 + 20.0, at.y - 60.0, -10.0);
                let over = at + Vec3::new(half.0 * 0.6, 4.0, -6.0);
                let cord = [
                    (at + Vec3::Y * 3.0, 2.5),
                    (over, 2.5),
                    (hang + Vec3::Y * 20.0, 2.5),
                ];
                limb(model.of(Stuff::Cloth), &cord);
                limb(
                    model.of(Stuff::Cloth),
                    &[(hang + Vec3::Y * 22.0, 4.0), (hang, 11.0)],
                );
            }
            CapTop::Feather => {
                let root = Vec3::new(half.0 * 0.9, at.y - 15.0, -half.1 * 0.3);
                let path = [
                    (root, 4.0),
                    (root + Vec3::new(30.0, 70.0, -50.0), 9.0),
                    (root + Vec3::new(40.0, 110.0, -120.0), 2.0),
                ];
                limb(model.of(Stuff::Cloth), &path);
            }
        }
    }
}
