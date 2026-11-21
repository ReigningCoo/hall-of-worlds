//! A food's tint, for its picture and model: its foods' colours by their
//! main role, weighed by grams, browned by a Pan or an Oven; and an accent,
//! the second colour, for its pieces.

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

/// The colour of each food's grams in these portions, as their pictures see
/// them.
fn coloured<'a>(
    portions: &'a [Portion],
    ingredients: &'a Ingredients,
) -> impl Iterator<Item = ([u8; 3], u64)> + 'a {
    portions.iter().filter_map(|portion| {
        let Portion::Food {
            source, key, grams, ..
        } = portion
        else {
            return None;
        };
        let def = ingredients.get(*source, key)?;
        let colour = match (def.source, def.main) {
            (Source::Milk, _) => [245, 240, 225],
            (Source::Egg, _) => [240, 200, 90],
            (_, Some(role)) => role_colour(role),
            (_, None) => PLAIN,
        };
        Some((colour, u64::from(*grams)))
    })
}

/// How much a Pan or an Oven browned it: a share of 100.
fn shade(history: &History) -> u64 {
    let browned = (history.tools.iter()).any(|tool| matches!(tool, Tool::Pan | Tool::Oven));
    if browned { 80 } else { 100 }
}

/// The tint of these portions after these steps, as sRGB bytes.
pub fn tint(portions: &[Portion], history: &History, ingredients: &Ingredients) -> [u8; 3] {
    let mut sum = [0u64; 3];
    let mut weight = 0u64;
    for (colour, grams) in coloured(portions, ingredients) {
        for (total, part) in sum.iter_mut().zip(colour) {
            *total += u64::from(part) * grams;
        }
        weight += grams;
    }
    if weight == 0 {
        return PLAIN;
    }
    let shade = shade(history);
    sum.map(|total| (total / weight * shade / 100).min(255) as u8)
}

/// A second colour for its model's pieces, toppings and fillings (cooking
/// PR 11): the colour with the second most grams, browned as the tint is;
/// `None` when every food shares one colour.
pub fn accent(
    portions: &[Portion],
    history: &History,
    ingredients: &Ingredients,
) -> Option<[u8; 3]> {
    let mut groups: Vec<([u8; 3], u64)> = Vec::new();
    for (colour, grams) in coloured(portions, ingredients) {
        match groups.iter_mut().find(|(seen, _)| *seen == colour) {
            Some((_, total)) => *total += grams,
            None => groups.push((colour, grams)),
        }
    }
    // Most grams first; ties by colour, so the same foods give the same.
    groups.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let (colour, _) = groups.get(1)?;
    let shade = shade(history);
    Some(colour.map(|part| (u64::from(part) * shade / 100) as u8))
}
