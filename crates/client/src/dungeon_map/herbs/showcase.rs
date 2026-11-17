//! The herb showcase (debug API `client.herb_showcase`): every herb the server
//! has, at one stage, in rows round the player, in place of the patches
//! there, to look at them all.

use shared::herb::STAGES;

use crate::common::HerbTable;

/// Herbs in a row.
const COLUMNS: usize = 10;

/// Cells from one herb to the next.
const SPACING: i64 = 2;

/// Every herb at `stage` (kept to 1 to `STAGES`), by cell: in `COLUMNS` a
/// row, in the table's order, from the north-west, round `centre`.
pub(super) fn cells(
    table: &HerbTable,
    (x, y): (i64, i64),
    stage: u8,
) -> Vec<((i64, i64), String, u8)> {
    let stage = stage.clamp(1, STAGES);
    let rows = table.len().div_ceil(COLUMNS) as i64;
    let west = x - (COLUMNS as i64 - 1) * SPACING / 2;
    let north = y - (rows - 1) * SPACING / 2;
    table
        .iter()
        .enumerate()
        .map(|(index, def)| {
            let column = (index % COLUMNS) as i64;
            let row = (index / COLUMNS) as i64;
            let at = (west + column * SPACING, north + row * SPACING);
            (at, def.key.clone(), stage)
        })
        .collect()
}
