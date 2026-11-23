//! Beds, chests and wardrobes (tavern furniture plan PR 7), drawn with decor
//! so bedding has its own colours. The frame's origin is the middle of the
//! piece on the ground, facing south (`+z`): a bed's foot, a chest's or
//! wardrobe's front; `-z` is the wall behind. A bed is smaller than its cells
//! (owner's), leaving room to walk round it; chests and wardrobes are shown
//! shut. Sizes are first guesses.

use bevy::prelude::*;
use shared::chest::{Chest, Wardrobe};
use shared::furniture::Bed;

use super::super::CELL_SIZE;
use super::colours::{iron, painted};
use super::model::{Model, srgb};

const C: f32 = CELL_SIZE;
/// How far a piece stands off the wall behind it.
const OFF_WALL: f32 = C * 0.06;
/// How tall a wardrobe is.
pub(in crate::dungeon_map) const WARDROBE_HEIGHT: f32 = C * 0.95;
/// How tall a chest is, lid and all.
pub(in crate::dungeon_map) const CHEST_HEIGHT: f32 = C * 0.44;

fn linen() -> Vec3 {
    srgb(0.9, 0.88, 0.82)
}

/// A bed of wood coloured `wood`, its blanket in a colour by `variant`.
pub(super) fn bed(model: &mut Model, bed: Bed, wood: Vec3, variant: u8) {
    let half_width = match bed {
        Bed::Single => C * 0.36,
        Bed::Double => C * 0.8,
    };
    let head = -C + OFF_WALL;
    let foot = C * 0.8;
    let (left, right) = (-half_width, half_width);
    let leg = C * 0.06;
    // Legs, rails and the boards at each end.
    for z in [head, foot - leg] {
        for x in [left, right - leg] {
            model.block(
                Vec3::new(x, 0.0, z),
                Vec3::new(x + leg, C * 0.22, z + leg),
                wood,
            );
        }
    }
    model.block(
        Vec3::new(left, C * 0.12, head),
        Vec3::new(right, C * 0.22, foot),
        wood,
    );
    let board = C * 0.05;
    model.block(
        Vec3::new(left, C * 0.12, head),
        Vec3::new(right, C * 0.62, head + board),
        wood,
    );
    model.block(
        Vec3::new(left, C * 0.12, foot - board),
        Vec3::new(right, C * 0.34, foot),
        wood,
    );
    // The mattress, pillows by the head and a blanket over the rest.
    let inset = C * 0.03;
    let (mattress_low, mattress_high) = (C * 0.22, C * 0.31);
    model.block(
        Vec3::new(left + inset, mattress_low, head + board),
        Vec3::new(right - inset, mattress_high, foot - board),
        linen(),
    );
    let pillows = match bed {
        Bed::Single => vec![0.0],
        Bed::Double => vec![-half_width / 2.0, half_width / 2.0],
    };
    for x in pillows {
        let at = Vec3::new(x, mattress_high + C * 0.04, head + board + C * 0.14);
        model.ball(at, Vec3::new(C * 0.5, C * 0.09, C * 0.2), linen());
    }
    let (blanket, edge) = painted(variant);
    let cover = head + board + C * 0.32;
    model.block(
        Vec3::new(left + inset * 0.5, mattress_high - C * 0.02, cover),
        Vec3::new(right - inset * 0.5, mattress_high + C * 0.03, foot - board),
        blanket,
    );
    model.block(
        Vec3::new(left + inset * 0.5, mattress_high + C * 0.03, cover),
        Vec3::new(
            right - inset * 0.5,
            mattress_high + C * 0.04,
            cover + C * 0.1,
        ),
        edge,
    );
}

/// A chest of wood coloured `wood`, its lid shut, bound in iron.
pub(super) fn chest(model: &mut Model, chest: Chest, wood: Vec3) {
    let half = (f32::from(chest.length()) * C - C * 0.2) / 2.0;
    let back = -C / 2.0 + OFF_WALL;
    let front = back + C * 0.55;
    let lid = CHEST_HEIGHT - C * 0.08;
    model.block(
        Vec3::new(-half, 0.0, back),
        Vec3::new(half, lid, front),
        wood,
    );
    let over = C * 0.015;
    model.block(
        Vec3::new(-half - over, lid, back - over),
        Vec3::new(half + over, CHEST_HEIGHT, front + over),
        wood * 0.85,
    );
    // Straps over the lid and down the front, and a lock in the middle.
    let strap = C * 0.04;
    let out = C * 0.01;
    for x in [-half * 0.6, half * 0.6] {
        model.block(
            Vec3::new(x - strap / 2.0, CHEST_HEIGHT, back),
            Vec3::new(x + strap / 2.0, CHEST_HEIGHT + out, front + over + out),
            iron(),
        );
        model.block(
            Vec3::new(x - strap / 2.0, 0.0, front + over),
            Vec3::new(x + strap / 2.0, CHEST_HEIGHT, front + over + out),
            iron(),
        );
    }
    model.block(
        Vec3::new(-C * 0.05, lid - C * 0.1, front),
        Vec3::new(C * 0.05, lid + C * 0.02, front + over + out * 1.5),
        iron(),
    );
}

/// A wardrobe of wood coloured `wood`, its doors shut.
pub(super) fn wardrobe(model: &mut Model, wardrobe: Wardrobe, wood: Vec3) {
    let half = (f32::from(wardrobe.length()) * C - C * 0.12) / 2.0;
    let back = -C / 2.0 + OFF_WALL;
    let front = back + C * 0.6;
    let plinth = C * 0.06;
    model.block(
        Vec3::new(-half + C * 0.02, 0.0, back),
        Vec3::new(half - C * 0.02, plinth, front - C * 0.02),
        wood * 0.8,
    );
    model.block(
        Vec3::new(-half, plinth, back),
        Vec3::new(half, WARDROBE_HEIGHT - C * 0.05, front),
        wood,
    );
    let cornice = C * 0.03;
    model.block(
        Vec3::new(-half - cornice, WARDROBE_HEIGHT - C * 0.05, back),
        Vec3::new(half + cornice, WARDROBE_HEIGHT, front + cornice),
        wood * 0.85,
    );
    // A door a cell, panelled, with a knob by the middle.
    let doors = wardrobe.length();
    let door = 2.0 * half / f32::from(doors);
    let gap = C * 0.01;
    for i in 0..doors {
        let left = -half + door * f32::from(i) + gap;
        let right = left + door - 2.0 * gap;
        let (low, high) = (plinth + C * 0.08, WARDROBE_HEIGHT - C * 0.14);
        model.block(
            Vec3::new(left + C * 0.06, low, front),
            Vec3::new(right - C * 0.06, high, front + C * 0.015),
            wood * 0.9,
        );
        let knob_x = if i % 2 == 0 && doors > 1 {
            right - C * 0.05
        } else {
            left + C * 0.05
        };
        let knob = Vec3::new(knob_x, C * 0.5, front + C * 0.02);
        model.ball(knob, Vec3::splat(C * 0.035), iron());
    }
}
