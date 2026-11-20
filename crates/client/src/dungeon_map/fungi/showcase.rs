//! The fungus showcase (debug API `client.fungus_showcase`): a patch of every
//! fungus (of one form, if asked) at one stage, in rows south of the player
//! (in front of them as the camera starts), in place of the patches there, to
//! look at them all.

use shared::fungus::{Form, Fungi};

/// Patches in a row.
const COLUMNS: usize = 8;

/// How far south of the player the first row is.
const FIRST_ROW: i64 = 3;

/// Cells from one to the next.
const SPACING: i64 = 2;

/// A patch of every fungus of `form`, or all, at `stage`, all in sight: in
/// `COLUMNS` a row, in the table's order, from the north-west, the first row
/// `FIRST_ROW` south of `(x, y)`.
pub(super) fn cells(
    (x, y): (i64, i64),
    (stage, form): (u8, Option<Form>),
) -> Vec<((i64, i64), String, u8, bool)> {
    let fungi: Vec<_> = (Fungi::built_in().iter())
        .filter(|def| form.is_none_or(|form| def.look.form == form))
        .collect();
    let wide = fungi.len().clamp(1, COLUMNS) as i64;
    let west = x - (wide - 1) * SPACING / 2;
    let north = y + FIRST_ROW;
    (fungi.into_iter().enumerate())
        .map(|(index, def)| {
            let column = (index % COLUMNS) as i64;
            let row = (index / COLUMNS) as i64;
            let at = (west + column * SPACING, north + row * SPACING);
            (at, def.key.clone(), stage, true)
        })
        .collect()
}
