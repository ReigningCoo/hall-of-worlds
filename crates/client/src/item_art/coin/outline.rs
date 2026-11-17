//! `Outline`: a flat 2D shape on the coin's face with a height, raised with a
//! bevelled edge (or pressed in, for a dent), and `relief`: the face's height at
//! any point from a stack of them, lowest first.

use bevy::prelude::*;

/// How far in from an outline's edge it takes to reach its full height, in the
/// face's units (its radius is 1).
const BEVEL: f32 = 0.035;
/// Where the relief starts fading to the field, so nothing climbs the rim.
const FADE_FROM: f32 = 0.95;

/// What an outline does to what is under it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Raise {
    /// Raised to this height (0 the field, 1 the highest), over what is lower.
    To(f32),
    /// Pressed in by this much from whatever is there.
    Dent(f32),
}

#[derive(Clone, Debug)]
pub(super) struct Outline {
    /// Corners in order round it, x right and y up, in the face's units.
    pub(super) points: Vec<Vec2>,
    pub(super) raise: Raise,
}

impl Outline {
    pub(super) fn new(points: &[(f32, f32)], raise: Raise) -> Self {
        let points = points.iter().map(|&(x, y)| Vec2::new(x, y)).collect();
        Self { points, raise }
    }

    /// The same outline turned over left to right.
    pub(super) fn mirrored(&self) -> Self {
        let points = self.points.iter().rev().map(|p| Vec2::new(-p.x, p.y));
        Self {
            points: points.collect(),
            raise: self.raise,
        }
    }

    /// A shape made of its right half, from the top middle down to the bottom
    /// middle, and that half turned over for the left.
    pub(super) fn symmetric(right: &[(f32, f32)], raise: Raise) -> Self {
        let mut points: Vec<Vec2> = right.iter().map(|&(x, y)| Vec2::new(x, y)).collect();
        let left = right.iter().rev().filter(|p| p.0 != 0.0);
        points.extend(left.map(|&(x, y)| Vec2::new(-x, y)));
        Self { points, raise }
    }

    /// An oval round `middle`, as many-sided as a small shape needs.
    pub(super) fn oval(middle: (f32, f32), half: (f32, f32), raise: Raise) -> Self {
        let points = (0..16).map(|i| {
            let angle = i as f32 / 16.0 * std::f32::consts::TAU;
            Vec2::new(
                middle.0 + half.0 * angle.cos(),
                middle.1 + half.1 * angle.sin(),
            )
        });
        Self {
            points: points.collect(),
            raise,
        }
    }

    /// How far inside its edge `at` is, or `None` outside.
    fn depth(&self, at: Vec2) -> Option<f32> {
        let mut inside = false;
        let mut nearest = f32::MAX;
        let count = self.points.len();
        for i in 0..count {
            let (a, b) = (self.points[i], self.points[(i + 1) % count]);
            if (a.y > at.y) != (b.y > at.y) && at.x < a.x + (at.y - a.y) / (b.y - a.y) * (b.x - a.x)
            {
                inside = !inside;
            }
            let along = ((at - a).dot(b - a) / (b - a).length_squared()).clamp(0.0, 1.0);
            nearest = nearest.min(at.distance(a + (b - a) * along));
        }
        inside.then_some(nearest)
    }

    /// How much of its height it has at `at`: 0 outside, rising over the bevel
    /// with a rounded shoulder, 1 inside that.
    fn share(&self, at: Vec2) -> f32 {
        let Some(depth) = self.depth(at) else {
            return 0.0;
        };
        let s = (depth / BEVEL).min(1.0);
        s * (2.0 - s)
    }
}

/// The face's height at `at` (0 the field, 1 the highest), from outlines lowest
/// first, fading to the field near the rim.
pub(super) fn relief(outlines: &[Outline], at: Vec2) -> f32 {
    let mut height: f32 = 0.0;
    for outline in outlines {
        let share = outline.share(at);
        if share == 0.0 {
            continue;
        }
        height = match outline.raise {
            Raise::To(top) => height.max(top * share),
            Raise::Dent(depth) => (height - depth * share).max(0.0),
        };
    }
    let fade = ((1.0 - at.length()) / (1.0 - FADE_FROM)).clamp(0.0, 1.0);
    height * fade
}
