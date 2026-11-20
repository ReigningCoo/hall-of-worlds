//! Which looks suit a creature: its kind's traits and choices, and its gender's
//! rules: a female creature has no beard (owner, 2026-11-13), and a choice only
//! for one gender (`Choice::gender`) is only that gender's. Checked by the
//! server for a new character, kept to by mobs' rolls and by old saves as they
//! load, and followed by the creation screen.

use crate::creature::CreatureKind;
use crate::gender::Gender;

use super::{Appearance, Trait};

impl Trait {
    /// Whether a creature of this kind and gender has a choice for this trait:
    /// one of its kind's looks, and not a beard on a female.
    pub fn applies(self, kind: CreatureKind, gender: Gender) -> bool {
        kind.def().looks.contains(&self) && !(self == Trait::Beard && gender == Gender::Female)
    }
}

impl Appearance {
    /// Whether every choice is for a trait this kind and gender have, and is one
    /// of its choices for this gender.
    pub fn suits(&self, kind: CreatureKind, gender: Gender) -> bool {
        self.0.iter().all(|(&which, &choice)| {
            which.applies(kind, gender)
                && which
                    .choices_for(kind)
                    .get(choice as usize)
                    .is_some_and(|choice| choice.suits(gender))
        })
    }

    /// Drops what this gender can't have (a female's beard), and puts a choice
    /// only for another gender back to the first this one may have: for a
    /// gender newly chosen or rolled, and for saves from before the rules.
    pub fn fit(&mut self, kind: CreatureKind, gender: Gender) {
        if gender == Gender::Female {
            self.0.remove(&Trait::Beard);
        }
        for (&which, choice) in self.0.iter_mut() {
            let choices = which.choices_for(kind);
            if choices
                .get(*choice as usize)
                .is_some_and(|chosen| !chosen.suits(gender))
            {
                *choice = which.suiting(kind, gender).first().copied().unwrap_or(0);
            }
        }
    }

    /// This look, as `fit` leaves it.
    pub fn fitted(mut self, kind: CreatureKind, gender: Gender) -> Self {
        self.fit(kind, gender);
        self
    }
}
