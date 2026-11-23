//! `SpellKey`: a spell known by a short number made from its design, so the
//! action bar (whose slots are small and copied about) can hold one, and the
//! spellbook finds it again (magic plan PR 5).

use serde::{Deserialize, Serialize};

use super::design::{EffectStep, Spell, Step, WherePart, WhereStep};
use super::filters::Filters;
use super::place::Where;

/// A spell's design boiled down to a number: the same design always gives the
/// same key, on every machine (FNV-1a over its message bytes).
/// Written as the bare number (`transparent`), so saves' untagged bar slots
/// read it back.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SpellKey(pub u64);

impl SpellKey {
    /// The key of this design.
    pub fn of(spell: &Spell) -> Self {
        // Postcard writes the same bytes for the same design everywhere. A
        // where step of one where is written as before merging (PR 10), so
        // the keys saved on action bars stay the same.
        let steps: Vec<Keyed> = spell.steps.iter().map(Keyed::of).collect();
        let bytes = postcard::to_allocvec(&steps).unwrap_or_default();
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in bytes {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        Self(hash)
    }
}

/// A step as its key is made from: written as `Step` was before merging,
/// with merged where steps a third kind.
#[derive(Serialize)]
enum Keyed<'a> {
    Where {
        place: Where,
        count: u8,
        filters: Filters,
    },
    Effects(&'a EffectStep),
    Merged {
        filters: Filters,
        parts: Vec<WherePart>,
    },
}

impl<'a> Keyed<'a> {
    fn of(step: &'a Step) -> Self {
        match step {
            Step::Where(WhereStep {
                place,
                count,
                filters,
                more,
            }) if more.is_empty() => Keyed::Where {
                place: *place,
                count: *count,
                filters: *filters,
            },
            Step::Where(step) => Keyed::Merged {
                filters: step.filters,
                parts: step.parts().collect(),
            },
            Step::Effects(step) => Keyed::Effects(step),
        }
    }
}
