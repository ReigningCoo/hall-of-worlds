//! What brewing teaches of herbs (PR 7; the herb knowledge rule that potions
//! teach effects): when a potion forms, the herbs its ointment was ground from
//! (the mix remembered with that ointment's doses, `Recipes`) show their
//! effects on the stats the potion holds. Nothing is learned when no potion
//! forms, or when the ointment's mix isn't remembered (given, or made before
//! the same mix made something else).

use super::Recipes;
use crate::herb::{HerbKnowledge, Herbs};
use crate::potion::Contents;

/// Learns, for each herb of the mix that made `ointment`, its effects on the
/// stats in `potion`: whether anything was new.
pub fn learn_from_potion(
    known: &mut HerbKnowledge,
    recipes: &Recipes,
    herbs: &Herbs,
    ointment: &Contents,
    potion: &Contents,
) -> bool {
    let Some(recipe) = recipes.iter().find(|recipe| &recipe.made == ointment) else {
        return false;
    };
    let mut new = false;
    for key in &recipe.herbs {
        let Some(def) = herbs.get(key) else {
            continue;
        };
        for (n, effect) in def.effects.iter().enumerate() {
            let shown = (potion.doses().iter()).any(|dose| dose.stat == effect.stat);
            if shown {
                new |= known.learn_effect(key, n);
            }
        }
    }
    new
}
