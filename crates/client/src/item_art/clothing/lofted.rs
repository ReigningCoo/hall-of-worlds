//! Stiff and wound shapes built from rings: the crowns of caps and hats, hennins and
//! the rolls of an escoffion, turbans' wraps and the chaperon's padded roll.

use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::item_art::armour::{Model, Stuff};
use crate::item_art::body::loft;

/// Points round a ring.
pub(super) const POINTS: usize = 32;
/// Points round a roll's tube.
const TUBE: usize = 10;

/// A ring round `centre`, half `across` and `deep`, tipped about the body's left-right
/// line by `tip` (back as it grows), each point pushed out by `swell(angle)`; angles go
/// from the front towards the left.
pub(super) fn oval_by(
    centre: Vec3,
    (across, deep): (f32, f32),
    tip: f32,
    swell: &dyn Fn(f32) -> f32,
) -> Vec<Vec3> {
    let turn = Quat::from_rotation_x(-tip);
    (0..POINTS)
        .map(|at| {
            let angle = at as f32 / POINTS as f32 * TAU;
            let more = swell(angle);
            let flat = Vec3::new(
                (across + more) * angle.sin(),
                0.0,
                (deep + more) * angle.cos(),
            );
            centre + turn * flat
        })
        .collect()
}

/// The same, even all round.
pub(super) fn oval(centre: Vec3, half: (f32, f32), tip: f32) -> Vec<Vec3> {
    oval_by(centre, half, tip, &|_| 0.0)
}

/// A solid through these rings, closed at both ends.
pub(super) fn solid(model: &mut Model, stuff: Stuff, rings: &[Vec<Vec3>]) {
    loft(model.of(stuff), rings);
}

/// A roll `tube` thick along a closed ring.
pub(super) fn roll(model: &mut Model, stuff: Stuff, ring: &[Vec3], tube: f32) {
    let count = ring.len();
    let centre = ring.iter().copied().sum::<Vec3>() / count as f32;
    // The ring's own up, square to it.
    let up = (ring[0] - centre)
        .cross(ring[count / 4] - centre)
        .normalize_or(Vec3::Y);
    let up = if up.y < 0.0 { -up } else { up };
    let mut sections: Vec<Vec<Vec3>> = (0..=count)
        .map(|at| {
            let point = ring[at % count];
            let out = (point - centre).reject_from(up).normalize_or(Vec3::X);
            (0..TUBE)
                .map(|step| {
                    let turn = step as f32 / TUBE as f32 * TAU;
                    point + (out * turn.cos() + up * turn.sin()) * tube
                })
                .collect()
        })
        .collect();
    // Close the seam onto the first section exactly.
    sections[count] = sections[0].clone();
    loft(model.of(stuff), &sections);
}
