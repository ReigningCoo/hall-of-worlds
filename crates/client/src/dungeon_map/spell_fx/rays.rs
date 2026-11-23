//! The shapes of the wheres that happen at once: Touch's arc swept in front,
//! Surround's ring and dome, and Beam's rays, each fading quickly, with
//! sparkles scattered along them.

use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::prelude::*;
use shared::facing::Heading;
use shared::spell::{Shown, touch_half_arc};

use super::super::CELL_SIZE;
use super::fade::Grow;
use super::paint::Paint;
use super::place::{CHEST, radians, spot_drawn, units_drawn};
use crate::common::DrawnGround;

/// How long each shape lasts, in seconds.
const ARC_LIFE: f32 = 0.35;
const RING_LIFE: f32 = 0.5;
const BEAM_LIFE: f32 = 0.45;

/// A beam's core's radius and its glow's, in cells.
const BEAM_CORE: f32 = 0.03;
const BEAM_GLOW: f32 = 0.09;

/// Sparkles a cell along a shape, and how big, in cells.
const SPARKS_A_CELL: f32 = 6.0;
const SPARK: f32 = 0.035;

/// Touch: a flat arc at chest height, as wide as its count makes it, from
/// the body out to arm's reach.
pub(super) fn touch_arc(
    paint: &mut Paint,
    ground: &DrawnGround,
    shown: &Shown,
    colours: &[LinearRgba],
) {
    let half = radians(Heading(touch_half_arc(shown.count)));
    let reach = units_drawn(u64::from(shown.reach));
    let mesh = paint.add_mesh(CircularSector::new(reach, half));
    let centre = spot_drawn(ground, shown.from, CHEST);
    // The sector points up `y` in its own plane: laid flat (pointing north,
    // `-z`), then turned to face the way the touch went.
    let flat = Quat::from_rotation_x(-FRAC_PI_2);
    let turned = Quat::from_rotation_y(-(radians(shown.heading) + FRAC_PI_2));
    let transform = Transform::from_translation(centre).with_rotation(turned * flat);
    paint.fading(mesh, transform, colours[0] * 0.6, (ARC_LIFE, Grow::Spread));
    let sparks = (reach / CELL_SIZE * half * 2.0 * SPARKS_A_CELL) as usize + 4;
    for n in 0..sparks {
        let across = (paint.random() * 2.0 - 1.0) * half;
        let angle = radians(shown.heading) + across;
        let out = Vec3::new(angle.cos(), 0.0, angle.sin());
        let at = centre + out * reach * (0.6 + 0.4 * paint.random());
        let colour = colours[n % colours.len()];
        paint.sparkle(at, out * 0.6 * CELL_SIZE, colour, SPARK * CELL_SIZE);
    }
}

/// Surround: a ring on the ground at its reach, spreading out, under a faint
/// dome, with sparkles round its edge.
pub(super) fn ring(paint: &mut Paint, ground: &DrawnGround, shown: &Shown, colours: &[LinearRgba]) {
    let reach = units_drawn(u64::from(shown.reach));
    let centre = spot_drawn(ground, shown.from, 0.08);
    let band = 0.06 * CELL_SIZE;
    let mesh = paint.add_mesh(Torus::new((reach - band).max(band), reach));
    let transform = Transform::from_translation(centre);
    paint.fading(mesh, transform, colours[0], (RING_LIFE, Grow::Spread));
    let ball = paint.shared().ball;
    let size = Vec3::new(reach, reach * 0.6, reach);
    let dome = Transform::from_translation(centre).with_scale(size);
    let soft = colours.get(1).copied().unwrap_or(colours[0]) * 0.12;
    paint.fading(ball, dome, soft, (RING_LIFE, Grow::Spread));
    let sparks = (reach / CELL_SIZE * TAU * SPARKS_A_CELL) as usize;
    for n in 0..sparks {
        let angle = paint.random() * TAU;
        let out = Vec3::new(angle.cos(), 0.0, angle.sin());
        let at = centre + out * reach + Vec3::Y * paint.random() * 0.4 * CELL_SIZE;
        let rise = (out * 0.3 + Vec3::Y * 0.5) * CELL_SIZE;
        paint.sparkle(at, rise, colours[n % colours.len()], SPARK * CELL_SIZE);
    }
}

/// Beam: a bright ray to each beam's end in a soft glow, sparkles along it.
pub(super) fn beams(
    paint: &mut Paint,
    ground: &DrawnGround,
    shown: &Shown,
    colours: &[LinearRgba],
) {
    let from = spot_drawn(ground, shown.from, CHEST);
    let rod = paint.shared().rod;
    for &end in &shown.ends {
        let to = spot_drawn(ground, end, CHEST);
        let length = from.distance(to);
        if length < 0.01 {
            continue;
        }
        let along = (to - from) / length;
        let turned = Quat::from_rotation_arc(Vec3::Y, along);
        let middle = (from + to) / 2.0;
        let layers = [
            (BEAM_CORE, colours[0] * 1.5),
            (BEAM_GLOW, colours[0] * 0.35),
        ];
        for (radius, colour) in layers {
            let size = Vec3::new(radius * CELL_SIZE, length, radius * CELL_SIZE);
            let transform = Transform::from_translation(middle)
                .with_rotation(turned)
                .with_scale(size);
            paint.fading(rod.clone(), transform, colour, (BEAM_LIFE, Grow::Steady));
        }
        let sparks = (length / CELL_SIZE * SPARKS_A_CELL) as usize + 2;
        for n in 0..sparks {
            let at = from + along * length * paint.random();
            let drift = paint.random_way() * 0.3 * CELL_SIZE;
            paint.sparkle(at, drift, colours[n % colours.len()], SPARK * CELL_SIZE);
        }
    }
}
