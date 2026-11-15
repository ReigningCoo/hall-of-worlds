//! Stiff headdresses: the hennin's steeple or flowerpot leaning back from the crown,
//! the escoffion's horned rolls, the French hood's crescent band and the gable hood's
//! point over the brow, each on a close cap, and the kokoshnik's crest standing over
//! the brow; with a veil from the top or behind, and pearls, jewelled bands or a gold
//! net. Under a helmet only the close cap fits.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::prelude::*;
use shared::armour::neck::Hem;
use shared::clothing::{Headdress, HeaddressDecoration, HeaddressForm, HeaddressVeil};

use super::drape::Fall;
use super::lofted::{oval, roll, solid};
use super::shape::Wear;
use super::skull::OFF_HEAD;
use crate::item_art::armour::surface::{grid, studs};
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::{blob, limb};

/// How stiff pieces are drawn.
const STIFF: f32 = 3.0;

impl Wear<'_> {
    pub fn headdress(&self, model: &mut Model, dress: &Headdress) {
        let off = OFF_HEAD;
        // The close cap the hair is gathered into.
        let edge = match dress.form {
            HeaddressForm::FrenchHood | HeaddressForm::Gable => 0.62,
            _ => 0.45,
        };
        let face = match dress.form {
            HeaddressForm::FrenchHood | HeaddressForm::Gable => 1.2,
            _ => 0.0,
        };
        let gap = move |down: f32| face * ((down - 0.3) / 0.1).clamp(0.0, 1.0);
        self.over_head(model, Stuff::Main, (0.0, edge), &|_, _| off, &gap);
        if self.helmeted() {
            return;
        }
        match dress.form {
            HeaddressForm::Steeple => self.steeple(model, dress, 2.0, 0.08, 0.6),
            HeaddressForm::Flowerpot => self.steeple(model, dress, 1.0, 0.75, 0.42),
            HeaddressForm::Horned => self.horned(model, dress),
            HeaddressForm::FrenchHood => self.crescent(model, dress),
            HeaddressForm::Gable => self.gable(model, dress),
            HeaddressForm::Kokoshnik => self.kokoshnik(model, dress),
        }
        if dress.veil == HeaddressVeil::Behind {
            let fall = Fall {
                front: self.shoulder() - 40.0,
                back: self.shoulder() - 120.0,
                side: None,
            };
            let top = |angle: f32| self.on_head(angle, 0.38, off + STIFF + 2.0).0;
            let drape = self.drape_off();
            let hem = (Hem::Straight, 0.0);
            self.drape(
                model,
                Stuff::Shade,
                (2.0, TAU - 2.0),
                &top,
                &fall,
                hem,
                drape,
            );
        }
        self.horn_slits(model, off);
    }

    /// A cone from the back of the head, `high` heads tall, leaning back by `lean`,
    /// its top `top` of its base wide; a veil hanging from its tip.
    fn steeple(&self, model: &mut Model, dress: &Headdress, high: f32, top: f32, lean: f32) {
        let off = OFF_HEAD + STIFF;
        let base_y = self.head_y(0.35);
        let half = self.head_half(base_y, off);
        let rise = self.head_height() * high;
        let way = Vec3::new(0.0, lean.cos(), -lean.sin());
        let start = Vec3::new(0.0, base_y, -half.1 * 0.25);
        let rings: Vec<Vec<Vec3>> = (0..=6)
            .map(|step| {
                let t = step as f32 / 6.0;
                let scale = 1.0 + (top - 1.0) * t;
                oval(
                    start + way * rise * t,
                    (half.0 * scale, half.1 * scale),
                    lean,
                )
            })
            .collect();
        let tip = start + way * rise;
        solid(model, Stuff::Main, &rings);
        if dress.veil == HeaddressVeil::FromTop {
            // Sheer, from the tip down behind to the shoulders.
            let low = Vec3::new(
                0.0,
                self.shoulder() - 160.0,
                -self.frame.head.1 as f32 - 120.0,
            );
            let middle = (tip + low) / 2.0 + Vec3::new(0.0, 0.0, -30.0);
            let path = [
                (tip, Vec3::NEG_Z),
                (middle, Vec3::NEG_Z),
                (low, Vec3::NEG_Z),
            ];
            self.ribbon(model, Stuff::Cloth, &path, 160.0);
        }
        self.decorate(model, dress, &rings[0], &rings[1]);
    }

    /// Rolls standing out from the temples, with a band over the crown between them.
    fn horned(&self, model: &mut Model, dress: &Headdress) {
        let off = OFF_HEAD + STIFF;
        let y = self.head_y(0.38);
        let half = self.head_half(y, off);
        for side in [-1.0f32, 1.0] {
            let root = Vec3::new(side * half.0 * 0.8, y, 0.0);
            let rings: Vec<Vec<Vec3>> = (0..=4)
                .map(|step| {
                    let t = step as f32 / 4.0;
                    let at = root + Vec3::new(side * 110.0 * t, 70.0 * t, -10.0 * t);
                    let size = 38.0 * (1.0 - 0.45 * t);
                    oval(at, (size * 0.7, size), 0.0)
                })
                .collect();
            solid(model, Stuff::Main, &rings);
            if dress.decoration == HeaddressDecoration::GoldNet {
                for ring in &rings[1..] {
                    roll(model, Stuff::Field, ring, 1.6);
                }
            }
        }
        let band = (0..=12).map(|step| {
            let angle = -FRAC_PI_2 + PI * step as f32 / 12.0;
            (
                self.on_head(angle, 0.28 + 0.1 * angle.cos().abs(), off).0,
                7.0,
            )
        });
        limb(model.of(Stuff::Main), &band.collect::<Vec<_>>());
        let ring = oval(Vec3::Y * y, half, 0.0);
        self.decorate(model, dress, &ring, &ring);
    }

    /// The French hood's crescent: a stiff band arching over the head behind the brow.
    fn crescent(&self, model: &mut Model, dress: &Headdress) {
        let off = OFF_HEAD + STIFF;
        let surface = |u: f32, v: f32| {
            let angle = (u - 0.5) * 2.0 * 1.9;
            let down = 0.62 - (0.62 - 0.12) * (1.0 - (angle / 1.9).powi(2)).sqrt();
            let (point, out) = self.on_head(angle, down - 0.08 * v, off);
            (point + out * 25.0 * v, out)
        };
        plate(model.of(Stuff::Main), &grid(&surface, 3, 24), STIFF, false);
        self.billaments(model, dress, &|u| surface(u, 0.0));
    }

    /// The gable: a pointed frame over the brow, its sides down beside the face.
    fn gable(&self, model: &mut Model, dress: &Headdress) {
        let off = OFF_HEAD + STIFF + 4.0;
        let surface = |u: f32, v: f32| {
            let angle = (u - 0.5) * 2.0 * 1.25;
            // Straight sides up to a point over the middle of the brow.
            let down = 0.7 - 0.5 * (1.0 - angle.abs() / 1.25);
            let (point, out) = self.on_head(angle, down + 0.06 * v, off);
            (point + Vec3::Z * 12.0 * (1.0 - angle.abs()), out)
        };
        plate(model.of(Stuff::Main), &grid(&surface, 3, 20), STIFF, false);
        self.billaments(model, dress, &|u| surface(u, 0.0));
    }

    /// The kokoshnik: a crest standing up over the brow, round to the ears.
    fn kokoshnik(&self, model: &mut Model, dress: &Headdress) {
        let off = OFF_HEAD + STIFF;
        let y = self.head_y(0.42);
        let half = self.head_half(y, off);
        let crest = self.head_height() * 0.75;
        let surface = |u: f32, v: f32| {
            let angle = (u - 0.5) * 2.0 * 1.7;
            let rise = crest * (1.0 - (angle / 1.7).powi(2)).max(0.0).sqrt();
            let out = Vec3::new(angle.sin(), 0.0, angle.cos());
            let point = Vec3::new(half.0 * angle.sin(), y + rise * v, half.1 * angle.cos());
            (point + out * 6.0 * v, out)
        };
        plate(model.of(Stuff::Main), &grid(&surface, 6, 24), STIFF, false);
        if dress.decoration == HeaddressDecoration::Pearls {
            studs(model, Stuff::Etch, &surface, (4, 10), 4.0, 4.0);
        }
        self.billaments(model, dress, &|u| surface(u, 1.0));
    }

    /// A jewelled band along an edge, if it has one.
    fn billaments(&self, model: &mut Model, dress: &Headdress, edge: &dyn Fn(f32) -> (Vec3, Vec3)) {
        if dress.decoration != HeaddressDecoration::Billaments {
            return;
        }
        let points: Vec<(Vec3, f32)> = (0..=24)
            .map(|step| {
                let (at, out) = edge(step as f32 / 24.0);
                (at + out * 3.0, 4.0)
            })
            .collect();
        limb(model.of(Stuff::Field), &points);
        for (at, _) in points.iter().step_by(3) {
            blob(model.of(Stuff::Etch), *at + Vec3::Z * 3.0, Vec3::splat(4.5));
        }
    }

    /// Pearls round a stiff piece's base, or gold net over it.
    fn decorate(&self, model: &mut Model, dress: &Headdress, base: &[Vec3], next: &[Vec3]) {
        match dress.decoration {
            HeaddressDecoration::Pearls => {
                for (at, above) in base.iter().zip(next).step_by(2) {
                    blob(
                        model.of(Stuff::Etch),
                        at.lerp(*above, 0.1),
                        Vec3::splat(4.5),
                    );
                }
            }
            HeaddressDecoration::Billaments => roll(model, Stuff::Field, base, 4.0),
            HeaddressDecoration::GoldNet => roll(model, Stuff::Field, base, 2.5),
            HeaddressDecoration::None => {}
        }
    }
}
