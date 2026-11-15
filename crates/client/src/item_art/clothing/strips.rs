//! What lies on and over the trunk's cloth: the yoke over the shoulders, straps, lines
//! and ribbons along it (seams, edges, bands, clavi), buttons and lacing.

use std::f32::consts::TAU;

use bevy::prelude::*;

use super::shape::Wear;
use crate::item_art::armour::surface::{grid, ring_grid};
use crate::item_art::armour::{Model, Stuff, plate};
use crate::item_art::body::{blob, limb};

/// A place on the trunk: height and angle round.
pub(super) type On = (f32, f32);
/// How far cloth is cut off the skin, by height and angle round.
pub(super) type Ease<'e> = &'e dyn Fn(f32, f32) -> f32;

impl Wear<'_> {
    /// Over the shoulders, from the trunk's top (`ease` off it) in to round the neck,
    /// `open` beyond the neck's skin, standing up off the shoulders as far as it does
    /// off the trunk so it lies over a shirt's yoke.
    pub fn yoke(&self, model: &mut Model, stuff: Stuff, ease: f32, open: f32) {
        let hips = self.hips();
        let (shoulder, base) = (self.shoulder(), self.neck_base());
        let ease = self.ease(ease, shoulder);
        let open = self.neck() + self.sleeve_ease(open);
        let surface = |u: f32, v: f32| {
            let angle = u * TAU;
            let (outer, _) = hips.at(shoulder, angle, ease);
            let inner = Vec3::new(open * angle.cos(), base, open * angle.sin());
            (outer.lerp(inner, v) + Vec3::Y * ease * 0.6, Vec3::Y)
        };
        plate(
            model.of(stuff),
            &ring_grid(&surface, 3, 40),
            self.thick,
            true,
        );
    }

    /// A broad strap from the front of the trunk at `from` over each shoulder (1 the
    /// left, -1 the right) to the back, `wide` across: a bodice's or vest's shoulders
    /// above a low neckline, or a draped tunic's one shoulder.
    #[allow(clippy::too_many_arguments)]
    pub fn straps(
        &self,
        model: &mut Model,
        stuff: Stuff,
        sides: &[f32],
        from: f32,
        wide: f32,
        ease: f32,
    ) {
        let base = self.neck_base();
        for &side in sides {
            let over = side * (self.neck() + 10.0 + wide / 2.0);
            let lift = self.ease(ease, self.shoulder()) + self.thick;
            let top = (Vec3::new(over, base + lift * 0.6, 0.0), Vec3::Y);
            let up = |angle: f32| -> Vec<(Vec3, Vec3)> {
                (0..4)
                    .map(|step| {
                        let height = Self::between(from, base, step as f32 / 5.0);
                        self.at(height, angle, ease + 1.0)
                    })
                    .collect()
            };
            let mut path = up(super::shape::FRONT - side * 0.5);
            path.push(top);
            path.extend(up(super::shape::BACK + side * 0.5).into_iter().rev());
            self.ribbon(model, stuff, &path, wide);
        }
    }

    /// A flat strip `wide` across along points with their ways out.
    pub fn ribbon(&self, model: &mut Model, stuff: Stuff, path: &[(Vec3, Vec3)], wide: f32) {
        if path.len() < 2 {
            return;
        }
        let surface = |u: f32, v: f32| {
            let at = v * (path.len() - 1) as f32;
            let (i, t) = ((at.floor() as usize).min(path.len() - 2), at.fract());
            let t = if at >= (path.len() - 1) as f32 {
                1.0
            } else {
                t
            };
            let (a, b) = (path[i], path[i + 1]);
            let point = a.0.lerp(b.0, t);
            let out = a.1.lerp(b.1, t).normalize_or(Vec3::Z);
            let across = (b.0 - a.0).cross(out).normalize_or(Vec3::X);
            (point + across * (u - 0.5) * wide, out)
        };
        let rows = (path.len() - 1) * 2;
        plate(model.of(stuff), &grid(&surface, rows, 1), self.thick, false);
    }

    /// Points along the trunk through these places, `lift` over cloth `ease` off it.
    pub fn along(&self, places: &[On], ease: Ease, lift: f32) -> Vec<(Vec3, Vec3)> {
        self.runs(places, ease, lift).concat()
    }

    /// The same in runs, broken where they cross a tail's slit.
    fn runs(&self, places: &[On], ease: Ease, lift: f32) -> Vec<Vec<(Vec3, Vec3)>> {
        let mut runs = vec![Vec::new()];
        let mut push = |height: f32, angle: f32| {
            if self.in_slit(height, angle) {
                runs.push(Vec::new());
                return;
            }
            let (point, out) = self.at(height, angle, ease(height, angle));
            runs.last_mut()
                .expect("one at least")
                .push((point + out * (self.thick + lift), out));
        };
        for pair in places.windows(2) {
            // About every 30 mm, down or round (the trunk is about 150 mm round to
            // its middle).
            let far = (pair[0].0 - pair[1].0)
                .abs()
                .max((pair[0].1 - pair[1].1).abs() * 150.0);
            let steps = (far / 30.0).ceil().max(2.0) as usize;
            for step in 0..steps {
                let t = step as f32 / steps as f32;
                let height = Self::between(pair[0].0, pair[1].0, t);
                let angle = Self::between(pair[0].1, pair[1].1, t);
                push(height, angle);
            }
        }
        if let Some(&(height, angle)) = places.last() {
            push(height, angle);
        }
        runs.retain(|run| run.len() > 1);
        runs
    }

    /// A line `wide` thick along the trunk through these places.
    pub fn line(&self, model: &mut Model, stuff: Stuff, places: &[On], ease: Ease, wide: f32) {
        for run in self.runs(places, ease, wide / 2.0) {
            let path: Vec<(Vec3, f32)> = run
                .into_iter()
                .map(|(point, _)| (point, wide / 2.0))
                .collect();
            limb(model.of(stuff), &path);
        }
    }

    /// A ribbon along the trunk through these places.
    pub fn strip(&self, model: &mut Model, stuff: Stuff, places: &[On], ease: Ease, wide: f32) {
        for run in self.runs(places, ease, 0.6) {
            self.ribbon(model, stuff, &run, wide);
        }
    }

    /// Buttons (or hooks, studs) every `apart` from `top` down to `bottom` at `angle`.
    #[allow(clippy::too_many_arguments)]
    pub fn buttons(
        &self,
        model: &mut Model,
        stuff: Stuff,
        angle: f32,
        (top, bottom): (f32, f32),
        ease: Ease,
        apart: f32,
        size: f32,
    ) {
        let count = ((top - bottom) / apart).floor().max(1.0) as usize;
        for at in 0..=count {
            let height = Self::between(top, bottom, at as f32 / count as f32);
            self.button(model, stuff, (height, angle), ease, size);
        }
    }

    /// One button (or knot, stud) on the cloth at a place.
    pub fn button(
        &self,
        model: &mut Model,
        stuff: Stuff,
        (height, angle): On,
        ease: Ease,
        size: f32,
    ) {
        if self.in_slit(height, angle) {
            return;
        }
        let (point, out) = self.at(height, angle, ease(height, angle));
        let centre = point + out * (self.thick + size * 0.8);
        blob(model.of(stuff), centre, Vec3::splat(size));
    }

    /// Lacing from `top` down to `bottom` at `angle`: the two edges and the cord
    /// crossing between them.
    pub fn lacing(&self, model: &mut Model, angle: f32, (top, bottom): (f32, f32), ease: Ease) {
        let gap = 0.07;
        for side in [-1.0, 1.0] {
            let edge = [(top, angle + side * gap), (bottom, angle + side * gap)];
            self.line(model, Stuff::Shade, &edge, ease, 3.0);
        }
        let crossings = ((top - bottom) / 30.0).floor().max(2.0) as usize;
        for at in 0..crossings {
            let (high, low) = (
                Self::between(top, bottom, at as f32 / crossings as f32),
                Self::between(top, bottom, (at + 1) as f32 / crossings as f32),
            );
            let side = if at % 2 == 0 { 1.0 } else { -1.0 };
            let cord = [(high, angle - side * gap), (low, angle + side * gap)];
            self.line(model, Stuff::Backing, &cord, &|h, a| ease(h, a) + 1.0, 1.6);
        }
    }
}
