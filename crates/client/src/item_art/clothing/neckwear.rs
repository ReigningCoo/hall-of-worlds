//! Neckwear round the throat, over the clothes' collars and any necklace: scarves wound
//! twice with their ends down the chest, neckerchiefs knotted in front with their point
//! behind, cravats, ruffs of figure-eight pleats (the cartwheel on a wire frame),
//! falling bands lying flat on the shoulders, and the jabot's falls of lace.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::armour::neck::Hem;
use shared::clothing::{Neckwear, NeckwearForm, RuffSize};

use super::drape::Fall;
use super::lofted::{oval, roll};
use super::shape::{FRONT, Wear};
use crate::item_art::armour::neck::dir;
use crate::item_art::armour::surface::ring_grid;
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::blob;

/// Out from the neck: over a collar.
const OVER_COLLAR: f32 = 20.0;

impl Wear<'_> {
    pub fn neckwear(&self, model: &mut Model, neckwear: &Neckwear) {
        let base = self.neck_base();
        let radius = self.neck() + OVER_COLLAR;
        // Under torso armour only what is round the neck shows.
        let ends = !self.wearer().is_some_and(|wearer| wearer.covered.torso);
        match neckwear.form {
            NeckwearForm::Scarf => {
                roll(
                    model,
                    Stuff::Main,
                    &oval(Vec3::Y * (base + 20.0), (radius, radius), 0.08),
                    13.0,
                );
                let upper = oval(Vec3::Y * (base + 44.0), (radius - 4.0, radius - 4.0), -0.12);
                roll(model, Stuff::Main, &upper, 12.0);
                if ends {
                    self.hanging_end(model, 0.3, self.chest() - 70.0, 70.0);
                    self.hanging_end(model, -0.15, self.chest() - 20.0, 65.0);
                }
            }
            NeckwearForm::Neckerchief => {
                roll(
                    model,
                    Stuff::Main,
                    &oval(Vec3::Y * (base + 22.0), (radius, radius), 0.1),
                    7.0,
                );
                let knot = dir(0.0) * (radius + 6.0) + Vec3::Y * (base + 14.0);
                blob(model.of(Stuff::Main), knot, Vec3::new(13.0, 11.0, 9.0));
                // Its point down the back.
                let fall = Fall {
                    front: base - 10.0,
                    back: self.shoulder() - 110.0,
                    side: None,
                };
                let top = |angle: f32| self.neck_top(angle, OVER_COLLAR - 4.0);
                let drape = self.drape_off();
                let hem = (Hem::Straight, 0.0);
                self.drape(
                    model,
                    Stuff::Main,
                    (2.0, TAU - 2.0),
                    &top,
                    &fall,
                    hem,
                    drape,
                );
            }
            NeckwearForm::Cravat => {
                roll(
                    model,
                    Stuff::Main,
                    &oval(Vec3::Y * (base + 30.0), (radius, radius), 0.0),
                    10.0,
                );
                let knot = dir(0.0) * (radius + 8.0) + Vec3::Y * (base + 18.0);
                blob(model.of(Stuff::Main), knot, Vec3::new(15.0, 13.0, 10.0));
                if ends {
                    self.hanging_end(model, 0.06, self.chest() - 10.0, 45.0);
                    self.hanging_end(model, -0.06, self.chest() + 10.0, 45.0);
                }
            }
            NeckwearForm::Ruff => {
                let reach = match neckwear.ruff {
                    Some(RuffSize::Wide) => 70.0,
                    Some(RuffSize::Cartwheel) => 135.0,
                    _ => 38.0,
                };
                self.ruff(model, Stuff::Main, reach);
            }
            NeckwearForm::FallingBand => {
                let fall = Fall {
                    front: base - 85.0,
                    back: base - 35.0,
                    side: None,
                };
                let top = |angle: f32| self.neck_top(angle, OVER_COLLAR - 6.0) + Vec3::Y * 10.0;
                let drape = self.drape_off();
                let hem = (Hem::Straight, 0.0);
                self.drape(
                    model,
                    Stuff::Main,
                    (0.12, TAU - 0.12),
                    &top,
                    &fall,
                    hem,
                    drape,
                );
            }
            NeckwearForm::Jabot => {
                roll(
                    model,
                    Stuff::Main,
                    &oval(Vec3::Y * (base + 30.0), (radius, radius), 0.0),
                    8.0,
                );
                if ends {
                    self.jabot(model);
                }
            }
        }
    }

    /// One end of a scarf or cravat hanging from the throat down the chest to `low`,
    /// to one side of the front by `side` (trunk angles, to the left as it grows).
    fn hanging_end(&self, model: &mut Model, side: f32, low: f32, wide: f32) {
        let ease = self.drape_off();
        let top = self.neck_base() - 15.0;
        let places = [(top, FRONT - side * 0.5), (low, FRONT - side)];
        self.strip(model, Stuff::Main, &places, &|_, _| ease, wide);
    }

    /// A ruff of figure-eight pleats standing out `reach` round the neck.
    pub fn ruff(&self, model: &mut Model, stuff: Stuff, reach: f32) {
        let middle = Self::between(self.neck_base(), self.chin(), 0.35);
        let inner = self.neck() + 8.0;
        const PLEATS: f32 = 36.0;
        let surface = |u: f32, v: f32| {
            let angle = u * TAU;
            let radius = inner + reach * v;
            let wave = (angle * PLEATS).sin() * (5.0 + 0.16 * reach * v);
            // A wide one droops a little towards its edge, more behind.
            let droop = reach * 0.12 * v * v * (1.0 + (-angle.cos()).max(0.0));
            let point = dir(angle) * radius + Vec3::Y * (middle + wave - droop);
            (point, Vec3::Y)
        };
        let columns = (PLEATS as usize) * 4;
        plate(model.of(stuff), &ring_grid(&surface, 4, columns), 5.0, true);
    }

    /// Falls of lace one under another down the front from the throat.
    fn jabot(&self, model: &mut Model) {
        let ease = self.drape_off();
        let top = self.neck_base() - 10.0;
        for fall in 0..4 {
            let high = top - fall as f32 * 32.0;
            let wide = 45.0 + fall as f32 * 14.0;
            let places = [(high, FRONT), (high - 55.0, FRONT)];
            let lift = 3.0 + (3 - fall) as f32 * 3.0;
            self.strip(model, Stuff::Main, &places, &|_, _| ease + lift, wide);
        }
    }
}
