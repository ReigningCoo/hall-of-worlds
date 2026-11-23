//! Standing barrels and sacks (tavern furniture plan PR 6), in their own
//! colours: a barrel bellied, iron-hooped, its head as high as a table's top
//! (a round board over it when topped); sacks of sackcloth or pale flour
//! cloth, tied at the neck, standing or lying in a heap. The frame's origin is
//! the middle of the cell on the ground, facing south. Sizes are first
//! guesses.

use bevy::prelude::*;
use shared::furniture::{Barrel, Sacks};

use super::super::CELL_SIZE;
use super::super::furniture::TABLE_TOP;
use super::colours::iron;
use super::model::{Model, srgb};

const C: f32 = CELL_SIZE;
/// A barrel's radius at its ends and at its belly.
const END: f32 = C * 0.25;
const BELLY: f32 = C * 0.29;

/// A barrel of wood coloured `wood`.
pub(super) fn barrel(model: &mut Model, barrel: Barrel, wood: Vec3) {
    let height = match barrel {
        Barrel::Bare => TABLE_TOP,
        Barrel::Topped => TABLE_TOP - C * 0.03,
    };
    let up = |share: f32| Vec3::Y * height * share;
    let still = Quat::IDENTITY;
    // Narrower ends and a wider belly, overlapping.
    model.cylinder(up(0.15), (END, height * 0.3), still, wood);
    model.cylinder(up(0.5), (BELLY, height * 0.5), still, wood);
    model.cylinder(up(0.85), (END, height * 0.3), still, wood);
    let band = C * 0.018;
    for (share, radius) in [(0.08, END), (0.3, BELLY), (0.7, BELLY), (0.92, END)] {
        model.hoop(up(share), (radius + band * 0.3, band), iron());
    }
    match barrel {
        Barrel::Bare => {
            let head = Vec3::new(END * 1.8, C * 0.01, END * 1.8);
            model.oval(up(1.0), head, wood * 0.8);
        }
        Barrel::Topped => {
            let top = Vec3::new(C * 0.84, C * 0.03, C * 0.84);
            model.oval(up(1.0) + Vec3::Y * C * 0.015, top, wood);
        }
    }
}

/// Sackcloth or flour cloth, by `variant`.
fn cloth(variant: u8) -> Vec3 {
    match variant % 3 {
        0 => srgb(0.7, 0.58, 0.4),
        1 => srgb(0.62, 0.52, 0.36),
        _ => srgb(0.9, 0.87, 0.8),
    }
}

/// A sack standing with its middle `at` (on the ground), tied at the top.
fn standing(model: &mut Model, at: Vec3, colour: Vec3) {
    let body = Vec3::new(0.3, 0.42, 0.26) * C;
    model.ball(at + Vec3::Y * body.y / 2.0, body, colour);
    let neck = at + Vec3::Y * (body.y - C * 0.01);
    model.cylinder(neck, (C * 0.045, C * 0.07), Quat::IDENTITY, colour);
    model.ball(
        neck + Vec3::Y * C * 0.05,
        Vec3::new(0.09, 0.05, 0.09) * C,
        colour,
    );
}

/// A sack lying with its middle `at` (its underside), along `x` if
/// `east_west`, else along `z`.
fn lying(model: &mut Model, at: Vec3, east_west: bool, colour: Vec3) {
    let (long, wide) = (C * 0.46, C * 0.28);
    let size = if east_west {
        Vec3::new(long, C * 0.2, wide)
    } else {
        Vec3::new(wide, C * 0.2, long)
    };
    model.ball(at + Vec3::Y * size.y / 2.0, size, colour);
}

/// Sacks, coloured by `variant`.
pub(super) fn sacks(model: &mut Model, sacks: Sacks, variant: u8) {
    let colour = cloth(variant);
    let other = cloth(variant / 3 + 1);
    match sacks {
        Sacks::One => standing(model, Vec3::ZERO, colour),
        Sacks::Two => {
            standing(model, Vec3::new(-C * 0.14, 0.0, C * 0.02), colour);
            standing(model, Vec3::new(C * 0.14, 0.0, -C * 0.03), other);
        }
        Sacks::Heap => {
            lying(model, Vec3::new(0.0, 0.0, -C * 0.15), true, colour);
            lying(model, Vec3::new(0.0, 0.0, C * 0.15), true, other);
            lying(model, Vec3::Y * C * 0.17, false, colour);
        }
    }
}
