//! The vegetable showcase (debug API `client.vegetable_showcase`): a cell of
//! every vegetable, pulse and grain (of one plant shape, if asked) at one
//! stage, in rows south of the player (in front of them as the camera
//! starts), in place of the plants there, to look at them all.

use shared::vegetable::{PlantShape, Vegetables};

/// Cells in a row: as many as the player's field of view takes in from
/// `FIRST_ROW` away.
const COLUMNS: usize = 8;

/// How far south of the player the first row is.
const FIRST_ROW: i64 = 4;

/// Cells from one to the next.
const SPACING: i64 = 2;

/// A cell of every vegetable of `shape`, or all, at `stage`: in `COLUMNS` a
/// row, in the table's order, from the north-west, the first row
/// `FIRST_ROW` south of `(x, y)`.
pub(super) fn cells(
    (x, y): (i64, i64),
    (stage, shape): (u8, Option<PlantShape>),
) -> Vec<((i64, i64), &'static str, u8)> {
    let plants: Vec<_> = (Vegetables::built_in().iter())
        .filter(|def| shape.is_none_or(|shape| def.plant == shape))
        .collect();
    let wide = plants.len().clamp(1, COLUMNS) as i64;
    let west = x - (wide - 1) * SPACING / 2;
    let north = y + FIRST_ROW;
    (plants.into_iter().enumerate())
        .map(|(index, def)| {
            let column = (index % COLUMNS) as i64;
            let row = (index / COLUMNS) as i64;
            let at = (west + column * SPACING, north + row * SPACING);
            (at, def.key.as_str(), stage)
        })
        .collect()
}
