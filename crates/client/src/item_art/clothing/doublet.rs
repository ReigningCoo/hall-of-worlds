//! Doublets: close-fitted, often padded, to the waist, the hip or with a short skirt;
//! the peascod's pointed belly, quilting, eyelets for points, collars, sleeves puffed,
//! winged or laced on, close-set buttons or lacing down the front, slashing, frogging
//! and contrasting linings.

use std::f32::consts::TAU;

use shared::clothing::{
    Doublet, DoubletCollar, DoubletDecoration, DoubletFastening, DoubletLength, DoubletSleeves,
    Padding,
};

use super::shape::{FITTED, FRONT, Wear};
use super::strips::Ease;
use crate::item_art::armour::{Model, Stuff};

impl Wear<'_> {
    pub(super) fn doublet(&self, model: &mut Model, doublet: &Doublet) {
        let (shoulder, waist) = (self.shoulder(), self.waist());
        let hem = self.hem(match doublet.length {
            DoubletLength::Waist => waist - 25.0,
            DoubletLength::Hip => self.hip() + 10.0,
            DoubletLength::Thigh => Self::between(self.hip(), self.knee(), 0.3),
        });
        let padded = !matches!(doublet.padding, Padding::None | Padding::Lined);
        let body = if padded { FITTED + 4.0 } else { FITTED };
        let peascod = doublet.padding == Padding::Peascod;
        let skirt = if doublet.length == DoubletLength::Thigh {
            0.3
        } else {
            0.05
        };
        let ease = move |height: f32, angle: f32| {
            let belly = match peascod {
                // Forwards over the belly, most just above the waist.
                true => {
                    let front = (angle - FRONT).cos().max(0.0).powi(2);
                    let near = 1.0 - ((height - waist - 60.0) / 140.0).abs().min(1.0);
                    28.0 * front * near
                }
                false => 0.0,
            };
            body + belly + (waist - height).max(0.0) * skirt
        };
        // A peascod comes to a point below the waist at the front.
        let bottom = move |angle: f32| match peascod {
            true => hem - 50.0 * (angle - FRONT).cos().max(0.0).powi(6),
            false => hem,
        };
        self.trunk(model, Stuff::Main, &|_| shoulder, &bottom, &ease);
        self.yoke(model, Stuff::Main, body, 8.0);
        if padded {
            for at in 0..16 {
                let angle = at as f32 / 16.0 * TAU;
                let down = [(shoulder - 10.0, angle), (bottom(angle) + 10.0, angle)];
                self.line(model, Stuff::Shade, &down, &ease, 2.0);
            }
        }
        if doublet.length == DoubletLength::Thigh {
            self.band(model, Stuff::Shade, waist, 5.0, ease(waist, 0.0));
        }
        if doublet.points {
            for at in 0..12 {
                let angle = at as f32 / 12.0 * TAU;
                for side in [-0.06, 0.06] {
                    self.button(
                        model,
                        Stuff::Backing,
                        (waist - 10.0, angle + side),
                        &ease,
                        2.5,
                    );
                }
            }
        }
        match doublet.collar {
            DoubletCollar::None => {}
            DoubletCollar::Standing => self.collar(model, Stuff::Main, 40.0, 8.0, body),
            DoubletCollar::Band => {
                let band = [
                    (shoulder, FRONT - 0.15),
                    (self.chest() - 30.0, FRONT + 0.55),
                ];
                self.strip(model, Stuff::Cloth, &band, &ease, 30.0);
                self.neck_ring(model, Stuff::Cloth, 8.0, body, 12.0);
            }
        }
        self.doublet_sleeves(model, doublet.sleeves);
        self.doublet_front(model, doublet.fastening, hem, &ease);
        self.doublet_decoration(model, doublet.decoration, hem, &ease);
    }

    fn doublet_sleeves(&self, model: &mut Model, sleeves: DoubletSleeves) {
        let (elbow, wrist) = (self.elbow(), self.wrist());
        let end = wrist - 10.0;
        match sleeves {
            DoubletSleeves::Close => self.sleeves(model, Stuff::Main, (-15.0, end), &|_| FITTED),
            DoubletSleeves::Puffed => {
                let puff = move |down: f32| {
                    let near = 1.0 - ((down - elbow * 0.3) / (elbow * 0.35)).abs().min(1.0);
                    FITTED + 26.0 * near
                };
                self.sleeves(model, Stuff::Main, (-15.0, end), &puff);
            }
            DoubletSleeves::Wings => {
                self.sleeves(model, Stuff::Main, (-15.0, end), &|_| FITTED);
                self.rolls(model, Stuff::Main, FITTED);
            }
            DoubletSleeves::Detachable => {
                self.sleeves(model, Stuff::Main, (-15.0, end), &|_| FITTED);
                self.cuffs(model, Stuff::Shade, 0.0, 4.0, FITTED);
                self.sleeve_ties(model, FITTED);
            }
        }
    }

    fn doublet_front(&self, model: &mut Model, fastening: DoubletFastening, hem: f32, ease: Ease) {
        let shoulder = self.shoulder();
        let span = (shoulder - 15.0, hem + 15.0);
        let seam = [(shoulder, FRONT), (hem, FRONT)];
        match fastening {
            DoubletFastening::Buttons => {
                self.line(model, Stuff::Shade, &seam, ease, 2.5);
                self.buttons(model, Stuff::Rivet, FRONT + 0.05, span, ease, 22.0, 3.5);
            }
            DoubletFastening::Lacing => self.lacing(model, FRONT, span, ease),
            DoubletFastening::Hooks => {
                self.line(model, Stuff::Shade, &seam, ease, 2.5);
                self.buttons(model, Stuff::Rivet, FRONT, span, ease, 40.0, 2.0);
            }
            DoubletFastening::Ties => {
                // A bow at the right of the chest, its ends hanging.
                let at = (self.chest() - 20.0, FRONT + 0.45);
                self.button(model, Stuff::Cloth, at, ease, 9.0);
                for side in [-0.06, 0.06] {
                    let end = [at, (at.0 - 150.0, at.1 + side)];
                    self.strip(model, Stuff::Cloth, &end, ease, 14.0);
                }
            }
        }
    }

    fn doublet_decoration(
        &self,
        model: &mut Model,
        decoration: DoubletDecoration,
        hem: f32,
        ease: Ease,
    ) {
        let (shoulder, waist) = (self.shoulder(), self.waist());
        match decoration {
            DoubletDecoration::None => {}
            DoubletDecoration::Slashing => {
                for row in 0..3 {
                    let top = shoulder - 50.0 - row as f32 * 70.0;
                    for column in [-0.55, -0.3, 0.3, 0.55] {
                        let angle = FRONT + column;
                        let slash = [(top, angle), (top - 50.0, angle)];
                        self.strip(model, Stuff::Cloth, &slash, ease, 7.0);
                    }
                }
            }
            DoubletDecoration::Frogging => {
                let rows = ((shoulder - 40.0 - waist) / 35.0) as usize;
                for row in 0..=rows {
                    let height = shoulder - 40.0 - row as f32 * 35.0;
                    let across = [(height, FRONT - 0.42), (height, FRONT + 0.42)];
                    self.line(model, Stuff::Field, &across, ease, 4.0);
                }
            }
            DoubletDecoration::ContrastLining => {
                for side in [-0.04, 0.04] {
                    let edge = [(shoulder, FRONT + side), (hem, FRONT + side)];
                    self.strip(model, Stuff::Cloth, &edge, ease, 10.0);
                }
                self.neck_ring(model, Stuff::Cloth, 8.0, ease(shoulder, 0.0), 6.0);
            }
        }
    }
}
