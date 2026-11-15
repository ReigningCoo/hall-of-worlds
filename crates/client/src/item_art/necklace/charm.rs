//! Charms: small flat shapes of metal hung below the bail, each from plates and bands
//! in the plane it lies in, details picked out in shade.

use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use shared::item_form::Charm;

use super::ornament::Heights;
use super::shapes::{Place, band, ellipse, slab};
use crate::item_art::armour::{Model, Stuff};

/// About how tall and how wide a charm is.
const TALL: f32 = 16.0;
const WIDE: f32 = 12.0;

/// The charm, its top at `top` of the plane.
pub(super) fn charm(model: &mut Model, place: &Place, heights: &Heights, shape: Charm, top: Vec2) {
    let plate = heights.plate;
    let raised = (plate.1, plate.1 + heights.rise);
    let middle = top + Vec2::Y * TALL / 2.0;
    let half = Vec2::new(WIDE, TALL) / 2.0;
    let metal =
        |model: &mut Model, outline: &[Vec2]| slab(model.of(Stuff::Main), place, outline, plate);
    let line = |model: &mut Model, points: &[Vec2], thick: f32, closed: bool| {
        band(
            model.of(Stuff::Shade),
            place,
            points,
            |_| thick,
            raised,
            closed,
        )
    };
    match shape {
        Charm::Disc => {
            let centre = top + Vec2::Y * WIDE / 2.0;
            metal(model, &ellipse(centre, Vec2::splat(WIDE / 2.0), 24));
            line(
                model,
                &ellipse(centre, Vec2::splat(WIDE * 0.38), 24),
                0.5,
                true,
            );
        }
        Charm::Teardrop => {
            let outline: Vec<Vec2> = (0..24)
                .map(|step| {
                    let angle = step as f32 / 24.0 * TAU;
                    // Pointed at the top, round below.
                    let width = (angle / 2.0).sin().powf(1.5);
                    middle + Vec2::new(angle.sin() * half.x * width, -angle.cos() * half.y)
                })
                .collect();
            metal(model, &outline);
        }
        Charm::Leaf => {
            let outline: Vec<Vec2> = (0..24)
                .map(|step| {
                    let angle = step as f32 / 24.0 * TAU;
                    let width = angle.sin() * angle.sin().abs().sqrt();
                    middle + Vec2::new(width * half.x * 0.8, -angle.cos() * half.y)
                })
                .collect();
            metal(model, &outline);
            let rib = [top + Vec2::Y * 1.0, top + Vec2::Y * (TALL - 1.5)];
            line(model, &rib, 0.4, false);
        }
        Charm::Crescent => {
            // Horns up: a band along an arc, thinning to its tips.
            let arc: Vec<Vec2> = (0..=16)
                .map(|step| {
                    let angle = -PI * 0.8 + step as f32 / 16.0 * PI * 1.6;
                    middle + Vec2::new(angle.sin(), angle.cos()) * half.x
                })
                .collect();
            let thick = |index: usize| 0.4 + 2.6 * (index as f32 / 16.0 * PI).sin();
            band(model.of(Stuff::Main), place, &arc, thick, plate, false);
        }
        Charm::Acorn => {
            let nut = middle + Vec2::Y * 2.0;
            metal(
                model,
                &ellipse(nut, Vec2::new(half.x * 0.7, half.y * 0.75), 20),
            );
            let cup = ellipse(top + Vec2::Y * 4.0, Vec2::new(half.x * 0.85, 3.5), 16);
            slab(model.of(Stuff::Shade), place, &cup, raised);
        }
        Charm::Key => {
            let bow = top + Vec2::Y * 3.5;
            band(
                model.of(Stuff::Main),
                place,
                &ellipse(bow, Vec2::splat(3.0), 12),
                |_| 1.0,
                plate,
                true,
            );
            let shaft = [bow + Vec2::Y * 3.0, top + Vec2::Y * (TALL + 2.0)];
            band(model.of(Stuff::Main), place, &shaft, |_| 0.9, plate, false);
            let bit = [
                top + Vec2::new(0.0, TALL - 1.0),
                top + Vec2::new(3.5, TALL - 1.0),
            ];
            band(model.of(Stuff::Main), place, &bit, |_| 1.4, plate, false);
        }
        Charm::Tooth => {
            // Curved, wide at the root, to a point.
            let curve: Vec<Vec2> = (0..=10)
                .map(|step| {
                    let share = step as f32 / 10.0;
                    top + Vec2::new(share * share * 4.0, share * TALL)
                })
                .collect();
            let thick = |index: usize| 3.0 * (1.0 - index as f32 / 10.0) + 0.2;
            band(model.of(Stuff::Main), place, &curve, thick, plate, false);
        }
        Charm::Knot => {
            // Two loops side by side, the cord passing through.
            for side in [-1.0, 1.0] {
                let loop_middle = middle + Vec2::new(side * half.x * 0.42, 0.0);
                let ring = ellipse(loop_middle, Vec2::new(half.x * 0.45, half.y * 0.4), 16);
                band(model.of(Stuff::Main), place, &ring, |_| 1.0, plate, true);
            }
        }
        Charm::Shield => {
            let outline = shield(middle, half);
            metal(model, &outline);
            let inner: Vec<Vec2> = shield(middle, half * 0.78);
            line(model, &inner, 0.5, true);
        }
    }
}

/// A heater shield's outline: flat across the top, curving to a point below.
fn shield(middle: Vec2, half: Vec2) -> Vec<Vec2> {
    let mut outline = vec![
        middle + Vec2::new(-half.x, -half.y),
        middle + Vec2::new(half.x, -half.y),
    ];
    for step in 0..=8 {
        let share = step as f32 / 8.0;
        let x = half.x * (1.0 - share * share);
        outline.push(middle + Vec2::new(x, -half.y * 0.2 + share * half.y * 1.2));
    }
    for step in (0..8).rev() {
        let share = step as f32 / 8.0;
        let x = half.x * (1.0 - share * share);
        outline.push(middle + Vec2::new(-x, -half.y * 0.2 + share * half.y * 1.2));
    }
    outline
}
