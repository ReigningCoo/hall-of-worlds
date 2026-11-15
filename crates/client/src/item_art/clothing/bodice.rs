//! Bodices: close round the trunk to under the breasts, the waist or tabs below it;
//! high, round, square or low necks (the low ones over straps), short sleeves or ones
//! laced on, lacing at the front, side or back (or the choli's ties), boning and a
//! stomacher.

use std::f32::consts::TAU;

use shared::clothing::{Bodice, BodiceLength, BodiceNeck, BodiceSleeves, Lacing, Stiffening};

use super::shape::{BACK, CLOSE, FITTED, FRONT, LEFT, Wear};
use crate::item_art::armour::{Model, Stuff};

/// How much of a neckline's depth is cut at this angle round: all of it across the
/// front (`wide` the cosine from the front where it starts), none at the sides.
fn scooped(angle: f32, wide: f32) -> f32 {
    (((angle - FRONT).cos() - wide) / (1.0 - wide) * 1.6).clamp(0.0, 1.0)
}

impl Wear<'_> {
    pub(super) fn bodice(&self, model: &mut Model, bodice: &Bodice) {
        let (shoulder, waist) = (self.shoulder(), self.waist());
        let bottom = self.hem(match bodice.length {
            BodiceLength::Chest => self.chest() - 70.0,
            BodiceLength::Waist | BodiceLength::Tabs => waist - 15.0,
        });
        let ease = move |_: f32, _: f32| CLOSE;
        let (deep, wide) = match bodice.neck {
            BodiceNeck::High | BodiceNeck::Round => (0.0, 0.0),
            BodiceNeck::Square => (110.0, 0.55),
            BodiceNeck::Low => (150.0, 0.3),
        };
        let top = move |angle: f32| shoulder - deep * scooped(angle, wide);
        self.trunk(model, Stuff::Main, &top, &|_| bottom, &ease);
        match bodice.neck {
            BodiceNeck::High => {
                self.yoke(model, Stuff::Main, CLOSE, 6.0);
                self.neck_ring(model, Stuff::Shade, 6.0, CLOSE, 4.0);
            }
            BodiceNeck::Round => {
                self.yoke(model, Stuff::Main, CLOSE, 45.0);
                self.neck_ring(model, Stuff::Shade, 45.0, CLOSE, 4.0);
            }
            BodiceNeck::Square | BodiceNeck::Low => {
                let wide_strap = if bodice.sleeves == BodiceSleeves::None {
                    35.0
                } else {
                    55.0
                };
                self.straps(
                    model,
                    Stuff::Main,
                    &[1.0, -1.0],
                    top(FRONT - 0.5),
                    wide_strap,
                    CLOSE,
                );
                let edge: Vec<_> = (0..=16)
                    .map(|at| {
                        let angle = FRONT - 1.2 + at as f32 / 16.0 * 2.4;
                        (top(angle) - 3.0, angle)
                    })
                    .collect();
                self.line(model, Stuff::Shade, &edge, &ease, 3.0);
            }
        }
        if bodice.length == BodiceLength::Tabs {
            for at in 0..14 {
                let angle = (at as f32 + 0.5) / 14.0 * TAU;
                let tab = [(bottom + 5.0, angle), (bottom - 70.0, angle)];
                self.strip(model, Stuff::Main, &tab, &|_, _| FITTED, 38.0);
            }
        }
        if bodice.stiffening == Stiffening::Boned {
            for at in 0..12 {
                let angle = at as f32 / 12.0 * TAU;
                let bone = [(top(angle) - 15.0, angle), (bottom + 10.0, angle)];
                self.line(model, Stuff::Shade, &bone, &ease, 2.0);
            }
        }
        if bodice.stomacher {
            let panel = [(top(FRONT) - 5.0, FRONT), (bottom - 40.0, FRONT)];
            self.strip(model, Stuff::Cloth, &panel, &|_, _| CLOSE + 1.5, 110.0);
        }
        let span = (top(FRONT).min(shoulder - 40.0) - 10.0, bottom + 10.0);
        let lift = if bodice.stomacher { 3.0 } else { 0.0 };
        let laced = move |_: f32, _: f32| CLOSE + lift;
        match bodice.lacing {
            Lacing::Front => self.lacing(model, FRONT, span, &laced),
            Lacing::Side => self.lacing(model, LEFT, (shoulder - 80.0, bottom + 10.0), &ease),
            Lacing::Back => self.lacing(model, BACK, (shoulder - 30.0, bottom + 10.0), &ease),
            Lacing::Ties => {
                let at = (bottom + 20.0, BACK);
                self.button(model, Stuff::Shade, at, &ease, 8.0);
                for side in [-0.08, 0.08] {
                    let end = [at, (at.0 - 120.0, at.1 + side)];
                    self.strip(model, Stuff::Shade, &end, &ease, 12.0);
                }
            }
        }
        let (elbow, wrist) = (self.elbow(), self.wrist());
        match bodice.sleeves {
            BodiceSleeves::None => {}
            BodiceSleeves::Short => {
                self.shoulder_sleeves(model, Stuff::Main, elbow * 0.4, &|_| FITTED)
            }
            BodiceSleeves::LacedOn => {
                self.shoulder_sleeves(model, Stuff::Main, wrist - 10.0, &|_| FITTED);
                self.cuffs(model, Stuff::Shade, 0.0, 4.0, FITTED);
                self.sleeve_ties(model, FITTED);
            }
        }
    }
}
