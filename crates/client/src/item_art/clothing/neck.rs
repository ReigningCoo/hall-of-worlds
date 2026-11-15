//! Necklines on the yoke and the trunk's top: rings round the neck's opening, slits,
//! V and square necks drawn as lines, standing collars, and brooches on the shoulders.

use std::f32::consts::TAU;

use bevy::prelude::*;

use super::shape::{FRONT, Wear};
use crate::item_art::armour::surface::ring_grid;
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::{blob, limb};

impl Wear<'_> {
    /// How far from the neck's middle the yoke's opening is, cut `open` off it.
    fn opening(&self, open: f32) -> f32 {
        self.neck() + self.sleeve_ease(open)
    }

    /// How far the yoke is lifted off the shoulders, cut `ease` off the trunk.
    fn lift(&self, ease: f32) -> f32 {
        self.ease(ease, self.shoulder()) * 0.6
    }

    /// A point round the neck's opening, `out` beyond it.
    fn round_neck(&self, angle: f32, open: f32, ease: f32, out: f32) -> Vec3 {
        let radius = self.opening(open) + out;
        let y = self.neck_base() + self.lift(ease) + self.thick;
        Vec3::new(radius * angle.cos(), y, radius * angle.sin())
    }

    /// A line round the neck's opening.
    pub fn neck_ring(&self, model: &mut Model, stuff: Stuff, open: f32, ease: f32, wide: f32) {
        let points: Vec<(Vec3, f32)> = (0..=32)
            .map(|at| {
                let angle = at as f32 / 32.0 * TAU;
                (self.round_neck(angle, open, ease, wide / 2.0), wide / 2.0)
            })
            .collect();
        limb(model.of(stuff), &points);
    }

    /// A line from the neck's opening at `angle` down the trunk `deep` below the
    /// shoulders: a slit.
    pub fn slit(&self, model: &mut Model, angle: f32, deep: f32, open: f32, ease: f32) {
        let start = (self.round_neck(angle, open, ease, 2.0), 2.0);
        let mut path = vec![start];
        let down = self.along(
            &[(self.shoulder(), angle), (self.shoulder() - deep, angle)],
            &|_, _| ease,
            1.0,
        );
        path.extend(down.into_iter().map(|(point, _)| (point, 2.0)));
        limb(model.of(Stuff::Shade), &path);
    }

    /// A V or square neck as lines: from the opening either side of the front down to
    /// `deep` below the shoulders, meeting (V) or joined across (square).
    pub fn front_neck(&self, model: &mut Model, deep: f32, square: bool, open: f32, ease: f32) {
        let bottom = self.shoulder() - deep;
        for side in [-1.0, 1.0] {
            let angle = FRONT + side * 0.55;
            let start = (self.round_neck(angle, open, ease, 2.0), 2.5);
            let low = if square { FRONT + side * 0.45 } else { FRONT };
            let mut path = vec![start];
            path.extend(
                self.along(
                    &[(self.shoulder(), angle), (bottom, low)],
                    &|_, _| ease,
                    1.0,
                )
                .into_iter()
                .map(|(point, _)| (point, 2.5)),
            );
            limb(model.of(Stuff::Shade), &path);
        }
        if square {
            let across = [(bottom, FRONT - 0.45), (bottom, FRONT + 0.45)];
            self.line(model, Stuff::Shade, &across, &|_, _| ease, 5.0);
        }
    }

    /// A collar standing `high` up round the neck's opening.
    pub fn collar(&self, model: &mut Model, stuff: Stuff, high: f32, open: f32, ease: f32) {
        let radius = self.opening(open) + self.thick;
        let base = self.neck_base() + self.lift(ease);
        let surface = |u: f32, v: f32| {
            let angle = u * TAU;
            let out = Vec3::new(angle.cos(), 0.0, angle.sin());
            // Leaning in a little towards the top.
            (out * (radius - v * 4.0) + Vec3::Y * (base + v * high), out)
        };
        plate(
            model.of(stuff),
            &ring_grid(&surface, 2, 32),
            self.thick,
            true,
        );
    }

    /// A brooch or pin on top of each shoulder (1 the left, -1 the right).
    pub fn brooches(&self, model: &mut Model, sides: &[f32], ease: f32) {
        let x = (self.neck() + self.frame.shoulder_half as f32) / 2.0;
        let y = Self::between(self.neck_base(), self.shoulder(), 0.5) + self.lift(ease) + 8.0;
        for &side in sides {
            let centre = Vec3::new(side * x, y, 0.0);
            blob(model.of(Stuff::Rivet), centre, Vec3::new(8.0, 5.0, 8.0));
        }
    }
}
