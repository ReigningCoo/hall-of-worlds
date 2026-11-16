//! Wooden tables: on trestles (A-frames or boards), joined (four legs with
//! aprons and low stretchers), on plank legs, or on one pedestal.

use bevy::prelude::*;

use super::super::CELL_SIZE;
use super::tables::Table;
use super::{Grain, INSET, Part};

/// How thick legs are.
const LEG: f32 = CELL_SIZE * 0.065;

fn block(low: (f32, f32, f32), high: (f32, f32, f32), grain: Grain) -> Part {
    Part::block(Vec3::from(low), Vec3::from(high), grain)
}

/// A rail along the middle of the length from `from` to `to`, at `height`.
fn middle_stretcher(table: &Table, (from, to): (f32, f32), height: f32) -> Part {
    let half = CELL_SIZE * 0.03;
    let middle = table.width / 2.0;
    block(
        (from, height, middle - half),
        (to, height + half * 2.0, middle + half),
        Grain::Along,
    )
}

/// Boards on two trestles, A-frames or upright boards on feet (by a hash),
/// joined by a stretcher.
pub(super) fn trestle(table: &Table) -> Vec<Part> {
    let mut parts = table.board_top();
    let under = table.under;
    let (near, far) = (
        INSET + 0.05 * CELL_SIZE,
        table.width - INSET - 0.05 * CELL_SIZE,
    );
    let ends = [table.length * 0.22, table.length * 0.78];
    let a_frames = table.hash(100).is_multiple_of(2);
    for (i, &at) in ends.iter().enumerate() {
        if a_frames {
            let bar = CELL_SIZE * 0.07;
            let beam = block(
                (at - bar / 2.0, under - bar, near),
                (at + bar / 2.0, under, far),
                Grain::Across,
            );
            parts.push(table.vary(beam, 10 + i as i64));
            // Two legs at each end of the beam, splayed along the length.
            let splay = CELL_SIZE * 0.12;
            for z in [near + LEG * 0.5, far - LEG * 1.5] {
                for sign in [-1.0, 1.0] {
                    let foot = at + sign * splay - LEG / 2.0;
                    let leg = block(
                        (foot, 0.0, z),
                        (foot + LEG, under - bar, z + LEG),
                        Grain::Up,
                    );
                    parts.push(leg.leaning(Vec2::new(-sign * splay, 0.0)));
                }
            }
        } else {
            let half = CELL_SIZE * 0.025;
            let foot = CELL_SIZE * 0.06;
            let board = block((at - half, foot, near), (at + half, under, far), Grain::Up);
            parts.push(table.vary(board, 10 + i as i64));
            let wide = CELL_SIZE * 0.09;
            parts.push(block(
                (at - wide, 0.0, near - 0.02 * CELL_SIZE),
                (at + wide, foot, far + 0.02 * CELL_SIZE),
                Grain::Across,
            ));
        }
    }
    parts.push(middle_stretcher(table, (ends[0], ends[1]), under * 0.4));
    parts
}

/// Four legs at the corners, aprons under the top and low stretchers along
/// both sides.
pub(super) fn joined(table: &Table) -> Vec<Part> {
    let mut parts = table.board_top();
    let under = table.under;
    let (west, east) = (
        INSET + 0.02 * CELL_SIZE,
        table.length - INSET - 0.02 * CELL_SIZE - LEG,
    );
    let (north, south) = (
        INSET + 0.02 * CELL_SIZE,
        table.width - INSET - 0.02 * CELL_SIZE - LEG,
    );
    for x in [west, east] {
        for z in [north, south] {
            parts.push(block((x, 0.0, z), (x + LEG, under, z + LEG), Grain::Up));
        }
    }
    let (apron, thin) = (CELL_SIZE * 0.08, CELL_SIZE * 0.025);
    let rail = CELL_SIZE * 0.035;
    for z in [north, south] {
        let middle = z + LEG / 2.0;
        let side = block(
            (west + LEG, under - apron, middle - thin / 2.0),
            (east, under, middle + thin / 2.0),
            Grain::Along,
        );
        parts.push(table.vary(side, 20 + z as i64));
        let low = CELL_SIZE * 0.06;
        parts.push(block(
            (west + LEG, low, middle - rail / 2.0),
            (east, low + rail, middle + rail / 2.0),
            Grain::Along,
        ));
    }
    for x in [west, east] {
        let middle = x + LEG / 2.0;
        let end = block(
            (middle - thin / 2.0, under - apron, north + LEG),
            (middle + thin / 2.0, under, south),
            Grain::Across,
        );
        parts.push(table.vary(end, 30 + x as i64));
    }
    parts
}

/// A top on an upright plank at each end, joined by a rail under the top.
pub(super) fn plank_legs(table: &Table) -> Vec<Part> {
    let mut parts = table.board_top();
    let under = table.under;
    let thick = CELL_SIZE * 0.045;
    let (near, far) = (
        INSET + 0.03 * CELL_SIZE,
        table.width - INSET - 0.03 * CELL_SIZE,
    );
    let ends = [
        INSET + 0.05 * CELL_SIZE,
        table.length - INSET - 0.05 * CELL_SIZE - thick,
    ];
    for (i, &x) in ends.iter().enumerate() {
        let plank = block((x, 0.0, near), (x + thick, under, far), Grain::Up);
        parts.push(table.vary(plank, 40 + i as i64));
    }
    let rail = CELL_SIZE * 0.07;
    parts.push(middle_stretcher(
        table,
        (ends[0] + thick, ends[1]),
        under - rail - 0.02 * CELL_SIZE,
    ));
    parts
}

/// A top on one column, which stands on crossed feet and carries crossed
/// bearers under the top.
pub(super) fn pedestal(table: &Table) -> Vec<Part> {
    let mut parts = table.board_top();
    let under = table.under;
    let (middle_x, middle_z) = (table.length / 2.0, table.width / 2.0);
    let column = CELL_SIZE * 0.07;
    let (foot, bearer, half) = (CELL_SIZE * 0.07, CELL_SIZE * 0.05, CELL_SIZE * 0.04);
    parts.push(block(
        (middle_x - column, foot, middle_z - column),
        (middle_x + column, under - bearer, middle_z + column),
        Grain::Up,
    ));
    let reach_x = middle_x - INSET - 0.04 * CELL_SIZE;
    let reach_z = middle_z - INSET - 0.04 * CELL_SIZE;
    for (low, high) in [(0.0, foot), (under - bearer, under)] {
        parts.push(block(
            (middle_x - reach_x, low, middle_z - half),
            (middle_x + reach_x, high, middle_z + half),
            Grain::Along,
        ));
        parts.push(block(
            (middle_x - half, low, middle_z - reach_z),
            (middle_x + half, high, middle_z + reach_z),
            Grain::Across,
        ));
    }
    parts
}
