//! A pen's own wood (`shared::site::PenLook`): a sty's low shelter, a yard's
//! coop and a dovecote on its post, built of blocks like the fences.

use bevy::prelude::*;
use shared::site::{PenKind, PenLook};

use super::super::CELL_SIZE;
use super::super::town;
use super::super::walls::{Layer, Layers, Scene};
use super::{HALF_CELL, post, wood};

/// How thick a shelter's boards are, and how far it stands in from the fence.
const BOARD: f32 = CELL_SIZE * 0.05;
const INSET: f32 = CELL_SIZE * 0.08;
/// How high a sty's walls and a coop's are, and how high a coop stands.
const STY_HEIGHT: f32 = CELL_SIZE * 0.45;
const COOP_HEIGHT: f32 = CELL_SIZE * 0.5;
const COOP_LEGS: f32 = CELL_SIZE * 0.15;
/// A roof's steps, each this much lower towards the open front.
const ROOF_STEP: f32 = CELL_SIZE * 0.06;
const ROOF_STEPS: usize = 3;
/// A dovecote: its post, its box and its roof.
const DOVECOTE_POST: f32 = CELL_SIZE * 1.1;
const DOVECOTE_HALF: f32 = CELL_SIZE * 0.22;
const DOVECOTE_BOX: f32 = CELL_SIZE * 0.3;

/// A pen's shelter, drawn from its first cell, and a dovecote on its own.
pub fn build_pen(layers: &mut Layers, scene: &Scene, (x, y): (i64, i64)) {
    let Some(field) = scene.fields.iter().find(|field| field.contains(x, y)) else {
        return;
    };
    let Some(pen) = field.pen else {
        return;
    };
    let layer = town::flat_layer(scene, x, y);
    if (x, y) == (pen.shelter.x, pen.shelter.y) {
        shelter(layers, scene, layer, &pen);
    }
    if Some((x, y)) == pen.dovecote {
        dovecote(layers, scene, layer, town::cell_centre(x, y));
    }
}

/// A sty's lean-to (walls at the back and ends, open in front) or a coop
/// (closed, on legs), under a roof stepping down to the front.
fn shelter(layers: &mut Layers, scene: &Scene, layer: Layer, pen: &PenLook) {
    let r = pen.shelter;
    let low = town::cell_centre(r.x, r.y) - Vec3::new(HALF_CELL - INSET, 0.0, HALF_CELL - INSET);
    let far = town::cell_centre(r.x + r.width - 1, r.y + r.height - 1);
    let high = far + Vec3::new(HALF_CELL - INSET, 0.0, HALF_CELL - INSET);
    let coop = pen.kind == PenKind::Yard;
    let (bottom, height) = if coop {
        (COOP_LEGS, COOP_HEIGHT)
    } else {
        (0.0, STY_HEIGHT)
    };
    let top = bottom + height;
    // Its four sides, as a box from `low` to `high` seen from above: which
    // is the back (north, east, south, west) and which the front.
    let side = |which: u8| -> (Vec3, Vec3) {
        let (lo, hi) = (low.with_y(bottom), high.with_y(top));
        match which % 4 {
            0 => (lo, hi.with_z(lo.z + BOARD)),
            1 => (lo.with_x(hi.x - BOARD), hi),
            2 => (lo.with_z(hi.z - BOARD), hi),
            _ => (lo, hi.with_x(lo.x + BOARD)),
        }
    };
    let back = pen.back;
    for which in [back, back + 1, back + 3] {
        let (lo, hi) = side(which);
        wood(layers, scene, layer, lo, hi);
    }
    if coop {
        // Closed in front but for a low door, and standing on legs.
        let (lo, hi) = side(back + 2);
        wood(layers, scene, layer, lo.with_y(top - height * 0.4), hi);
        let (x0, x1, z0, z1) = (low.x + BOARD, high.x - BOARD, low.z + BOARD, high.z - BOARD);
        for (x, z) in [(x0, z0), (x1, z0), (x0, z1), (x1, z1)] {
            post(layers, scene, layer, Vec3::new(x, 0.0, z), bottom);
        }
        let floor = (low.with_y(bottom - BOARD), high.with_y(bottom));
        wood(layers, scene, layer, floor.0, floor.1);
    }
    // The roof: steps, the highest over the back, the lowest reaching the front.
    for step in 0..ROOF_STEPS {
        let share = (step + 1) as f32 / ROOF_STEPS as f32;
        let y = top + ROOF_STEP * (ROOF_STEPS - 1 - step) as f32;
        let over = BOARD * 2.0;
        let (mut lo, mut hi) = (low - Vec3::splat(over), high + Vec3::splat(over));
        let depth_x = (hi.x - lo.x) * share;
        let depth_z = (hi.z - lo.z) * share;
        match back % 4 {
            0 => hi.z = lo.z + depth_z,
            1 => lo.x = hi.x - depth_x,
            2 => lo.z = hi.z - depth_z,
            _ => hi.x = lo.x + depth_x,
        }
        wood(layers, scene, layer, lo.with_y(y), hi.with_y(y + BOARD));
    }
}

/// A dovecote: a box on a tall post, under a roof wider than it.
fn dovecote(layers: &mut Layers, scene: &Scene, layer: Layer, at: Vec3) {
    post(layers, scene, layer, at, DOVECOTE_POST);
    let half = Vec3::new(DOVECOTE_HALF, 0.0, DOVECOTE_HALF);
    let base = at + Vec3::Y * DOVECOTE_POST;
    wood(
        layers,
        scene,
        layer,
        base - half,
        base + half + Vec3::Y * DOVECOTE_BOX,
    );
    let mut top = base + Vec3::Y * DOVECOTE_BOX;
    for grow in [1.3, 0.9, 0.5] {
        let roof = half * grow;
        wood(
            layers,
            scene,
            layer,
            top - roof,
            top + roof + Vec3::Y * BOARD * 1.5,
        );
        top += Vec3::Y * BOARD * 1.5;
    }
}
