//! A Dish in servings: each of its counted stack one serving, eaten one at a
//! time.

use crate::cooking::Portion;

/// About what one serving weighs, in grams: a Dish of more is cut into
/// servings. First guess.
pub const SERVING_GRAMS: u32 = 300;

/// How many servings a dish of `grams` makes: at least 1.
pub fn servings(grams: u32) -> u32 {
    ((grams + SERVING_GRAMS / 2) / SERVING_GRAMS).max(1)
}

/// One serving's share of `portions`, cut `servings` ways (each at least a
/// gram or a ml).
pub(super) fn one_serving(portions: &[Portion], servings: u32) -> Vec<Portion> {
    let cut = |amount: u32| amount.div_ceil(servings.max(1)).max(1);
    (portions.iter())
        .map(|portion| match portion {
            Portion::Food {
                source,
                key,
                grams,
                quality,
            } => Portion::Food {
                source: *source,
                key: key.clone(),
                grams: cut(*grams),
                quality: *quality,
            },
            Portion::Water { ml } => Portion::Water { ml: cut(*ml) },
        })
        .collect()
}
