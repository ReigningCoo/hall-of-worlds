//! Cloth draped or tied over the head: veils framing the face and falling to the
//! reach (lace ones scalloped), kerchiefs knotted at the nape or under the chin, head
//! ties with their folds high on the head, the nemes with its lappets on the chest, the
//! square headcloth held by a cord ring, and the litham over the mouth and nose.
//! Turbans are wound (`turban.rs`).

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::prelude::*;
use shared::armour::neck::Hem;
use shared::clothing::{Headcloth, HeadclothForm, Reach};

use super::drape::Fall;
use super::lofted::{oval, roll};
use super::shape::Wear;
use super::skull::OFF_HEAD;
use crate::item_art::armour::neck::Around;
use crate::item_art::armour::surface::grid;
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::{blob, limb};

/// How far either side of the front a face is left open, from about the brow.
const FACE: f32 = 1.15;

impl Wear<'_> {
    pub fn headcloth(&self, model: &mut Model, cloth: &Headcloth) {
        let off = OFF_HEAD;
        match cloth.form {
            HeadclothForm::Veil => self.veil(model, cloth, 0.5),
            HeadclothForm::Corded => {
                self.veil(model, cloth, 0.5);
                if !self.helmeted() {
                    for (down, more) in [(0.27, 0.0), (0.33, 3.0)] {
                        let y = self.head_y(down);
                        let half = self.head_half(y, off + 8.0 + more);
                        roll(model, Stuff::Backing, &oval(Vec3::Y * y, half, 0.0), 6.0);
                    }
                }
            }
            HeadclothForm::Kerchief => self.kerchief(model, cloth),
            HeadclothForm::Turban => self.turban(model, cloth),
            HeadclothForm::HeadTie => self.head_tie(model),
            HeadclothForm::Nemes => self.nemes(model),
        }
        if cloth.face_covered {
            self.litham(model);
        }
        self.horn_slits(model, off);
    }

    /// The face's opening, by `down`: closed over the crown, open from the brow.
    fn framed(from: f32, wide: f32) -> impl Fn(f32) -> f32 {
        move |down: f32| wide * ((down - from) / 0.12).clamp(0.0, 1.0)
    }

    /// Over the head to `down`, open round the face, then falling to its reach.
    fn veil(&self, model: &mut Model, cloth: &Headcloth, down: f32) {
        let off = OFF_HEAD;
        let face = FACE + if self.snout() { 0.15 } else { 0.0 };
        self.over_head(
            model,
            Stuff::Main,
            (0.0, down),
            &|_, _| off,
            &Self::framed(0.24, face),
        );
        let shoulder = self.shoulder();
        let fall = match cloth.reach {
            Reach::Head => Fall {
                front: self.neck_base() + 20.0,
                back: self.neck_base() - 30.0,
                side: None,
            },
            Reach::Shoulders => Fall {
                front: shoulder - 50.0,
                back: shoulder - 90.0,
                side: Some(shoulder - 60.0),
            },
            Reach::Waist => Fall {
                front: self.chest() - 20.0,
                back: self.hem(self.waist()),
                side: Some(self.chest()),
            },
        };
        let hem = if cloth.lace {
            (Hem::Scalloped, 16.0)
        } else {
            (Hem::Straight, 0.0)
        };
        let top = |angle: f32| self.on_head(angle, down, off).0;
        let drape = self.drape_off();
        self.drape(
            model,
            Stuff::Main,
            (face, TAU - face),
            &top,
            &fall,
            hem,
            drape,
        );
    }

    /// A square or triangle over the head, knotted at the nape, or under the chin with
    /// its point down the back.
    fn kerchief(&self, model: &mut Model, cloth: &Headcloth) {
        let off = OFF_HEAD;
        let down = 0.62;
        let face = FACE + 0.1;
        self.over_head(
            model,
            Stuff::Main,
            (0.0, down),
            &|_, _| off,
            &Self::framed(0.3, face),
        );
        if cloth.reach == Reach::Head {
            let (knot, out) = self.on_head(PI, down - 0.04, off);
            blob(
                model.of(Stuff::Main),
                knot + out * 8.0,
                Vec3::new(16.0, 12.0, 12.0),
            );
            for side in [-1.0, 1.0] {
                let end = knot + out * 10.0 + Vec3::new(side * 18.0, -45.0, -12.0);
                limb(
                    model.of(Stuff::Main),
                    &[(knot + out * 8.0, 9.0), (end, 3.0)],
                );
            }
            return;
        }
        // Its point down the back, its ends knotted under the chin.
        let fall = Fall {
            front: self.chin(),
            back: self.shoulder() - 100.0,
            side: None,
        };
        let top = |angle: f32| self.on_head(angle, down, off).0;
        let drape = self.drape_off();
        let hem = (Hem::Straight, 0.0);
        self.drape(
            model,
            Stuff::Main,
            (face, TAU - face),
            &top,
            &fall,
            hem,
            drape,
        );
        let knot = Vec3::new(0.0, self.chin() - 16.0, self.frame.head.1 as f32 * 0.45);
        blob(model.of(Stuff::Main), knot, Vec3::new(14.0, 11.0, 11.0));
    }

    /// Folded and wound high, its folds standing up over the crown.
    fn head_tie(&self, model: &mut Model) {
        let off = OFF_HEAD;
        let down = 0.46;
        self.over_head(model, Stuff::Main, (0.0, down), &|_, _| off, &|_| 0.0);
        if self.helmeted() {
            return;
        }
        let crown = self.head_y(0.0) + off;
        let deep = self.frame.head.1 as f32;
        for (x, rise, z, size) in [
            (0.0, 35.0, deep * 0.3, 42.0),
            (-35.0, 22.0, deep * 0.15, 32.0),
            (35.0, 22.0, deep * 0.15, 32.0),
            (0.0, 18.0, -deep * 0.3, 30.0),
        ] {
            // Standing on the cloth over the crown.
            let centre = Vec3::new(x, crown + size * 0.8 + rise * 0.3, z);
            blob(
                model.of(Stuff::Main),
                centre,
                Vec3::new(size, size * 0.8, size * 0.6),
            );
        }
        let y = self.head_y(down - 0.04);
        roll(
            model,
            Stuff::Shade,
            &oval(Vec3::Y * y, self.head_half(y, off + 2.0), 0.0),
            4.0,
        );
    }

    /// The striped headcloth: tight across the brow, lappets down onto the chest and
    /// gathered behind.
    fn nemes(&self, model: &mut Model) {
        let off = OFF_HEAD;
        let down = 0.62;
        let face = FACE + if self.snout() { 0.15 } else { 0.0 };
        self.over_head(
            model,
            Stuff::Main,
            (0.0, down),
            &|_, _| off,
            &Self::framed(0.32, face),
        );
        let around = Around { frame: self.frame };
        let drape = self.drape_off();
        for side in [-1.0f32, 1.0] {
            let angle = side * (face + 0.2);
            let start = self.on_head(angle, down, off);
            let over = around.trunk(side * 0.75, self.shoulder() - 30.0, drape);
            let low = around.trunk(side * 0.7, self.chest() - 20.0, drape);
            let path = [start, (over, Vec3::Z), (low, Vec3::Z)];
            self.ribbon(model, Stuff::Main, &path, 75.0);
        }
        let fall = Fall {
            front: self.shoulder() - 20.0,
            back: self.shoulder() - 70.0,
            side: None,
        };
        let top = |angle: f32| self.on_head(angle, down, off).0;
        let from = face + 0.5;
        self.drape(
            model,
            Stuff::Main,
            (from, TAU - from),
            &top,
            &fall,
            (Hem::Straight, 0.0),
            drape,
        );
        // The band across the brow.
        let brow: Vec<(Vec3, f32)> = (0..=16)
            .map(|step| {
                let angle = -face + 2.0 * face * step as f32 / 16.0;
                (self.on_head(angle, 0.34, off + 2.0).0, 3.0)
            })
            .collect();
        limb(model.of(Stuff::Shade), &brow);
    }

    /// Cloth over the mouth and nose, and a snout, wrapped round to the back.
    fn litham(&self, model: &mut Model) {
        // Standing out over a snout's root.
        let lift = if self.snout() { 24.0 } else { 0.0 };
        let surface = |u: f32, v: f32| {
            let angle = (u - 0.5) * 2.0 * (FRAC_PI_2 + 0.5);
            let down = 0.6 + v * 0.42;
            let front = angle.cos().max(0.0);
            let (point, out) = self.on_head(angle, down, OFF_HEAD + 4.0);
            (point + Vec3::Z * front * lift, out)
        };
        plate(
            model.of(Stuff::Main),
            &grid(&surface, 6, 20),
            self.thick,
            false,
        );
    }
}
