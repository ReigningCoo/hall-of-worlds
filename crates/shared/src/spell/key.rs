//! `SpellKey`: a spell known by a short number made from its design, so the
//! action bar (whose slots are small and copied about) can hold one, and the
//! spellbook finds it again (magic plan PR 5).

use serde::{Deserialize, Serialize};

use super::design::Spell;

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
        // Postcard writes the same bytes for the same design everywhere.
        let bytes = postcard::to_allocvec(spell).unwrap_or_default();
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in bytes {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        Self(hash)
    }
}
