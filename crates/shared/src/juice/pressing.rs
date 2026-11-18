//! How much juice a fruit gives (owner: by its kcal and weight, realistic, so
//! one apple in a butt does nothing), which fruit can be juiced, and how long
//! a pressing takes.

use crate::fruit::{FruitDef, FruitSet, Fruits};
use crate::skill::Skills;

/// The most fruit one pressing takes, in grams: about a dozen apples.
pub const PRESSING_GRAMS: u32 = 2000;

/// Alchemy practice for each pressing, as for a bottle of oil.
pub const JUICE_PRACTICE: u32 = crate::potion::PRESS_PRACTICE;

/// The most kcal in 100 g a fruit may have to be juiced: drier or oilier
/// fruit (dates, avocados, bananas' kin) give a paste, not a juice.
const MOST_KCAL_PER_100G: u32 = 120;

/// The share of a fruit's water a press gets out, in thousandths.
const PRESSED_OUT: u32 = 800;

/// The share of a fruit's kcal (its sugar) that goes into the juice, in
/// thousandths; the rest stays in the pulp.
const SUGAR_IN_JUICE: u32 = 600;

/// Whether `def` can be pressed into juice: a fruit item (not a nut or a
/// herb's fruit), safe raw, and juicy enough.
pub fn juicable(def: &FruitDef) -> bool {
    def.is_item()
        && def.set == FruitSet::Fruit
        && !def.harmful_raw
        && def.grams > 0
        && def.kcal * 100 <= MOST_KCAL_PER_100G * def.grams
}

/// Every fruit that can be juiced, in the table's order.
pub fn juice_fruits() -> impl Iterator<Item = &'static FruitDef> {
    Fruits::built_in().items().filter(|def| juicable(def))
}

/// The water in each gram of `def`, in thousandths: what isn't its dry
/// matter, taken as a quarter of its kcal per gram (sugar is about 4 kcal a
/// gram). An apple, 52 kcal in 100 g: 870.
fn water_share(def: &FruitDef) -> u32 {
    let dry = (def.kcal * 250).div_ceil(def.grams.max(1));
    1000u32.saturating_sub(dry)
}

/// The millilitres of juice `grams` of `def` give, nothing if it can't be
/// juiced: `PRESSED_OUT` of its water, a gram a millilitre. A kilogram of
/// apples: 696 ml.
pub fn ml_from(def: &FruitDef, grams: u32) -> u32 {
    if !juicable(def) {
        return 0;
    }
    let share = u64::from(water_share(def)) * u64::from(PRESSED_OUT);
    (u64::from(grams) * share / 1_000_000) as u32
}

/// The kcal in a litre of `def`'s juice (for fermenting later: strength is
/// about this ÷ 68): `SUGAR_IN_JUICE` of the fruit's kcal in the juice it
/// gives. Apple about 450, grape about 630; none if it can't be juiced.
pub fn kcal_per_litre(def: &FruitDef) -> u32 {
    let ml = ml_from(def, def.grams);
    if ml == 0 {
        return 0;
    }
    let kcal = u64::from(def.kcal) * u64::from(SUGAR_IN_JUICE);
    (kcal / u64::from(ml)) as u32
}

/// How long one pressing takes, in action ticks: as long as a bottle of oil
/// (15 down to 11 seconds by Alchemy).
pub fn press_ticks(skills: &Skills) -> u64 {
    crate::potion::press_ticks(skills)
}
