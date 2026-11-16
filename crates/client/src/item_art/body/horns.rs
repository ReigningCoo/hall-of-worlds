//! Horns on a measured head, from the top of the skull outside the ears, each a path
//! of three points by its shape (`shared::body::Horns`), thinning to its tip.

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
pub(in crate::item_art) fn horns_of(layout: &Layout) -> Vec<[(Vec3, f32); 3]> {
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
        Horns::None | Horns::Back | Horns::Bull => 0.12,
    };
    (horn * share).max(8.0)
}

/// Each horn's middle and tip from its root, as shares of its length out to its side
/// (`x`), up and forwards, with how thick it is at its middle and tip as shares of
/// its root's; `None` for no horns.
fn bends(horns: Horns) -> Option<[(Vec3, f32); 2]> {
    Some(match horns {
        Horns::None => return None,
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
    })
}

fn paths(layout: &Layout, shape: &HeadShape, axes: &HeadAxes) -> Vec<[(Vec3, f32); 3]> {
    let parts = layout.parts;
    let Some([middle, tip]) = bends(parts.horns) else {
        return Vec::new();
    };
    let horn = parts.horn_length as f32;
    let skull = crown(shape);
    let (along, top, half) = (skull.along as f32, skull.back as f32, skull.half as f32);
    let thick = thickness(parts.horns, horn);
    Side::BOTH
        .into_iter()
        .map(|side| {
            let out = Vec3::new(side.sign() as f32, 1.0, 1.0);
            let base = axes.at(along, top * 0.85, out.x * half * 0.7);
            let at = |(bend, _): (Vec3, f32)| base + bend * out * horn;
            [
                (base, thick),
                (at(middle), thick * middle.1),
                // A blunt point, as the old horns' were.
                (at(tip), if parts.horns == Horns::Back { 2.0 } else { 3.0 }),
            ]
        })
        .collect()
}
