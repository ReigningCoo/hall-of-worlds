//! Over the shoulders: shawls round the back with their ends down the front, partlets
//! fitted to the neck and shoulders with a standing or ruffled collar, fichus tucked in
//! at the front, the dupatta over both shoulders, capelets to the elbows (slit at the
//! sides for wing-arms), and the fur tippet round the neck with its ends hanging.

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use shared::armour::neck::Hem;
use shared::clothing::{Shawl, ShawlCollar, ShawlForm, ShawlLength};

use super::drape::Fall;
use super::lofted::{oval, roll};
use super::shape::{FRONT, Wear};
use crate::item_art::armour::{Model, Stuff};
use crate::item_art::body::limb;

/// Off the neck where a shawl hangs from.
const ROUND_NECK: f32 = 14.0;

impl Wear<'_> {
    pub fn shawl(&self, model: &mut Model, shawl: &Shawl) {
        let off = self.drape_off();
        let top = |angle: f32| self.neck_top(angle, ROUND_NECK);
        let plain = (Hem::Straight, 0.0);
        let (shoulder, chest) = (self.shoulder(), self.chest());
        let end = match shawl.length {
            ShawlLength::Chest => chest - 50.0,
            ShawlLength::Waist => self.waist(),
            ShawlLength::Knee => self.hem(self.knee()),
        };
        let armoured = self.wearer().is_some_and(|wearer| wearer.covered.torso);
        match shawl.form {
            ShawlForm::Shawl => {
                let fall = Fall {
                    front: shoulder - 60.0,
                    back: self.waist().max(end),
                    side: Some(shoulder - 140.0),
                };
                self.drape(model, Stuff::Main, (-PI, PI), &top, &fall, plain, off);
                if shawl.length > ShawlLength::Chest && !armoured {
                    self.shawl_ends(model, end, 110.0, off);
                }
            }
            ShawlForm::Partlet => {
                let fall = Fall {
                    front: chest + 30.0,
                    back: chest + 20.0,
                    side: Some(shoulder - 15.0),
                };
                self.drape(model, Stuff::Main, (-PI, PI), &top, &fall, plain, off);
            }
            ShawlForm::Fichu => {
                let fall = Fall {
                    front: chest - 10.0,
                    back: shoulder - 130.0,
                    side: Some(shoulder - 25.0),
                };
                // Its ends cross over the breast and are tucked in.
                self.drape(model, Stuff::Main, (-PI, PI), &top, &fall, plain, off);
            }
            ShawlForm::Dupatta => {
                self.straps(
                    model,
                    Stuff::Main,
                    &[1.0, -1.0],
                    shoulder - 90.0,
                    120.0,
                    off,
                );
                if !armoured {
                    self.shawl_ends(model, end, 150.0, off);
                }
            }
            ShawlForm::Capelet => {
                let elbow = shoulder - self.frame.elbow_length as f32;
                let fall = Fall {
                    front: chest - 40.0,
                    back: chest - 60.0,
                    side: Some(elbow),
                };
                // Slit at the sides for wing-arms.
                let wings = self.wearer().is_some_and(|wearer| wearer.wings);
                let halves: &[(f32, f32)] = if wings {
                    &[
                        (-FRAC_PI_2 + 0.15, FRAC_PI_2 - 0.15),
                        (FRAC_PI_2 + 0.15, 3.0 * FRAC_PI_2 - 0.15),
                    ]
                } else {
                    &[(-PI, PI)]
                };
                for &half in halves {
                    self.drape(model, Stuff::Main, half, &top, &fall, plain, off);
                }
            }
            ShawlForm::Tippet => {
                let y = self.neck_base() + 14.0;
                let radius = self.neck() + ROUND_NECK + 10.0;
                roll(
                    model,
                    Stuff::Main,
                    &oval(Vec3::Y * y, (radius, radius), 0.1),
                    18.0,
                );
                if !armoured {
                    self.tippet_ends(model, end, off);
                }
            }
        }
        match shawl.collar {
            ShawlCollar::None => {}
            ShawlCollar::Standing => self.collar(model, Stuff::Main, 45.0, ROUND_NECK, 8.0),
            ShawlCollar::Ruffled => self.ruff(model, Stuff::Main, 30.0),
        }
    }

    /// The two ends hanging down the front from over the shoulders.
    fn shawl_ends(&self, model: &mut Model, end: f32, wide: f32, off: f32) {
        let top = self.shoulder() - 20.0;
        for side in [-1.0, 1.0] {
            let places = [(top, FRONT - side * 0.75), (end, FRONT - side * 0.6)];
            self.strip(model, Stuff::Main, &places, &|_, _| off + 4.0, wide);
        }
    }

    /// A fur tippet's ends, hanging from round the neck down the front.
    fn tippet_ends(&self, model: &mut Model, end: f32, off: f32) {
        for (side, low) in [(-1.0, end), (1.0, end + 60.0)] {
            let path: Vec<(Vec3, f32)> = (0..=6)
                .map(|step| {
                    let t = step as f32 / 6.0;
                    let height = Self::between(self.neck_base() - 20.0, low, t);
                    let angle = FRONT - side * (0.7 - 0.2 * t);
                    let (at, out) = self.at(height, angle, off);
                    (at + out * 16.0, 17.0 - 4.0 * t)
                })
                .collect();
            limb(model.of(Stuff::Main), &path);
        }
    }
}
