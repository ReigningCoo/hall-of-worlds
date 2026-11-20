//! Humps on the back (`shared::body::Parts::humps`, `hump`): one in the middle of a
//! dromedary's back, or two, one over the shoulders and one over the hips, as a
//! Bactrian camel's; each a lump rising from the top of the trunk.

use bevy::prelude::*;
use shared::body::Layout;

use super::trunk::{self, Axes};
use super::{Triangle, blob};

/// The humps, if the back has any.
pub(super) fn build(triangles: &mut Vec<Triangle>, layout: &Layout) {
    let tall = layout.parts.hump as f32;
    if tall <= 0.0 {
        return;
    }
    let places: &[f32] = match layout.parts.humps {
        0 => return,
        1 => &[0.55],
        _ => &[0.25, 0.8],
    };
    let axes = Axes::of(layout);
    let body = layout.spine.shoulders as f32;
    let long = body / (places.len() as f32 * 2.2);
    for &share in places {
        let along = body * share;
        let ring = trunk::ring_at(&layout.spine.rings, along);
        let middle = trunk::middle_of(&axes, &ring) + axes.along * (along - ring.along as f32);
        let top = middle - axes.front * (ring.belly + ring.back) as f32 / 2.0;
        blob(
            triangles,
            top - axes.front * tall * 0.3,
            Vec3::new(ring.half as f32 * 0.6, tall * 0.7, long),
        );
    }
}
