//! A dish's balance, the counterpart of Alchemy's purity: points for the
//! roles it has enough of, less a point for each role its dish wants and
//! lacks, less points for anything past its max share and for medicinal
//! herbs' bitterness. Water counts for none of it. Whole numbers only.

use super::mix::{Foods, RoleMix};
use super::{Ingredients, Portion, Role, Usage, Wanted};

/// The most roles that count: a dish with four is as balanced as one with
/// more.
pub const ROLES_COUNTED: i32 = 4;
/// Each part of this many percentage points an ingredient is past its max
/// share takes a point off.
pub const EXCESS_STEP: u32 = 20;
/// The roles counted where balance adds nothing to quality, nor takes away.
pub const BALANCED_ROLES: i32 = 2;

/// What the dish an output is wants (`Wants::of`): its roles, and the foods
/// it is made of, which may be past their max share (a fried egg is all
/// egg).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Wants {
    pub roles: Vec<Role>,
    pub exempt: Vec<Vec<Wanted>>,
}

/// The balance score of what these portions hold for a dish wanting
/// `wants`: the roles counted (each food adding its weight to its main role
/// in full and to its other roles at half; medicinal herbs to none), at most
/// `ROLES_COUNTED`; less a point for each wanted role missing; less a point
/// for each `EXCESS_STEP` (or part) a food's share by weight is past its max
/// share, unless the dish is made of it (at least 1 for an inedible one);
/// less a point for each medicinal herb in it.
pub fn balance(portions: &[Portion], ingredients: &Ingredients, wants: &Wants) -> i32 {
    let foods = Foods::of(portions, ingredients);
    if foods.grams() == 0 {
        return 0;
    }
    let mix = RoleMix::of(&foods);
    let mut score = mix.count().min(ROLES_COUNTED);
    score -= wants.roles.iter().filter(|&&role| !mix.has(role)).count() as i32;
    for &(def, grams) in &foods.0 {
        let exempt = (wants.exempt.iter().flatten()).any(|w| w.matches(def.source, &def.key));
        let past = foods.share(grams).saturating_sub(u32::from(def.max_share));
        let inedible = def.max_share == 0;
        if !exempt || inedible {
            score -= past.div_ceil(EXCESS_STEP).max(u32::from(inedible)) as i32;
        }
        if def.usage == Usage::MedicinalOnly {
            score -= 1;
        }
    }
    score
}

/// Herbs, spices and aromatics season a dish in small amounts.
pub fn is_seasoning(role: Role) -> bool {
    matches!(role, Role::Herb | Role::Spice | Role::Aromatic)
}
