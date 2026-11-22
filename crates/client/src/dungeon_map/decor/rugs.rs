//! Rugs lying on the floor: thin layers, each a little over the one under it,
//! in its frame from the north-west corner of its cells (`x` east, `z`
//! south). Patterns are simple shapes in the rug's two colours.

use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;
use shared::decor::{Rug, RugPattern};

use super::super::CELL_SIZE;
use super::colours;
use super::model::Model;

const C: f32 = CELL_SIZE;
/// How far a rug stands in from the edges of its cells.
const INSET: f32 = C * 0.1;
/// How thick each layer is.
const LAYER: f32 = 0.12;

pub(super) fn rug(model: &mut Model, (width, height): (u8, u8), rug: Rug) {
    let (main, other) = colours::rug(rug.colour);
    let size = Vec2::new(f32::from(width), f32::from(height)) * C - 2.0 * INSET;
    let middle = Vec3::new(f32::from(width), 0.0, f32::from(height)) * C / 2.0;
    // A flat box `inset` in from the rug's edges, layer `n` up.
    let layer = |model: &mut Model, inset: Vec2, n: f32, colour: Vec3| {
        let half = (size / 2.0 - inset).max(Vec2::splat(0.5));
        let low = middle + Vec3::new(-half.x, LAYER * n, -half.y);
        let high = middle + Vec3::new(half.x, LAYER * (n + 1.0), half.y);
        model.block(low, high, colour);
    };
    match rug.pattern {
        RugPattern::Plain => {
            layer(model, Vec2::ZERO, 0.0, main);
            fringe(model, size, middle, other);
        }
        RugPattern::Striped => {
            layer(model, Vec2::ZERO, 0.0, main);
            // Bands across its length.
            let east_west = size.x >= size.y;
            let long = size.x.max(size.y);
            let bands = (long / (0.35 * C)).floor().max(1.0) as i32;
            for i in 0..bands {
                let at = -long / 2.0 + long * (i as f32 + 0.5) / bands as f32;
                let (offset, extent) = if east_west {
                    (Vec3::X * at, Vec3::new(0.04 * C, LAYER, size.y))
                } else {
                    (Vec3::Z * at, Vec3::new(size.x, LAYER, 0.04 * C))
                };
                let centre = middle + offset + Vec3::Y * LAYER * 1.5;
                model.cuboid(centre, extent, Quat::IDENTITY, other);
            }
        }
        RugPattern::Bordered | RugPattern::Medallion => {
            layer(model, Vec2::ZERO, 0.0, other);
            layer(model, Vec2::splat(0.12 * C), 1.0, main);
            if rug.pattern == RugPattern::Medallion {
                let side = size.min_element() * 0.38;
                let centre = middle + Vec3::Y * LAYER * 2.5;
                let turn = Quat::from_rotation_y(FRAC_PI_4);
                model.cuboid(centre, Vec3::new(side, LAYER, side), turn, other);
            }
        }
        RugPattern::Braided => {
            // Rings of braid, outermost first.
            for (i, share) in [1.0f32, 0.72, 0.46, 0.2].into_iter().enumerate() {
                let colour = if i % 2 == 0 { main } else { other };
                let at = middle + Vec3::Y * LAYER * (i as f32 + 0.5);
                let across = Vec3::new(size.x * share, LAYER, size.y * share);
                model.oval(at, across, colour);
            }
        }
    }
}

/// Tassels along the two short ends.
fn fringe(model: &mut Model, size: Vec2, middle: Vec3, colour: Vec3) {
    let east_west = size.x >= size.y;
    let (long, short) = (size.x.max(size.y), size.x.min(size.y));
    for side in [-1.0, 1.0] {
        let reach = side * (long / 2.0 + 0.03 * C);
        let (offset, extent) = if east_west {
            (Vec3::X * reach, Vec3::new(0.06 * C, LAYER * 0.5, short))
        } else {
            (Vec3::Z * reach, Vec3::new(short, LAYER * 0.5, 0.06 * C))
        };
        model.cuboid(
            middle + offset + Vec3::Y * LAYER * 0.25,
            extent,
            Quat::IDENTITY,
            colour,
        );
    }
}
