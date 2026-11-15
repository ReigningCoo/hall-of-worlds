//! How a belt fastens and ends: a buckle's frame or a knot, a long end hanging down
//! the front (a cord's with a tassel), a sash's knot and hanging ends, and the obi's
//! bow behind and cord round its middle.

use bevy::prelude::*;
use shared::clothing::{Belt, BeltEnd, BeltFastening, BeltForm, Fittings};

use super::belt::Lay;
use super::shape::Wear;
use crate::item_art::armour::{Model, Stuff};
use crate::item_art::body::{blob, limb};

impl Wear<'_> {
    /// The buckle or knot at its start, and its end: tucked, or hanging long.
    pub(super) fn belt_fastening(
        &self,
        model: &mut Model,
        belt: &Belt,
        lay: &Lay,
        at: &dyn Fn(f32, f32, f32) -> (Vec3, Vec3),
        high: f32,
    ) {
        let lift = self.thick + 1.0;
        let (front, out) = at(0.0, 0.0, lift);
        let down = Vec3::NEG_Y;
        // Across the strap: up and down it worn, sideways lying straight.
        let across = match lay {
            Lay::Straight { .. } => Vec3::X,
            Lay::Round { .. } => Vec3::Y,
        };
        match (belt.form, belt.fastening) {
            (BeltForm::Obi, _) => {
                // A cord round its middle, and the bow at the back.
                let (back, back_out) =
                    self.belt_at(lay, self.belt_length(lay) / 2.0, 0.0, lift + 8.0);
                blob(
                    model.of(Stuff::Cloth),
                    back + back_out * 18.0,
                    Vec3::new(70.0, 45.0, 22.0),
                );
                self.obi_cord(model, lay, lift);
            }
            (BeltForm::Sash, _) => {
                // Knotted a little to the left, the ends hanging.
                let along = self.belt_length(lay) * 0.12;
                let (knot, knot_out) = at(along, 0.0, lift);
                blob(
                    model.of(Stuff::Shade),
                    knot + knot_out * 6.0,
                    Vec3::splat(high * 0.3),
                );
                for (shift, long) in [(-12.0, 220.0), (12.0, 180.0)] {
                    let start = knot + knot_out * 4.0 + Vec3::X * shift;
                    let end = start + down * long + knot_out * 6.0;
                    self.ribbon(
                        model,
                        Stuff::Main,
                        &[(start, knot_out), (end, knot_out)],
                        high * 0.55,
                    );
                }
            }
            (_, BeltFastening::Buckle) => {
                // A frame round the strap.
                let (a, b) = (high * 0.6 + 4.0, 12.0);
                let side = out.cross(across).normalize_or(Vec3::X);
                let corners = [
                    front + across * a + side * b,
                    front - across * a + side * b,
                    front - across * a - side * b,
                    front + across * a - side * b,
                    front + across * a + side * b,
                ];
                let path: Vec<(Vec3, f32)> =
                    corners.iter().map(|&c| (c + out * 2.0, 2.2)).collect();
                limb(model.of(Stuff::Rivet), &path);
                self.belt_end(model, belt, lay, front, out, down, high);
            }
            (_, BeltFastening::Tied) => {
                blob(
                    model.of(Stuff::Shade),
                    front + out * 6.0,
                    Vec3::splat(high * 0.4 + 4.0),
                );
                self.belt_end(model, belt, lay, front, out, down, high);
            }
        }
    }

    /// A long end hanging down the front.
    #[allow(clippy::too_many_arguments)]
    fn belt_end(
        &self,
        model: &mut Model,
        belt: &Belt,
        lay: &Lay,
        front: Vec3,
        out: Vec3,
        down: Vec3,
        high: f32,
    ) {
        if belt.end != BeltEnd::Long || matches!(lay, Lay::Straight { .. }) {
            return;
        }
        let start = front + out * (self.thick + 2.0) + Vec3::X * 10.0;
        let end = start + down * 300.0 + out * 8.0;
        if belt.form == BeltForm::Cord {
            limb(model.of(Stuff::Main), &[(start, 4.0), (end, 4.0)]);
            blob(
                model.of(Stuff::Main),
                end + down * 12.0,
                Vec3::new(8.0, 18.0, 8.0),
            );
        } else {
            self.ribbon(model, Stuff::Main, &[(start, out), (end, out)], high * 0.85);
            if belt.fittings != Fittings::None {
                blob(
                    model.of(Stuff::Rivet),
                    end + down * 6.0 + out * 2.0,
                    Vec3::new(high * 0.4, 8.0, 2.0),
                );
            }
        }
    }

    /// The obi's cord round its middle, tied at the front.
    fn obi_cord(&self, model: &mut Model, lay: &Lay, lift: f32) {
        let length = self.belt_length(lay);
        let path: Vec<(Vec3, f32)> = (0..=48)
            .map(|step| {
                (
                    self.belt_at(lay, step as f32 / 48.0 * length, 0.0, lift).0,
                    3.0,
                )
            })
            .collect();
        limb(model.of(Stuff::Cloth), &path);
        let (front, out) = self.belt_at(lay, length * 0.02, 0.0, lift + 3.0);
        blob(model.of(Stuff::Cloth), front + out * 3.0, Vec3::splat(7.0));
    }
}
