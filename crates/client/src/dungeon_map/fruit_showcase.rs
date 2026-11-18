//! The fruit showcase (debug API `client.fruit_showcase`): every fruit plant
//! (of one family, if asked) at one stage, in rows south of the player (in
//! front of them as the camera starts), in place of the plants there, to look
//! at them all.

use shared::fruit::{FruitStage, Fruits, PlantFamily};

/// Plants in a row.
const COLUMNS: usize = 8;

/// Cells from one plant to the next: trees stand this far apart in orchards.
const SPACING: i64 = 3;

/// Every fruit plant (not herbs' fruits, which grow as herbs) of `family`, or
/// all, at `stage`, by cell: in `COLUMNS` a row, in the table's order, from
/// the north-west, the first row `SPACING` south of `(x, y)`.
pub(super) fn cells(
    (x, y): (i64, i64),
    (stage, family): (FruitStage, Option<PlantFamily>),
) -> Vec<((i64, i64), &'static str, FruitStage)> {
    let plants: Vec<_> = (Fruits::built_in().iter())
        .filter(|def| !def.herb && family.is_none_or(|family| def.plant == family))
        .collect();
    let wide = plants.len().clamp(1, COLUMNS) as i64;
    let west = x - (wide - 1) * SPACING / 2;
    let north = y + SPACING;
    (plants.into_iter().enumerate())
        .map(|(index, def)| {
            let column = (index % COLUMNS) as i64;
            let row = (index / COLUMNS) as i64;
            let at = (west + column * SPACING, north + row * SPACING);
            (at, def.key.as_str(), stage)
        })
        .collect()
}
