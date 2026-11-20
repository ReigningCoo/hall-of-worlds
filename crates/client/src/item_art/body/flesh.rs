//! Bare red skin on a bird's measured head, drawn apart from its feathers in its own
//! colour: a comb along the top of the skull, wattles hanging under the beak, a
//! turkey's snood hanging from the forehead over its beak, and caruncles, knobbly
//! lumps over the face and round the beak's base.

use std::f32::consts::PI;

use bevy::prelude::*;
use shared::body::{HEAD_RINGS, HeadShape, Layout, Side};

use super::{HeadAxes, Triangle, blob, limb};

/// The bare skin this layout's head has, if any.
pub(in crate::item_art) fn flesh(layout: &Layout) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    let Some(shape) = layout.head.shape else {
        return triangles;
    };
    let axes = HeadAxes::of(layout, &shape);
    let parts = layout.parts;
    if parts.comb > 0 {
        comb(&mut triangles, parts.comb as f32, &shape, &axes);
    }
    if parts.wattles > 0 {
        wattles(&mut triangles, parts.wattles as f32, &shape, &axes);
    }
    if parts.snood > 0 {
        snood(&mut triangles, parts.snood as f32, &shape, &axes);
    }
    if parts.caruncles > 0 {
        caruncles(&mut triangles, parts.caruncles as f32, &shape, &axes);
    }
    triangles
}

/// A row of lobes along the top of the skull.
fn comb(triangles: &mut Vec<Triangle>, tall: f32, shape: &HeadShape, axes: &HeadAxes) {
    for (at, lobe) in [(2, 0.8), (3, 1.0), (4, 0.75)] {
        let ring = shape.rings[at];
        let top = axes.at(
            ring.along as f32,
            ring.back as f32 + tall * 0.35 * lobe,
            0.0,
        );
        blob(
            triangles,
            top,
            Vec3::new(3.0, tall * 0.5 * lobe, tall * 0.3),
        );
    }
}

/// Two wattles hanging under the beak.
fn wattles(triangles: &mut Vec<Triangle>, long: f32, shape: &HeadShape, axes: &HeadAxes) {
    let under = shape.rings[HEAD_RINGS - 3];
    for side in Side::BOTH {
        let across = side.sign() as f32 * under.half as f32 * 0.4;
        let hang = axes.at(
            under.along as f32,
            -(under.belly as f32) - long * 0.3,
            across,
        );
        blob(
            triangles,
            hang,
            Vec3::new(3.0, long * 0.35 * (PI / 4.0), long * 0.2),
        );
    }
}

/// From the forehead above the beak's base, forwards over it and hanging down one
/// side of it.
fn snood(triangles: &mut Vec<Triangle>, long: f32, shape: &HeadShape, axes: &HeadAxes) {
    let brow = shape.rings[HEAD_RINGS - 3];
    let nose = shape.rings[HEAD_RINGS - 1];
    let root = axes.at(brow.along as f32, brow.back as f32, 0.0);
    let over = axes.at(
        nose.along as f32,
        nose.back as f32 + long * 0.1,
        long * 0.08,
    );
    let tip = over + Vec3::new(long * 0.12, -long * 0.75, long * 0.08);
    limb(
        triangles,
        &[
            (root, long * 0.07),
            (over, long * 0.08),
            (over.lerp(tip, 0.5), long * 0.09),
            (tip, long * 0.06),
        ],
    );
}

/// Knobbly lumps over the face's sides and round the beak's base.
fn caruncles(triangles: &mut Vec<Triangle>, thick: f32, shape: &HeadShape, axes: &HeadAxes) {
    for ring in &shape.rings[HEAD_RINGS / 2..HEAD_RINGS - 1] {
        let mid = (ring.back - ring.belly) as f32 / 2.0;
        let deep = (ring.belly + ring.back) as f32 / 2.0;
        for side in Side::BOTH {
            let out = side.sign() as f32;
            for up in [-0.4, 0.3] {
                let at = axes.at(
                    ring.along as f32,
                    mid + deep * up,
                    out * ring.half as f32 * 0.9,
                );
                blob(triangles, at, Vec3::splat(thick));
            }
        }
    }
}
