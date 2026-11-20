//! Effects that start later on a sheet (`bonus::Later`, fungi plan PR 6):
//! laying them on, and starting each at its tick (from `forget_ended`), when a
//! fungus's poison starting teaches it (`fungus::FungusKnowledge`).

use super::Sheet;
use super::bonuses::Situation;
use crate::bonus::{Effect, Source};
use crate::fungus::Fungi;

impl Sheet {
    /// Lays these on at `now`: those starting by then at once, the rest to
    /// start later, known to its player or not (`known`).
    pub fn lay_on(&mut self, effects: Vec<Effect>, known: bool, situation: Situation, now: u64) {
        let (at_once, later): (Vec<_>, Vec<_>) =
            effects.into_iter().partition(|effect| effect.start <= now);
        self.forget_ended(situation, now);
        for effect in later {
            self.later.add(effect, known);
        }
        self.change_effects(situation, now, |on| {
            for effect in at_once {
                on.add(effect);
            }
        });
    }

    /// One laid on for later starts (its tick already settled by the caller).
    /// A fungus's poison starting is a poison known from then on: the rest of
    /// it coming later (a false recovery's worst) is known too.
    pub(super) fn start_pending(&mut self, effect: Effect) {
        if let Source::FungusPoison(tag) = effect.source
            && let Some(def) = Fungi::built_in().by_tag(tag)
        {
            self.fungi.learn_poison(&def.key);
            self.later.know(effect.source);
        }
        self.effects.add(effect);
    }
}

#[cfg(test)]
#[path = "later_tests.rs"]
mod tests;
