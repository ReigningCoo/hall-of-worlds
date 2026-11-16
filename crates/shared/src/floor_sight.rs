//! What a viewer on one floor of a building sees of its other floors (owner's
//! design, 2026-11-16): the outside in vertical slices, and straight down
//! through holes. Both sides work it out the same way: the server to send and
//! remember those cells, the client to draw them.
//!
//! - An outside cell (ground, or `Cell::Air` beside an upper floor) seen from
//!   where the viewer is shows its whole column, on every floor, together with
//!   the walls (and their windows and doors) facing into that column.
//! - An outside wall face seen (a wall beside a seen outside cell, with the
//!   viewer out on that side of it) shows that face on every floor, full
//!   height: on a floor set back, the face behind the air in front of it.
//! - A face is only seen from its outside side, so the wall under the window
//!   the viewer looks out of is never seen: they are behind it.
//! - Rooms inside other floors are never seen, except straight down through a
//!   ruin's holes; but in a seen column, a floor's cell under `Air` on the
//!   floor directly above is seen, as its roof is (a building narrowing as it
//!   goes up).
//! - The ground floor's outside is only seen within `GROUND_REACH` of the
//!   viewer.
//!
//! Only a building's floors above ground (`Site::is_building`): never caves,
//! and never a basement.

use crate::dungeon::{Cell, Level, Sight};

/// How far from the viewer (in cells, straight line) the ground floor's
/// outside is seen from other floors.
pub const GROUND_REACH: i64 = 24;

/// The four ways a wall can face.
const WAYS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

/// What is seen of each of a building's floors (`storeys`, from the ground up,
/// its basement left out) by a viewer at cell `at` on floor `viewer`, who sees
/// `seen` there. One `Sight` per floor, the viewer's own left empty.
pub fn other_floors(storeys: &[&Level], viewer: usize, at: (i64, i64), seen: &Sight) -> Vec<Sight> {
    let mut found: Vec<Sight> = storeys.iter().map(|level| Sight::new(level)).collect();
    let Some(own) = storeys.get(viewer) else {
        return found;
    };
    let (columns, faces, holes) = looked_at(own, at, seen);
    for (index, level) in storeys.iter().enumerate() {
        if index == viewer {
            continue;
        }
        let sight = &mut found[index];
        let near = |(x, y): (i64, i64)| {
            index != 0 || (x - at.0).pow(2) + (y - at.1).pow(2) <= GROUND_REACH.pow(2)
        };
        let over = storeys.get(index + 1).copied();
        for &column in &columns {
            show_column((level, over), sight, column, at, &near);
        }
        for &(wall, way) in &faces {
            show_face(level, sight, wall, way, &near);
        }
    }
    for (x, y) in holes {
        for index in (0..viewer).rev() {
            let level = storeys[index];
            if !level.contains(x, y) {
                break;
            }
            found[index].insert(x, y);
            if level.cell(x, y) != Cell::Hole {
                break;
            }
        }
    }
    found
}

/// Whether a cell is outside: ground and the like, or air.
pub fn is_out(cell: Cell) -> bool {
    cell == Cell::Air || cell.is_outside()
}

/// Whether a cell is part of a wall's face: wall, window or door.
fn is_face(cell: Cell) -> bool {
    cell.is_wall() || cell.is_opening()
}

/// Whether a viewer at `at` is out beyond the face of `wall` that looks `way`.
fn beyond(at: (i64, i64), wall: (i64, i64), way: (i64, i64)) -> bool {
    (at.0 - wall.0) * way.0 + (at.1 - wall.1) * way.1 >= 1
}

/// Outside cells seen on the viewer's floor, wall faces seen from outside
/// (with the way each looks), and holes seen.
#[allow(clippy::type_complexity)]
fn looked_at(
    own: &Level,
    at: (i64, i64),
    seen: &Sight,
) -> (
    Vec<(i64, i64)>,
    Vec<((i64, i64), (i64, i64))>,
    Vec<(i64, i64)>,
) {
    let (mut columns, mut faces, mut holes) = (Vec::new(), Vec::new(), Vec::new());
    let (min_x, min_y, width, height) = own.bounds();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            if !seen.is_seen(x, y) {
                continue;
            }
            let cell = own.cell(x, y);
            if is_out(cell) {
                columns.push((x, y));
            } else if cell == Cell::Hole {
                holes.push((x, y));
            } else if is_face(cell) {
                for way in WAYS {
                    let out = (x + way.0, y + way.1);
                    let looks_out = own.contains(out.0, out.1) && is_out(own.cell(out.0, out.1));
                    if looks_out && seen.is_seen(out.0, out.1) && beyond(at, (x, y), way) {
                        faces.push(((x, y), way));
                    }
                }
            }
        }
    }
    (columns, faces, holes)
}

/// Shows the column at `(x, y)` on one floor (`level`, with `over` the floor
/// directly above it): outside there, it and the walls facing into it; a wall
/// there (a lower floor reaching further out), that; inside under air, that
/// (its roof is seen).
fn show_column(
    (level, over): (&Level, Option<&Level>),
    sight: &mut Sight,
    (x, y): (i64, i64),
    at: (i64, i64),
    near: &impl Fn((i64, i64)) -> bool,
) {
    if !level.contains(x, y) {
        return;
    }
    let cell = level.cell(x, y);
    let under_air = over.is_some_and(|over| over.contains(x, y) && over.cell(x, y) == Cell::Air);
    if is_face(cell) || !is_out(cell) && under_air && near((x, y)) {
        sight.insert(x, y);
    }
    if !is_out(cell) || !near((x, y)) {
        return;
    }
    sight.insert(x, y);
    for way in WAYS {
        let wall = (x - way.0, y - way.1);
        if level.contains(wall.0, wall.1)
            && is_face(level.cell(wall.0, wall.1))
            && beyond(at, wall, way)
        {
            sight.insert(wall.0, wall.1);
        }
    }
}

/// Shows a face seen on the viewer's floor on another: the wall in its place,
/// or, where that floor is set back, the air in front and the face behind it.
fn show_face(
    level: &Level,
    sight: &mut Sight,
    wall: (i64, i64),
    way: (i64, i64),
    near: &impl Fn((i64, i64)) -> bool,
) {
    let (_, _, width, height) = level.bounds();
    let mut at = wall;
    for _ in 0..width.max(height) {
        if !level.contains(at.0, at.1) {
            return;
        }
        let cell = level.cell(at.0, at.1);
        if is_face(cell) {
            sight.insert(at.0, at.1);
            return;
        }
        if !is_out(cell) {
            return;
        }
        if near(at) {
            sight.insert(at.0, at.1);
        }
        at = (at.0 - way.0, at.1 - way.1);
    }
}

#[cfg(test)]
mod tests;
