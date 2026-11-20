//! A serving cut from a big fungus (a giant puffball, a hen of the woods): a
//! half-round slab showing its flesh, the skin round its curved edge.

use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

use super::lathe::{Round, angle, lathe};
use super::model::{Build, FungusPiece};
use super::paint::speckle;

/// How far round the slab goes, either side of its middle.
const HALF: f32 = 1.5;

pub(super) fn slice(build: &Build, n: usize) -> Vec<FungusPiece> {
    let c = build.colours;
    let thick = 0.16;
    let profile = [
        Vec2::new(0.0, 0.0),
        Vec2::new(0.47, 0.0),
        Vec2::new(0.5, 0.03),
        Vec2::new(0.5, thick - 0.03),
        Vec2::new(0.47, thick),
        Vec2::new(0.0, thick),
    ];
    let mesh = lathe(&profile, false, |row, i| {
        // Off the middle way (+z), how far round: the slab keeps what is near.
        let off = (angle(i) - FRAC_PI_2 + PI).rem_euclid(2.0 * PI) - PI;
        let inside_wedge = off.abs() <= HALF;
        let skin = (2..=3).contains(&row);
        let colour = match skin && inside_wedge {
            true => c.body,
            false => c.inside * (0.95 + 0.06 * speckle(row, i, n as u32)),
        };
        Round {
            out: if inside_wedge { 1.0 } else { 0.04 },
            colour,
        }
    });
    vec![FungusPiece::new(mesh)]
}
