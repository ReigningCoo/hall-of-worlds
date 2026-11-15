//! Hoods: over the head with an opening for the face (longer for a snout or beak), down
//! round the neck, with a cape draped over the shoulders (over a shawl), a point or a
//! liripipe behind, a buttoned front, dagged edges, the bashlyk's lappets wound round
//! the neck; and the chaperon, worn rolled up as a hat.

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use shared::armour::neck::Hem;
use shared::clothing::{Edge, Hood, HoodCape, HoodFace, Tail};

use super::drape::Fall;
use super::lofted::{oval, roll};
use super::shape::{BACK, Wear};
use super::skull::OFF_HEAD;
use crate::item_art::armour::{Model, Stuff};
use crate::item_art::body::{blob, limb};

impl Wear<'_> {
    /// Whether the wearer has a snout or beak to leave room for.
    pub fn snout(&self) -> bool {
        self.wearer().is_some_and(|wearer| wearer.head.snout)
    }

    pub fn hood(&self, model: &mut Model, hood: &Hood) {
        if hood.chaperon {
            self.chaperon(model, hood);
            return;
        }
        let shawl = self.wearer().is_some_and(|wearer| wearer.shawl);
        let off = OFF_HEAD;
        // The face's opening: from the brow down to under the chin, wider and longer
        // for a snout.
        let (from, wide, to): (f32, f32, f32) = match hood.face {
            HoodFace::Open => (0.3, 1.0, 1.08),
            HoodFace::Buttoned => (0.32, 0.85, 1.0),
            HoodFace::Close => (0.36, 0.72, 0.98),
        };
        let (wide, to) = if self.snout() {
            (wide.max(1.0) + 0.15, to + 0.14)
        } else {
            (wide, to)
        };
        let gap = move |down: f32| {
            if down <= from || down >= to {
                0.0
            } else {
                wide * ((down - from) / (to - from) * PI).sin().sqrt()
            }
        };
        let base = if hood.cape == HoodCape::None {
            self.down_at(self.neck_base() + 30.0)
        } else {
            self.down_at(self.neck_base() + 12.0)
        };
        self.over_head(model, Stuff::Main, (0.0, base), &|_, _| off, &gap);
        // The opening's edge, hemmed or drawn close.
        let edge: Vec<(Vec3, f32)> = (0..=24)
            .map(|step| {
                let down = from + (to - from) * step as f32 / 24.0;
                let angle = gap(down);
                (self.on_head(angle, down, off + 1.0).0, 2.0)
            })
            .chain((0..=24).map(|step| {
                let down = to - (to - from) * step as f32 / 24.0;
                let angle = -gap(down);
                (self.on_head(angle, down, off + 1.0).0, 2.0)
            }))
            .collect();
        limb(model.of(Stuff::Shade), &edge);
        if hood.face == HoodFace::Buttoned {
            for step in 0..3 {
                let down = to + 0.03 + step as f32 * 0.06;
                let (at, out) = self.on_head(0.0, down, off);
                blob(model.of(Stuff::Rivet), at + out * 3.0, Vec3::splat(4.0));
            }
        }
        self.hood_cape(model, hood, base, if shawl { off + 8.0 } else { off });
        self.hood_tail(model, hood.tail, off);
        if hood.lappets {
            self.lappets(model);
        }
        self.horn_slits(model, off);
    }

    /// The cape from the hood's lower edge over the shoulders.
    fn hood_cape(&self, model: &mut Model, hood: &Hood, base: f32, off: f32) {
        let (shoulder, chest) = (self.shoulder(), self.chest());
        let fall = match hood.cape {
            HoodCape::None => return,
            HoodCape::Neck => Fall {
                front: self.neck_base() - 60.0,
                back: self.neck_base() - 50.0,
                side: None,
            },
            HoodCape::Shoulders => Fall {
                front: shoulder - 70.0,
                back: shoulder - 80.0,
                side: Some(shoulder - 40.0),
            },
            HoodCape::Chest => Fall {
                front: chest - 40.0,
                back: chest - 60.0,
                side: Some(shoulder - 130.0),
            },
        };
        let hem = match hood.edge {
            Edge::Plain => (Hem::Straight, 0.0),
            Edge::Dagged => (Hem::Scalloped, 28.0),
        };
        let drape = self.drape_off() + off - OFF_HEAD;
        let top = |angle: f32| self.on_head(angle, base, off).0;
        self.drape(model, Stuff::Main, (-PI, PI), &top, &fall, hem, drape);
    }

    /// A point or a liripipe hanging from the back of the hood.
    fn hood_tail(&self, model: &mut Model, tail: Tail, off: f32) {
        let (start, out) = self.on_head(PI, 0.3, off);
        let start = start + out * 12.0;
        let point = start + Vec3::new(0.0, -25.0, -55.0);
        let end = match tail {
            Tail::None => return,
            Tail::Point => {
                limb(model.of(Stuff::Main), &[(start, 14.0), (point, 4.0)]);
                return;
            }
            Tail::Liripipe => self.waist(),
            Tail::LongLiripipe => self.hem(self.ankle() + 40.0),
        };
        let armoured = self.wearer().is_some_and(|wearer| wearer.covered.torso);
        // Off to one side of the back, clear of a tail's slit.
        let tailed = self.wearer().is_some_and(|wearer| wearer.tail.is_some());
        let angle = BACK + if tailed { 0.45 } else { 0.18 };
        let mut path = vec![(start, 14.0), (point, 12.0)];
        if !armoured {
            let ease = self.drape_off() + 16.0;
            let steps = 8;
            for step in 1..=steps {
                let height = Self::between(self.shoulder() - 60.0, end, step as f32 / steps as f32);
                let (at, _) = self.at(height, angle, ease);
                let thick = 13.0 - 7.0 * step as f32 / steps as f32;
                path.push((at, thick));
            }
        }
        limb(model.of(Stuff::Main), &path);
    }

    /// The bashlyk's long lappets, wound once round the neck with an end down the back.
    fn lappets(&self, model: &mut Model) {
        let off = self.drape_off();
        let y = self.neck_base() + 30.0;
        let radius = self.neck() + OFF_HEAD + 12.0;
        let ring = oval(Vec3::Y * y, (radius, radius), 0.15);
        roll(model, Stuff::Main, &ring, 12.0);
        let start = Vec3::new(radius * 0.7, y - 10.0, -radius * 0.7);
        let (shoulder, _) = self.at(self.shoulder() - 40.0, BACK - 0.3, off + 12.0);
        let (low, _) = self.at(self.chest() - 30.0, BACK - 0.35, off + 12.0);
        limb(
            model.of(Stuff::Main),
            &[(start, 13.0), (shoulder, 12.0), (low, 8.0)],
        );
    }

    /// The chaperon: a padded roll round the head with a crown of gathered cloth over
    /// it, and its cape hanging down one side.
    fn chaperon(&self, model: &mut Model, hood: &Hood) {
        let down = 0.36;
        self.over_head(
            model,
            Stuff::Main,
            (0.0, down),
            &|_, _| OFF_HEAD + 8.0,
            &|_| 0.0,
        );
        let y = self.head_y(down);
        let half = self.head_half(y, OFF_HEAD + 14.0);
        roll(model, Stuff::Main, &oval(Vec3::Y * y, half, 0.0), 22.0);
        // The cape, gathered at the roll, falling over the left shoulder.
        let side = FRAC_PI_2;
        let (top, out) = self.on_head(side, down + 0.05, OFF_HEAD + 30.0);
        let over = self.around().over_shoulder(
            side,
            self.frame.shoulder_half as f32 * 0.75,
            self.drape_off() + 6.0,
        );
        let bottom = over + Vec3::new(10.0, -110.0, 35.0);
        let path = [(top, out), (over, Vec3::Y), (bottom, Vec3::X)];
        let wide = if hood.cape == HoodCape::Chest {
            150.0
        } else {
            110.0
        };
        let off = self.drape_off();
        let clear = |point| self.clear_arms(point, off);
        self.ribbon_moved(model, Stuff::Main, &path, wide, &clear, (4, 4));
        if hood.tail.long() {
            let (start, _) = self.on_head(-side, down + 0.05, OFF_HEAD + 30.0);
            let over = self.around().over_shoulder(
                -side,
                self.frame.shoulder_half as f32 * 0.6,
                self.drape_off() + 8.0,
            );
            let low = over + Vec3::new(5.0, -200.0, 45.0);
            limb(
                model.of(Stuff::Main),
                &[(start, 14.0), (over, 12.0), (low, 8.0)],
            );
        }
        self.horn_slits(model, OFF_HEAD + 8.0);
    }
}
