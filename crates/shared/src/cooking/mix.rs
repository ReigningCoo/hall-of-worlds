//! What a set of portions holds, for balance and for naming: each food once
//! with its grams (`Foods`), and the weight of each role (`RoleMix`).
//! Medicinal herbs are only a bitter seasoning: they add to no role.

use super::{IngredientDef, Ingredients, Portion, Role, Usage, is_seasoning};

/// A role counts once it is at least this share of the role weight, in
/// percent...
pub const ROLE_PERCENT: u32 = 10;
/// ...or this, for seasonings (Herb, Spice, Aromatic), used in small amounts.
pub const SEASONING_PERCENT: u32 = 1;

/// Every food in some portions once, its grams added up, in the order first
/// met; water and foods with no culinary data left out.
pub(crate) struct Foods<'a>(pub Vec<(&'a IngredientDef, u32)>);

impl<'a> Foods<'a> {
    pub fn of(portions: &[Portion], ingredients: &'a Ingredients) -> Self {
        let mut foods: Vec<(&IngredientDef, u32)> = Vec::new();
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
        Self(foods)
    }

    pub fn grams(&self) -> u32 {
        self.0.iter().map(|(_, grams)| grams).sum()
    }

    /// A food's share of all, in whole percent.
    pub fn share(&self, grams: u32) -> u32 {
        (u64::from(grams) * 100 / u64::from(self.grams().max(1))) as u32
    }

    /// Whether a herb or spice is within its max share: then it never changes
    /// what a dish is (the leeway).
    pub fn within_leeway(&self, def: &IngredientDef, grams: u32) -> bool {
        matches!(def.main, Some(Role::Herb | Role::Spice))
            && self.share(grams) <= u32::from(def.max_share)
    }
}

/// The weight of each role: each food's grams twice on its main role, once
/// on each other.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct RoleMix {
    weights: [u64; 10],
}

impl RoleMix {
    /// The mix of these foods, medicinal herbs left out.
    pub fn of(foods: &Foods) -> Self {
        Self::of_some(foods, |_, _| true)
    }

    /// The mix with herbs and spices within their max share also left out:
    /// what decides what a dish is.
    pub fn beyond_leeway(foods: &Foods) -> Self {
        Self::of_some(foods, |def, grams| !foods.within_leeway(def, grams))
    }

    fn of_some(foods: &Foods, keep: impl Fn(&IngredientDef, u32) -> bool) -> Self {
        let mut weights = [0u64; 10];
        for &(def, grams) in &foods.0 {
            if def.usage == Usage::MedicinalOnly || !keep(def, grams) {
                continue;
            }
            for &role in &def.roles {
                let full = def.main == Some(role);
                weights[role as usize] += u64::from(grams) * if full { 2 } else { 1 };
            }
        }
        Self { weights }
    }

    pub fn weight(&self, role: Role) -> u64 {
        self.weights[role as usize]
    }

    /// Whether `role` is enough of the mix to count (`ROLE_PERCENT`, or
    /// `SEASONING_PERCENT` for a seasoning).
    pub fn has(&self, role: Role) -> bool {
        let total: u64 = self.weights.iter().sum();
        let percent = match is_seasoning(role) {
            true => SEASONING_PERCENT,
            false => ROLE_PERCENT,
        };
        let weight = self.weight(role);
        weight > 0 && weight * 100 >= total * u64::from(percent)
    }

    /// The heaviest role, the first in `Role`'s order of equals; `None` with
    /// nothing in it.
    pub fn biggest(&self) -> Option<Role> {
        let most = ROLES.iter().map(|&role| self.weight(role)).max()?;
        (most > 0).then(|| ROLES.into_iter().find(|&role| self.weight(role) == most))?
    }

    /// How many roles count.
    pub fn count(&self) -> i32 {
        ROLES.iter().filter(|&&role| self.has(role)).count() as i32
    }
}

/// Every role, in `Role`'s order.
pub(crate) const ROLES: [Role; 10] = [
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
