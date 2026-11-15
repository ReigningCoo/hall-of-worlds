//! The strand: a cross-section swept along the path, one tube with no links of its
//! own. Its faces are shaded to fake what it is made of: a cable chain's links lying
//! flat and edge on in turn, a rope chain's twist, a box chain's square links, a snake
//! chain's fine scales, a braided cord's plaits, knots along a knotted cord, and a
//! torc's twist with its shaped ends.

use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use shared::item_form::{CordStyle, Link, Strand, Thickness, TorcEnd};

use super::path::Path;
use super::shapes::{blob, tube};
use super::{NecklaceLook, Wear};
use crate::item_art::armour::{Model, Stuff, face};

/// Steps along a worn strand: its shading doesn't show on a token.
const WORN_STEPS: usize = 96;
/// The most steps along a strand in a picture.
const MOST_STEPS: usize = 360;
/// How far apart a knotted cord's knots are, and how long each is.
const KNOT_EVERY: f32 = 32.0;
const KNOT_LENGTH: f32 = 7.0;
/// How much thicker a torc is at its ends than behind the neck.
const TORC_SWELL: f32 = 0.3;

/// Half its thickness as made, a little more than life so it reads in a picture.
pub(super) fn radius(strand: &Strand) -> f32 {
    match strand {
        Strand::Cord { .. } => 2.0,
        Strand::Chain { thickness, .. } => match thickness {
            Thickness::Fine => 1.3,
            Thickness::Medium => 2.0,
            Thickness::Heavy => 3.2,
        },
        Strand::Torc { .. } => 4.5,
    }
}

/// How long one repeat of its pattern is: a link, a plait, a twist.
fn pitch(strand: &Strand) -> f32 {
    match strand {
        Strand::Cord { style, .. } => match style {
            CordStyle::Braided => 5.0,
            _ => KNOT_EVERY,
        },
        Strand::Chain { .. } => radius(strand) * 3.0,
        Strand::Torc { .. } => 8.0,
    }
}

/// Whether a face this far along and this far round its section is in shadow.
fn shaded(strand: &Strand, along: f32, round: f32) -> bool {
    let repeat = along / pitch(strand);
    let link = repeat.floor() as i64;
    let within = repeat.fract();
    let facing_out = round.cos();
    match strand {
        Strand::Chain { link: style, .. } => match style {
            // Links flat to the eye have a dark hole; those edge on, dark sides.
            Link::Cable if link % 2 == 0 => facing_out > 0.3 && (0.3..0.7).contains(&within),
            Link::Cable => facing_out.abs() < 0.5,
            Link::Rope => (repeat + round / TAU * 2.0).fract() < 0.4,
            Link::Box => within < 0.22,
            Link::Snake => (repeat * 3.0).fract() < 0.18,
        },
        Strand::Cord { style, .. } => match style {
            CordStyle::Plain => false,
            CordStyle::Braided => (repeat + round.sin().abs() * 0.5).fract() < 0.4,
            CordStyle::Knotted => (within * KNOT_EVERY - KNOT_EVERY / 2.0).abs() < 1.0,
        },
        Strand::Torc { .. } => (repeat + round / TAU * 4.0).fract() < 0.3,
    }
}

/// How much thicker than usual it is here: a knot, or a torc swelling to its ends.
fn swell(strand: &Strand, along: f32, angle: f32) -> f32 {
    match strand {
        Strand::Cord {
            style: CordStyle::Knotted,
            ..
        } => {
            let from_knot = (along.rem_euclid(KNOT_EVERY) - KNOT_EVERY / 2.0).abs();
            if from_knot < KNOT_LENGTH / 2.0 {
                1.0 + 0.8 * (1.0 - from_knot / (KNOT_LENGTH / 2.0)).sqrt()
            } else {
                1.0
            }
        }
        Strand::Torc { .. } => 1.0 + TORC_SWELL * angle.cos(),
        _ => 1.0,
    }
}

/// The strand's tube, and a torc's ends.
pub(super) fn sweep(model: &mut Model, look: &NecklaceLook, path: &Path) {
    let strand = &look.strand;
    let steps = match path.wear {
        Wear::Worn => WORN_STEPS,
        Wear::Alone => ((path.length() / pitch(strand) * 4.0) as usize).clamp(96, MOST_STEPS),
    };
    let closed = !matches!(strand, Strand::Torc { .. });
    let sides = match (path.wear, strand) {
        (Wear::Worn, _) => 4,
        (
            _,
            Strand::Chain {
                link: Link::Box, ..
            },
        ) => 4,
        _ => 8,
    };
    let angles = path.angles(strand, steps);
    let count = angles.len();
    let centres: Vec<(Vec3, Vec3)> = angles.iter().map(|&angle| path.at(angle)).collect();
    let mut along = vec![0.0];
    for index in 1..count {
        let step = centres[index].0.distance(centres[index - 1].0);
        along.push(along[index - 1] + step);
    }
    // Box links are square, flat to the skin; everything else round.
    let turn = if sides == 4 { PI / 4.0 } else { 0.0 };
    let twist = |index: usize| match (path.wear, strand) {
        (Wear::Alone, Strand::Torc { .. }) => along[index] / pitch(strand) * TAU / 4.0,
        _ => 0.0,
    };
    let section = |index: usize| -> Vec<Vec3> {
        let (centre, out) = centres[index];
        let (before, after) = match closed {
            true => ((index + count - 1) % count, (index + 1) % count),
            false => (index.saturating_sub(1), (index + 1).min(count - 1)),
        };
        let ahead = (centres[after].0 - centres[before].0).normalize_or(Vec3::X);
        let out = (out - ahead * out.dot(ahead)).normalize_or(Vec3::Z);
        let across = ahead.cross(out);
        let swell = swell(strand, along[index], angles[index]);
        (0..sides)
            .map(|side| {
                let round = side as f32 / sides as f32 * TAU + turn + twist(index);
                let lobe = match (path.wear, strand) {
                    (Wear::Alone, Strand::Torc { .. }) => 1.0 + 0.12 * (round * 4.0).cos(),
                    _ => 1.0,
                };
                let deep = match path.wear {
                    Wear::Worn => path.half_deep,
                    Wear::Alone => path.half_deep * swell * lobe,
                };
                centre
                    + out * (round.cos() * deep)
                    + across * (round.sin() * path.half_wide * swell * lobe)
            })
            .collect()
    };
    let rings: Vec<Vec<Vec3>> = (0..count).map(section).collect();
    let (light, dark) = match strand {
        Strand::Cord { .. } => (Stuff::Cloth, Stuff::Trim),
        _ => (Stuff::Main, Stuff::Shade),
    };
    let spans = if closed { count } else { count - 1 };
    for index in 0..spans {
        let next = (index + 1) % count;
        let middle = (along[index] + along[next].max(along[index])) / 2.0;
        for side in 0..sides {
            let other = (side + 1) % sides;
            let round = (side as f32 + 0.5) / sides as f32 * TAU;
            let stuff = match path.wear == Wear::Alone && shaded(strand, middle, round) {
                true => dark,
                false => light,
            };
            let out = rings[index][side] - centres[index].0;
            let (a, b) = (rings[index][side], rings[index][other]);
            let (c, d) = (rings[next][side], rings[next][other]);
            let triangles = model.of(stuff);
            face(triangles, a, b, d, out);
            face(triangles, a, d, c, out);
        }
    }
    if let Strand::Torc { ends } = strand {
        for (end, inward) in [(0, 1), (count - 1, count - 2)] {
            let (centre, out) = centres[end];
            let ahead = (centre - centres[inward].0).normalize_or(Vec3::X);
            let ring = &rings[end];
            for side in 0..sides {
                let (a, b) = (ring[side], ring[(side + 1) % sides]);
                face(model.of(Stuff::Main), centre, a, b, ahead);
            }
            torc_end(model, *ends, path, centre, ahead, out);
        }
    }
}

/// A torc's end, past the last of its bar.
fn torc_end(model: &mut Model, end: TorcEnd, path: &Path, centre: Vec3, ahead: Vec3, out: Vec3) {
    let (wide, deep) = (path.half_wide * (1.0 + TORC_SWELL), path.half_deep);
    let across = ahead.cross(out).normalize_or(Vec3::Y);
    let round = wide * 0.7;
    if path.wear == Wear::Worn {
        // Worn, any end is a small knob, flat on the skin: on a token's scale its
        // shape doesn't show, and a long end would stand off the curve of the neck.
        if end != TorcEnd::Plain {
            let axes = [ahead * wide * 0.8, out * deep, across * wide * 1.2];
            blob(model.of(Stuff::Main), centre + ahead * wide * 0.5, axes);
        }
        return;
    }
    match end {
        TorcEnd::Plain => {}
        TorcEnd::Ball => blob(
            model.of(Stuff::Main),
            centre + ahead * wide,
            [ahead * wide * 1.4, out * deep * 1.4, across * wide * 1.4],
        ),
        TorcEnd::Buffer => blob(
            model.of(Stuff::Main),
            centre + ahead * wide * 0.5,
            [ahead * wide * 0.6, out * deep * 1.3, across * wide * 1.7],
        ),
        TorcEnd::Scroll => {
            // Curling back on itself, outwards and down.
            let curl: Vec<Vec3> = (0..=12)
                .map(|step| {
                    let turn = step as f32 / 12.0 * TAU * 0.85;
                    let size = wide * (2.2 - step as f32 * 0.1);
                    centre + ahead * (turn.sin() * size) - across * ((1.0 - turn.cos()) * size)
                })
                .collect();
            tube(model.of(Stuff::Main), &curl, round, 8);
        }
        TorcEnd::Beast => {
            let snout = centre + ahead * wide * 1.6;
            let head = [ahead * wide * 1.9, out * deep * 1.1, across * wide * 1.1];
            blob(model.of(Stuff::Main), snout, head);
            for side in [-1.0, 1.0] {
                let ear = centre + ahead * wide * 0.6 + across * side * wide * 0.9;
                let ear_axes = [
                    across * side * wide * 0.6,
                    ahead * wide * 0.4,
                    out * deep * 0.9,
                ];
                blob(model.of(Stuff::Main), ear, ear_axes);
                let eye = snout + across * side * wide * 0.6 + out * deep * 0.5;
                let eye_axes = [ahead * wide * 0.25, out * deep * 0.5, across * wide * 0.25];
                blob(model.of(Stuff::Shade), eye, eye_axes);
            }
        }
    }
}
