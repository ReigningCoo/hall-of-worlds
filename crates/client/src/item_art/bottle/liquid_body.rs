//! The liquid's body: the bottle's own shape a little inside its glass, cut
//! level at the top, so it shows between the glass's far and near sides. Bands
//! of it (`from` to `to`, shares of the full height) make layers.

use bevy::prelude::*;

use super::heart;
use super::look::Shape;
use super::profiles::{Profile, profile};
use super::surface::Surface;

/// How much of the glass's width the liquid fills.
const INSET: f32 = 0.9;
/// How far above the glass's bottom it starts.
const BOTTOM: f32 = 2.0;
/// How high it comes, against the shoulder (where the neck starts).
const FILL: f32 = 0.85;
/// The heart's: just under its widest.
const HEART_FILL: f32 = 44.0;
/// The middle the heart's liquid is drawn in towards.
const HEART_MIDDLE: Vec3 = Vec3::new(0.0, 30.0, 0.0);

/// The bottom and top of the liquid in a bottle of this shape.
pub(super) fn levels(shape: Shape) -> (f32, f32) {
    let top = match shape {
        Shape::Heart => HEART_FILL,
        _ => profile(shape).shoulder * FILL,
    };
    (BOTTOM, top)
}

/// The liquid from `from` to `to` of the way up.
pub(super) fn band(shape: Shape, from: f32, to: f32) -> Surface {
    let (bottom, top) = levels(shape);
    let low = bottom + (top - bottom) * from;
    let high = bottom + (top - bottom) * to;
    match shape {
        Shape::Heart => heart_band(low, high),
        _ => lathed_band(&profile(shape), low, high),
    }
}

/// The heart's pillow drawn in and squashed between the two heights: as it
/// only widens from its tip to above `HEART_FILL`, nothing squashed sticks out.
fn heart_band(low: f32, high: f32) -> Surface {
    let mut body = heart::heart_body();
    for point in body.rows.iter_mut().flatten() {
        let at = HEART_MIDDLE + (*point - HEART_MIDDLE) * INSET;
        *point = Vec3::new(at.x, at.y.clamp(low, high), at.z);
    }
    body
}

/// The outline cut between the two heights, drawn in, closed flat at both, with
/// each edge ring twice so the edges stay sharp.
fn lathed_band(outline: &Profile, low: f32, high: f32) -> Surface {
    let points = &outline.points;
    let radius = |y: f32| {
        let i = points.windows(2).position(|pair| pair[1].y >= y);
        match i {
            Some(i) => {
                let (a, b) = (points[i], points[i + 1]);
                let t = if b.y > a.y {
                    (y - a.y) / (b.y - a.y)
                } else {
                    1.0
                };
                a.x + (b.x - a.x) * t.clamp(0.0, 1.0)
            }
            None => points[points.len() - 1].x,
        }
    };
    let mut cut = vec![Vec2::new(0.0, low)];
    cut.extend([Vec2::new(radius(low), low); 2]);
    cut.extend(points.iter().filter(|p| p.y > low && p.y < high).copied());
    cut.extend([Vec2::new(radius(high), high); 2]);
    cut.push(Vec2::new(0.0, high));
    let inset = Profile {
        points: cut
            .into_iter()
            .map(|p| Vec2::new(p.x * INSET, p.y))
            .collect(),
        section: outline.section,
        shoulder: outline.shoulder,
    };
    Surface::lathe(&inset)
}
