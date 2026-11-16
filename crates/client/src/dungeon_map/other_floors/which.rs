//! Which floors below are drawn, and which of their cells: what is seen of them
//! now (`shared::floor_sight`, worked out here as the server does) and what is
//! remembered of their slices.

use shared::dungeon::{Cell, Level, Sight};
use shared::floor_sight::{self, is_out};
use shared::protocol::LevelInfo;

/// The floors below the player's in a building, as `(level index, storeys
/// down)`, nearest first: none in a cave, a town, a basement or on the ground
/// floor.
pub(super) fn floors_below(info: &LevelInfo) -> Vec<(u32, u32)> {
    if !info.building || info.storey(info.index) < 1 {
        return Vec::new();
    }
    (0..info.index)
        .rev()
        .map(|index| (index, info.index - index))
        .collect()
}

/// The floors above the player's in a building, as `(level index, storeys
/// up)`, nearest first: none in a cave, a town or a basement.
pub(super) fn floors_above(info: &LevelInfo) -> Vec<(u32, u32)> {
    if !info.building || info.storey(info.index) < 0 {
        return Vec::new();
    }
    let storeys = info.levels - u32::from(info.basement);
    (info.index + 1..storeys)
        .map(|index| (index, index - info.index))
        .collect()
}

/// Whether the floors below are drawn under the player's (an upper floor of a
/// building), so holes in it show them.
pub(in crate::dungeon_map) fn has_floors_below(info: Option<&LevelInfo>) -> bool {
    info.is_some_and(|info| !floors_below(info).is_empty())
}

/// What the player sees now of each floor of the building (`storeys`, from the
/// ground up, theirs at `own`), standing at `at` and seeing `seen` on theirs.
/// Floors the client knows nothing of are walls.
pub(super) fn seen_now(
    storeys: &[Option<&Level>],
    own: &Level,
    viewer: usize,
    at: (i64, i64),
    seen: &Sight,
) -> Vec<Sight> {
    let (min_x, min_y, width, height) = own.bounds();
    let unknown = Level::new(min_x, min_y, width, height);
    let levels: Vec<&Level> = storeys
        .iter()
        .enumerate()
        .map(|(index, level)| match level {
            _ if index == viewer => own,
            Some(level) => level,
            None => &unknown,
        })
        .collect();
    floor_sight::other_floors(&levels, viewer, at, seen)
}

/// What is drawn of a floor below: everything seen now, and of what is
/// remembered (if `remembered`), the slices (outside cells and the walls facing
/// them) and anything seen straight down from the player's floor (through
/// `above`'s holes and air). On the ground floor, nothing further than `range`
/// cells from `at`.
pub(super) fn drawn(
    level: &Level,
    known: &Sight,
    now: &Sight,
    above: &[&Level],
    (at, range): ((i64, i64), Option<f32>),
    remembered: bool,
) -> Sight {
    let mut drawn = Sight::new(level);
    let (min_x, min_y, width, height) = level.bounds();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            let near = range.is_none_or(|range| {
                let (dx, dy) = ((x - at.0) as f32, (y - at.1) as f32);
                dx * dx + dy * dy <= range * range
            });
            let shown = now.is_seen(x, y)
                || remembered
                    && known.is_seen(x, y)
                    && (in_slice(level, x, y) || open_above(above, x, y));
            if near && shown {
                drawn.insert(x, y);
            }
        }
    }
    drawn
}

/// Whether a cell is part of the outside's slices: outside, or a wall, window
/// or door facing outside.
fn in_slice(level: &Level, x: i64, y: i64) -> bool {
    let cell = level.cell(x, y);
    if is_out(cell) {
        return true;
    }
    let face = cell.is_wall() || cell.is_opening();
    face && [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().any(|(dx, dy)| {
        let (nx, ny) = (x + dx, y + dy);
        level.contains(nx, ny) && is_out(level.cell(nx, ny))
    })
}

/// Whether every floor above, up to the player's, is open at a cell (a hole,
/// or air), so it can be seen straight down.
fn open_above(above: &[&Level], x: i64, y: i64) -> bool {
    !above.is_empty()
        && above
            .iter()
            .all(|level| level.contains(x, y) && matches!(level.cell(x, y), Cell::Hole | Cell::Air))
}

/// How much each storey down is darkened (the colour kept, 1 none): `day` in
/// full daylight, `night` with no light outside, by `daylight` between.
pub(super) fn kept_per_storey(day: f32, night: f32, daylight: f32) -> f32 {
    let share = daylight.clamp(0.0, 1.0);
    1.0 - (night + (day - night) * share).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests;
