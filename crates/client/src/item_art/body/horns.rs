//! Horns on a measured head, from the top of the skull outside the ears, each a path
//! of three points by its shape (`shared::body::Horns`), thinning to its tip; a ram's
//! a coil of more, back, down and round beside the face.

use bevy::prelude::*;
use shared::body::{HeadShape, Horns, Layout, Side};

use super::{HeadAxes, Triangle, crown, limb};

/// The horns, if the head has any.
pub(super) fn build(
    triangles: &mut Vec<Triangle>,
    layout: &Layout,
    shape: &HeadShape,
    axes: &HeadAxes,
) {
    for path in paths(layout, shape, axes) {
        limb(triangles, &path);
    }
}

/// Where a shaped head's horns run, one path for each, from the root out; none for a
/// head without horns or without a shape.
pub(in crate::item_art) fn horns_of(layout: &Layout) -> Vec<Vec<(Vec3, f32)>> {
    match &layout.head.shape {
        Some(shape) => paths(layout, shape, &HeadAxes::of(layout, shape)),
        None => Vec::new(),
    }
}

/// How thick a horn of this shape is at its root, for its length.
fn thickness(horns: Horns, horn: f32) -> f32 {
    let share = match horns {
        Horns::Upturned => 0.09,
        Horns::Forward => 0.17,
        Horns::Spiral => 0.11,
        Horns::Sweep => 0.1,
        Horns::None | Horns::Back | Horns::Bull => 0.12,
    };
    (horn * share).max(8.0)
}

/// Each horn's middle and tip from its root, as shares of its length out to its side
/// (`x`), up and forwards, with how thick it is at its middle and tip as shares of
/// its root's; `None` for no horns.
fn bends(horns: Horns) -> Option<[(Vec3, f32); 2]> {
    Some(match horns {
        Horns::None | Horns::Spiral => return None,
        Horns::Bull => [
            (Vec3::new(0.55, 0.05, 0.05), 0.7),
            (Vec3::new(0.75, 0.45, 0.1), 0.0),
        ],
        Horns::Back => [
            (Vec3::new(0.15, 0.4, -0.5), 0.6),
            (Vec3::new(0.25, 0.45, -0.95), 0.0),
        ],
        Horns::Upturned => [
            (Vec3::new(0.6, 0.08, 0.1), 0.65),
            (Vec3::new(0.85, 0.5, -0.05), 0.0),
        ],
        Horns::Forward => [
            (Vec3::new(0.55, 0.0, 0.12), 0.7),
            (Vec3::new(0.68, 0.15, 0.5), 0.0),
        ],
        Horns::Sweep => [
            (Vec3::new(0.65, -0.1, -0.25), 0.7),
            (Vec3::new(0.8, 0.1, -0.7), 0.0),
        ],
    })
}

fn paths(layout: &Layout, shape: &HeadShape, axes: &HeadAxes) -> Vec<Vec<(Vec3, f32)>> {
    let parts = layout.parts;
    let horn = parts.horn_length as f32;
    let skull = crown(shape);
    let (along, top, half) = (skull.along as f32, skull.back as f32, skull.half as f32);
    let thick = thickness(parts.horns, horn);
    if parts.horns == Horns::Spiral && horn > 0.0 {
        return Side::BOTH
            .into_iter()
            .map(|side| coil(axes, (along, top, half), horn, thick, side))
            .collect();
    }
    let Some([middle, tip]) = bends(parts.horns) else {
        return Vec::new();
    };
    Side::BOTH
        .into_iter()
        .map(|side| {
            let out = Vec3::new(side.sign() as f32, 1.0, 1.0);
            let base = axes.at(along, top * 0.85, out.x * half * 0.7);
            let at = |(bend, _): (Vec3, f32)| base + bend * out * horn;
            vec![
                (base, thick),
                (at(middle), thick * middle.1),
                // A blunt point, as the old horns' were.
                (at(tip), if parts.horns == Horns::Back { 2.0 } else { 3.0 }),
            ]
        })
        .collect()
}

/// Points along a ram's coil.
const COIL_POINTS: usize = 12;
/// How many turns it makes.
const COIL_TURNS: f32 = 1.2;

/// A ram's horn: from the top of the skull, back, down past the ear and forwards
/// round beside the face, tighter as it goes and drifting out from the head, as
/// long as `horn` along its curve.
fn coil(
    axes: &HeadAxes,
    (along, top, half): (f32, f32, f32),
    horn: f32,
    thick: f32,
    side: Side,
) -> Vec<(Vec3, f32)> {
    let out = side.sign() as f32;
    // Its curve is about 5 times its first radius long.
    let first = horn / 5.2;
    let (start_along, start_up) = (along, top * 0.85);
    (0..=COIL_POINTS)
        .map(|step| {
            let t = step as f32 / COIL_POINTS as f32;
            let turned = t * COIL_TURNS * std::f32::consts::TAU;
            let radius = first * (1.0 - 0.45 * t);
            // From the top of its circle, round towards the back, then down.
            let (up, back) = (radius * turned.cos(), radius * turned.sin());
            let across = out * (half * 0.6 + horn * (0.06 + 0.2 * t));
            let at = axes.at(start_along - back, start_up - first + up, across);
            (at, thick * (1.0 - 0.75 * t).max(0.2))
        })
        .collect()
}
