//! How many units a drink holds, and the Tolerance it teaches.

use crate::potion::Liquid;

/// Tolerance practice for each whole unit drunk while tipsy or worse.
pub const TOLERANCE_PRACTICE_PER_UNIT: u32 = 10;

/// Thousandths of a unit in `ml` of `liquid`: a drink's strength (tenths of a
/// percent) of its volume, at 10 ml of pure alcohol a unit, so 250 ml of 5%
/// ale is 1.25 units; pure alcohol is 10 a 100 ml. Nothing else holds any
/// (vinegar's has turned): `Liquid::alcohol_strength`.
pub fn units_in(liquid: &Liquid, ml: u32) -> u32 {
    let milli = u64::from(ml) * u64::from(liquid.alcohol_strength()) / 10;
    u32::try_from(milli).unwrap_or(u32::MAX)
}

/// Tolerance practice for drinking `milli` thousandths of a unit, if it left
/// the drinker tipsy or worse.
pub fn tolerance_practice(milli: u32, tipsy_or_worse: bool) -> u32 {
    match tipsy_or_worse {
        true => milli * TOLERANCE_PRACTICE_PER_UNIT / 1000,
        false => 0,
    }
}
