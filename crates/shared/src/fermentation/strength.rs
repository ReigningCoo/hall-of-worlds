//! How strong a drink ferments (owner: about kcal a litre ÷ 68, capped by the
//! yeast's tolerance by its quality; leftover sugar is sweetness; under 1%
//! nothing ferments).

use crate::item::Quality;

use super::{DrinkKind, Mash};

/// kcal a litre for each percent of alcohol.
pub const KCAL_PER_PERCENT: u64 = 68;

/// The least strength anything ferments to: 1%.
pub const LEAST_STRENGTH: u32 = 10;

/// What wild yeast (opened juice left alone) stands: 6%.
pub const WILD_TOLERANCE: u32 = 60;

/// The strongest a yeast of `quality` ferments to before it dies, in tenths
/// of a percent: Poor 7%, Common 9% (bread-like), up 2% a step to
/// Legendary's 17%, Artifact 20%.
pub fn tolerance(quality: Quality) -> u32 {
    match quality {
        Quality::Poor => 70,
        Quality::Common => 90,
        Quality::Uncommon => 110,
        Quality::Rare => 130,
        Quality::Epic => 150,
        Quality::Legendary => 170,
        Quality::Artifact => 200,
    }
}

/// The tolerance of the best of `yeasts` (the hardiest keeps going after the
/// rest die), or `WILD_TOLERANCE` with none.
pub fn best_tolerance(yeasts: &[Quality]) -> u32 {
    yeasts
        .iter()
        .map(|&q| tolerance(q))
        .max()
        .unwrap_or(WILD_TOLERANCE)
}

/// How sweet a drink is by the sugar left in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sweetness {
    /// Under 40 kcal a litre left (about 10 g of sugar).
    Dry,
    /// Under 100.
    OffDry,
    /// Under 180.
    Medium,
    Sweet,
}

impl Sweetness {
    pub fn of(left_kcal_per_litre: u64) -> Self {
        match left_kcal_per_litre {
            0..40 => Self::Dry,
            40..100 => Self::OffDry,
            100..180 => Self::Medium,
            _ => Self::Sweet,
        }
    }
}

/// What a mash ferments into.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fermented {
    pub kind: DrinkKind,
    /// Tenths of a percent of alcohol.
    pub strength: u32,
    /// Sugar left, kcal a litre.
    pub left_kcal: u64,
    pub sweetness: Sweetness,
}

/// What `mash` ferments into with yeast standing `tolerance` (tenths of a
/// percent: `best_tolerance` of the yeasts used); nothing under
/// `LEAST_STRENGTH`, as it would be weaker than 1%.
pub fn ferment(mash: &Mash, tolerance: u32) -> Option<Fermented> {
    let per_litre = mash.kcal_per_litre();
    let could = per_litre * 10 / KCAL_PER_PERCENT;
    let strength = could.min(u64::from(tolerance)) as u32;
    if strength < LEAST_STRENGTH {
        return None;
    }
    let left_kcal = per_litre - u64::from(strength) * KCAL_PER_PERCENT / 10;
    Some(Fermented {
        kind: mash.base.kind(),
        strength,
        left_kcal,
        sweetness: Sweetness::of(left_kcal),
    })
}
