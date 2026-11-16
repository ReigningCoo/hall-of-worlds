//! A fringe (`shared::body::Parts::fringe`): long shaggy hair over the brow, as a
//! highland cow's, drawn as overlapping lumps across the forehead hanging down over
//! the eyes, in the coat's colour.

use bevy::prelude::*;
use shared::body::{HeadShape, Layout};

use super::{HeadAxes, Triangle, blob, crown};

/// How many lumps across the brow.
const LUMPS: usize = 5;

/// The fringe, if the head has one.
pub(super) fn build(
    triangles: &mut Vec<Triangle>,
    layout: &Layout,
    shape: &HeadShape,
    axes: &HeadAxes,
) {
    let hang = layout.parts.fringe as f32;
    if hang <= 0.0 {
        return;
    }
    let skull = crown(shape);
    let (along, top, half) = (skull.along as f32, skull.back as f32, skull.half as f32);
    // From the crown forwards over the brow, each lump hanging down the face.
    let brow = along + hang * 0.6;
    for lump in 0..LUMPS {
        let across = (lump as f32 / (LUMPS - 1) as f32 - 0.5) * half * 1.5;
        // The middle ones hang furthest.
        let long = 1.0 - (across / (half * 1.2)).powi(2) * 0.4;
        let middle = axes.at(brow, top - hang * 0.35 * long, across);
        let size = Vec3::new(half * 0.32, hang * 0.5 * long, hang * 0.4);
        blob(triangles, middle, size);
    }
}
