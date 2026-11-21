//! A dish's balance, the counterpart of Alchemy's purity: points for the
//! roles it has enough of, less points for anything past its max share and
//! for medicinal herbs' bitterness. Water counts for none of it. Whole
//! numbers only.

use super::{Ingredients, Portion, Role, Usage};

/// The most roles that count: a dish with four is as balanced as one with
/// more.
pub const ROLES_COUNTED: i32 = 4;
/// A role counts once it is at least this share of the dish's role weight,
/// in percent...
pub const ROLE_PERCENT: u32 = 10;
/// ...or this, for seasonings (Herb, Spice, Aromatic), which are used in
/// small amounts.
pub const SEASONING_PERCENT: u32 = 1;
/// Each part of this many percentage points an ingredient is past its max
/// share takes a point off.
pub const EXCESS_STEP: u32 = 20;
/// The roles counted where balance adds nothing to quality, nor takes away.
pub const BALANCED_ROLES: i32 = 2;

/// The balance score of what these portions hold: the roles counted (each
/// food adding its weight to its main role in full and to its other roles at
/// half), at most `ROLES_COUNTED`; less a point for each `EXCESS_STEP`
/// (or part) an ingredient's share by weight is past its max share; less a
/// point for each medicinal herb in it. Foods with no culinary data are left
/// out (`plan` refuses them first).
pub fn balance(portions: &[Portion], ingredients: &Ingredients) -> i32 {
    // Each food once, its grams added up, in a steady order.
    let mut foods: Vec<(&super::IngredientDef, u32)> = Vec::new();
    for portion in portions {
        let Portion::Food {
            source, key, grams, ..
        } = portion
        else {
            continue;
        };
        let Some(def) = ingredients.get(*source, key) else {
            continue;
        };
        match foods.iter_mut().find(|(d, _)| std::ptr::eq(*d, def)) {
            Some((_, total)) => *total += grams,
            None => foods.push((def, *grams)),
        }
    }
    let total: u32 = foods.iter().map(|(_, g)| g).sum();
    if total == 0 {
        return 0;
    }
    let mut weights = [0u64; 10];
    for (def, grams) in &foods {
        for &role in &def.roles {
            let full = def.main == Some(role);
            weights[role_index(role)] += u64::from(*grams) * if full { 2 } else { 1 };
        }
    }
    let role_total: u64 = weights.iter().sum();
    let counted = ROLES
        .iter()
        .filter(|&&role| {
            let percent = if is_seasoning(role) {
                SEASONING_PERCENT
            } else {
                ROLE_PERCENT
            };
            let weight = weights[role_index(role)];
            weight > 0 && weight * 100 >= role_total * u64::from(percent)
        })
        .count() as i32;
    let mut score = counted.min(ROLES_COUNTED);
    for (def, grams) in &foods {
        let share = u64::from(*grams) * 100 / u64::from(total);
        let past = share.saturating_sub(u64::from(def.max_share)) as u32;
        let inedible = def.max_share == 0;
        score -= past.div_ceil(EXCESS_STEP).max(u32::from(inedible)) as i32;
        if def.usage == Usage::MedicinalOnly {
            score -= 1;
        }
    }
    score
}

/// Every role, in `Role`'s order.
const ROLES: [Role; 10] = [
    Role::Starch,
    Role::Protein,
    Role::Vegetable,
    Role::Fat,
    Role::Sweet,
    Role::Acid,
    Role::Nut,
    Role::Aromatic,
    Role::Herb,
    Role::Spice,
];

fn role_index(role: Role) -> usize {
    role as usize
}

/// Herbs, spices and aromatics season a dish in small amounts.
pub fn is_seasoning(role: Role) -> bool {
    matches!(role, Role::Herb | Role::Spice | Role::Aromatic)
}
