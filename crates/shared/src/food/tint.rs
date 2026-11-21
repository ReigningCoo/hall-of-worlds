//! A food's tint, for its picture (and PR 11's models): its foods' colours
//! by their main role, weighed by grams, browned by a Pan or an Oven.

use crate::cooking::{History, Ingredients, Portion, Role, Source, Tool};

/// What a food with no main role, or nothing at all, looks like.
const PLAIN: [u8; 3] = [200, 190, 170];

/// The colour of a food mostly of this role.
const fn role_colour(role: Role) -> [u8; 3] {
    match role {
        Role::Starch => [225, 200, 150],
        Role::Protein => [160, 100, 80],
        Role::Vegetable => [110, 150, 60],
        Role::Fat => [240, 215, 120],
        Role::Sweet => [200, 80, 90],
        Role::Acid => [230, 170, 60],
        Role::Nut => [170, 120, 70],
        Role::Aromatic => [220, 205, 170],
        Role::Herb => [70, 125, 55],
        Role::Spice => [180, 75, 40],
    }
}

/// The tint of these portions after these steps, as sRGB bytes.
pub fn tint(portions: &[Portion], history: &History, ingredients: &Ingredients) -> [u8; 3] {
    let mut sum = [0u64; 3];
    let mut weight = 0u64;
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
        let colour = match (def.source, def.main) {
            (Source::Milk, _) => [245, 240, 225],
            (Source::Egg, _) => [240, 200, 90],
            (_, Some(role)) => role_colour(role),
            (_, None) => PLAIN,
        };
        for (total, part) in sum.iter_mut().zip(colour) {
            *total += u64::from(part) * u64::from(*grams);
        }
        weight += u64::from(*grams);
    }
    if weight == 0 {
        return PLAIN;
    }
    let browned = (history.tools.iter()).any(|tool| matches!(tool, Tool::Pan | Tool::Oven));
    let shade = if browned { 80 } else { 100 };
    sum.map(|total| (total / weight * shade / 100).min(255) as u8)
}
