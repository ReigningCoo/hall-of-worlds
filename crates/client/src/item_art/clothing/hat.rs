//! Brimmed and stiff hats (Outer), over whatever else is on the head: a crown low,
//! round, tall, flat-topped or the conical hat's single cone, and a brim narrow to very
//! wide, turned up behind (the bycocket) or cocked on three sides (the tricorne); a
//! band, feather or cockade, a chin strap, and straw's plaited rings.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::clothing::{Crown, Garment, Hat, HatBrim, HatDecoration, HatMake};

use super::lofted::{oval, roll, solid};
use super::shape::Wear;
use super::skull::OFF_HEAD;
use super::turban::turban_reach;
use crate::item_art::armour::neck::dir;
use crate::item_art::armour::surface::ring_grid;
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::{blob, limb};

/// Where a hat's band sits: about the brow.
const BAND: f32 = 0.38;

/// How far out from the head a piece reaches where a hat's band sits, if it is one a
/// hat goes over.
pub(super) fn reach_under(garment: &Garment) -> Option<f32> {
    Some(match garment {
        Garment::Hood(hood) if hood.chaperon => OFF_HEAD + 44.0,
        Garment::Hood(_) => OFF_HEAD + 5.0,
        Garment::Cap(_) => OFF_HEAD + 10.0,
        Garment::Headcloth(cloth) => match cloth.turban {
            Some(size) => turban_reach(size),
            None => OFF_HEAD + 6.0,
        },
        Garment::Headdress(_) => OFF_HEAD + 8.0,
        _ => return None,
    })
}

impl Wear<'_> {
    pub fn hat(&self, model: &mut Model, hat: &Hat) {
        let under = self.wearer().and_then(|wearer| wearer.head.under);
        let off = under.map_or(OFF_HEAD - 1.0, |under| under + 3.0);
        let y = self.head_y(BAND);
        let half = self.head_half(y, off);
        // The crown clears the top of the head and whatever is on it.
        let piled = under.map_or(0.0, |under| (under - OFF_HEAD) * 1.6);
        let least = self.head_y(0.0) - y + off + piled + 8.0;
        if hat.crown == Crown::Cone {
            self.cone_hat(model, hat, y, half, least);
        } else {
            let (high, top, rounded) = match hat.crown {
                Crown::Low => (55.0, 0.8, true),
                Crown::Round => (85.0, 0.85, true),
                Crown::Tall => (150.0, 0.86, false),
                _ => (80.0, 1.0, false),
            };
            let high = least.max(high);
            let ring = |rise: f32, scale: f32| {
                oval(Vec3::Y * (y + rise), (half.0 * scale, half.1 * scale), 0.0)
            };
            let mut rings = vec![
                ring(0.0, 1.0),
                ring(high * 0.6, Self::between(1.0, top, 0.6)),
            ];
            if rounded {
                rings.push(ring(high * 0.9, top * 0.85));
                rings.push(ring(high, top * 0.45));
            } else {
                rings.push(ring(high, top));
                rings.push(ring(high + 3.0, top * 0.93));
            }
            solid(model, Stuff::Main, &rings);
            self.brim(model, hat, y, half);
            if hat.make == HatMake::Straw {
                for ring in rings.iter().skip(1).take(rings.len().saturating_sub(2)) {
                    roll(model, Stuff::Shade, ring, 1.4);
                }
            }
            self.hat_trim(model, hat, y, half, high);
        }
        if hat.chin_strap {
            let left = Vec3::new(half.0 * 0.97, y - 4.0, 0.0);
            let right = Vec3::new(-half.0 * 0.97, y - 4.0, 0.0);
            let under_chin = Vec3::new(0.0, self.chin() - 14.0, self.frame.head.1 as f32 * 0.35);
            limb(
                model.of(Stuff::Backing),
                &[(left, 2.0), (under_chin, 2.0), (right, 2.0)],
            );
        }
        self.horn_slits(model, off);
    }

    /// The brim round the crown's foot.
    fn brim(&self, model: &mut Model, hat: &Hat, y: f32, half: (f32, f32)) {
        let wide = match hat.brim {
            HatBrim::Narrow => 32.0,
            HatBrim::Wide | HatBrim::TurnedUpBehind => 75.0,
            HatBrim::Cocked => 80.0,
            HatBrim::VeryWide => 130.0,
        };
        let surface = |u: f32, v: f32| {
            let angle = u * TAU;
            // How far this part of the brim is turned up.
            let up = match hat.brim {
                HatBrim::TurnedUpBehind => (-angle.cos()).max(0.0).powf(0.7),
                HatBrim::Cocked => (1.0 - (3.0 * angle).cos()) / 2.0 * 0.9,
                _ => 0.0,
            };
            // A pointed front on a bycocket, longer corners on a tricorne.
            let reach = wide
                * match hat.brim {
                    HatBrim::TurnedUpBehind => 0.7 + 0.5 * angle.cos().max(0.0).powi(4),
                    HatBrim::Cocked => 1.0 + 0.25 * (1.0 - up),
                    _ => 1.0,
                };
            let droop = match hat.brim {
                HatBrim::Wide | HatBrim::VeryWide => 0.12 * reach * v * v,
                _ => 0.0,
            };
            let start = Vec3::new(half.0 * angle.sin(), y, half.1 * angle.cos());
            let out = dir(angle);
            let flat = reach * v * (1.0 - up);
            let rise = reach * v * up;
            let point = start + out * flat + Vec3::Y * (rise - droop);
            (
                point,
                (Vec3::Y * (1.0 - up) + out * up).normalize_or(Vec3::Y),
            )
        };
        plate(
            model.of(Stuff::Main),
            &ring_grid(&surface, 4, 48),
            self.thick,
            true,
        );
        if hat.make == HatMake::Straw {
            for ring in 1..4 {
                let v = ring as f32 / 4.0;
                let points: Vec<(Vec3, f32)> = (0..=48)
                    .map(|at| {
                        let (point, out) = surface(at as f32 / 48.0, v);
                        (point + out * self.thick * 0.5, 1.2)
                    })
                    .collect();
                limb(model.of(Stuff::Shade), &points);
            }
        }
    }

    /// The conical hat: one wide shallow cone from its point down over the head.
    fn cone_hat(&self, model: &mut Model, hat: &Hat, y: f32, half: (f32, f32), least: f32) {
        let apex = Vec3::Y * (y + least.max(110.0));
        let reach = match hat.brim {
            HatBrim::Narrow => 50.0,
            HatBrim::VeryWide => 190.0,
            _ => 130.0,
        };
        let surface = |u: f32, v: f32| {
            let angle = u * TAU;
            let rim = Vec3::new(
                (half.0 + reach) * angle.sin(),
                y - 15.0,
                (half.1 + reach) * angle.cos(),
            );
            let point = apex.lerp(rim, v.max(0.02));
            let out = (dir(angle) * (apex.y - rim.y) + Vec3::Y * reach).normalize_or(Vec3::Y);
            (point, out)
        };
        plate(
            model.of(Stuff::Main),
            &ring_grid(&surface, 6, 40),
            self.thick,
            true,
        );
        if hat.make == HatMake::Straw {
            for ring in 1..6 {
                let v = ring as f32 / 6.0;
                let points: Vec<(Vec3, f32)> = (0..=40)
                    .map(|at| {
                        let (point, out) = surface(at as f32 / 40.0, v);
                        (point + out * self.thick * 0.5, 1.2)
                    })
                    .collect();
                limb(model.of(Stuff::Shade), &points);
            }
        }
        blob(
            model.of(Stuff::Shade),
            apex + Vec3::Y * 3.0,
            Vec3::splat(7.0),
        );
    }

    /// A band round the crown's foot, a feather or a cockade.
    fn hat_trim(&self, model: &mut Model, hat: &Hat, y: f32, half: (f32, f32), high: f32) {
        let band = || oval(Vec3::Y * (y + 12.0), (half.0 + 1.0, half.1 + 1.0), 0.0);
        match hat.decoration {
            HatDecoration::None => {}
            HatDecoration::Band => roll(model, Stuff::Cloth, &band(), 6.0),
            HatDecoration::Feather => {
                roll(model, Stuff::Cloth, &band(), 4.0);
                let root = Vec3::new(half.0 * 0.95, y + 18.0, -half.1 * 0.2);
                let path = [
                    (root, 4.0),
                    (root + Vec3::new(25.0, high * 0.7, -60.0), 11.0),
                    (root + Vec3::new(10.0, high * 0.9, -170.0), 2.0),
                ];
                limb(model.of(Stuff::Cloth), &path);
            }
            HatDecoration::Cockade => {
                let at = Vec3::new(half.0 * 0.75, y + high * 0.45, half.1 * 0.75);
                let out = Vec3::new(0.7, 0.0, 0.7);
                blob(
                    model.of(Stuff::Cloth),
                    at + out * 6.0,
                    Vec3::new(20.0, 20.0, 20.0) * 0.8,
                );
                blob(model.of(Stuff::Rivet), at + out * 18.0, Vec3::splat(6.0));
            }
        }
    }
}
