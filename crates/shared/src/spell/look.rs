//! How a where looks on screen (magic plan PR 7): coloured by the effects it
//! carries, which is every effect after it in the spell, Void read. Sent with
//! the news of each where so every client draws it alike.

use serde::{Deserialize, Serialize};

use super::design::{EffectStep, Spell, Step};
use super::part::{Part, parts};

/// The most parts a look keeps.
pub const LOOK_PARTS: usize = 4;

/// The parts a where carries, the first written first, each once, at most
/// `LOOK_PARTS`. Empty draws as plain magic.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Look {
    pub parts: [Option<Part>; LOOK_PARTS],
}

impl Look {
    /// The look of `spell`'s step at `index` and everything after it: the
    /// effects of every effect step from there on, in order.
    pub fn from_step(spell: &Spell, index: usize) -> Self {
        let mut look = Self::default();
        let steps = spell.steps.iter().skip(index);
        let found = steps.filter_map(|step| match step {
            Step::Effects(step) => Some(parts(step)),
            Step::Where(_) => None,
        });
        for (part, _) in found.flatten() {
            look.add(part);
        }
        look
    }

    /// The look of one effect step alone: what it lands as.
    pub fn of_effects(step: &EffectStep) -> Self {
        let mut look = Self::default();
        for (part, _) in parts(step) {
            look.add(part);
        }
        look
    }

    /// Adds `part` if it isn't there and there is room.
    fn add(&mut self, part: Part) {
        if self.parts.contains(&Some(part)) {
            return;
        }
        if let Some(free) = self.parts.iter_mut().find(|slot| slot.is_none()) {
            *free = Some(part);
        }
    }

    /// Its parts, in order.
    pub fn iter(&self) -> impl Iterator<Item = Part> + '_ {
        self.parts.iter().flatten().copied()
    }
}
