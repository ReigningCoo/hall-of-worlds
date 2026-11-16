//! Roofs where a floor is open air over the storey under it (a building
//! narrowing as it goes up): looking down over the air shows the top of the
//! storey below, not its rooms (only holes show those). A floor below is
//! roofed at its walls' tops; a floor above roofs the storey under it at its
//! own foot, so the wedge towards the camera leaves that out too.

use bevy::prelude::*;
use shared::dungeon::{Cell, Level, Sight};
use shared::floor_sight::is_out;

use super::super::mesh::Builder;
use super::super::{CELL_SIZE, WALL_HEIGHT};

/// The cells of a floor below (`level`, its cells known `known`) that get a
/// roof: known, under `Air` on `over` (the floor directly above), and neither
/// outside, a hole, nor a wall or opening drawn (`drawn`), whose own top is
/// there already.
pub(super) fn roofed(
    (level, known): (&Level, &Sight),
    over: &Level,
    drawn: &Sight,
) -> Vec<(i64, i64)> {
    let (min_x, min_y, width, height) = level.bounds();
    let mut cells = Vec::new();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            if !known.is_seen(x, y) || !over.contains(x, y) || over.cell(x, y) != Cell::Air {
                continue;
            }
            let cell = level.cell(x, y);
            let face = cell.is_wall() || cell.is_opening();
            if is_out(cell) || cell == Cell::Hole || face && drawn.is_seen(x, y) {
                continue;
            }
            cells.push((x, y));
        }
    }
    cells
}

/// The cells of a floor above (`level`) that roof the storey under it
/// (`under`, its cells known `known`): `Air` over anything known there but
/// outside or a hole. Cells the client doesn't know of `level` read as walls,
/// so get none.
pub(super) fn roofing(level: &Level, (under, known): (&Level, &Sight)) -> Vec<(i64, i64)> {
    let (min_x, min_y, width, height) = level.bounds();
    let mut cells = Vec::new();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            if level.cell(x, y) != Cell::Air || !under.contains(x, y) || !known.is_seen(x, y) {
                continue;
            }
            let cell = under.cell(x, y);
            if !is_out(cell) && cell != Cell::Hole {
                cells.push((x, y));
            }
        }
    }
    cells
}

/// Which floor's own height a roof is at: the top of a floor below's walls,
/// or the foot of a floor above.
pub(super) enum RoofAt {
    Top,
    Foot,
}

/// A flat roof over `cells`, lit `light`.
pub(super) fn roof_mesh(cells: &[(i64, i64)], at: RoofAt, light: Vec3) -> Mesh {
    let mut builder = Builder::default();
    let half = CELL_SIZE / 2.0;
    let height = match at {
        RoofAt::Top => WALL_HEIGHT,
        RoofAt::Foot => 0.0,
    };
    for &(x, y) in cells {
        let centre = Vec3::new(x as f32 * CELL_SIZE, height, y as f32 * CELL_SIZE);
        builder.quad(centre, Vec3::Y, Vec3::NEG_Z, (half, half), |_| light);
    }
    builder.into_mesh()
}

#[cfg(test)]
mod tests;
