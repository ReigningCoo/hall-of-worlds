//! What a character has found out about fungi (saved on `Sheet::fungi`), as
//! `herb::HerbKnowledge` for herbs: names (learned by a Mycology check on
//! picking, `names_on_picking`), the four herb effects of those used like
//! herbs, and poisons (learned by suffering one, `Sheet::start_pending`, or
//! known with the name). It only changes what is shown and what is recognised
//! on eating: an unknown fungus still works.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::herb::EFFECTS;

/// What one character knows of fungi, by key.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FungusKnowledge {
    /// Fungi whose poison it has suffered.
    #[serde(default)]
    pub poisons: BTreeSet<String>,
    /// Fungi it knows by name.
    #[serde(default)]
    pub names: BTreeSet<String>,
    /// The herb effects it knows of fungi used like herbs, each in its order.
    #[serde(default)]
    pub effects: BTreeMap<String, [bool; EFFECTS]>,
}

impl FungusKnowledge {
    /// Whether it knows this fungus by name.
    pub fn knows_name(&self, key: &str) -> bool {
        self.names.contains(key)
    }

    /// Learns its name: whether it was new.
    pub fn learn_name(&mut self, key: &str) -> bool {
        self.names.insert(key.to_owned())
    }

    /// Whether it knows this fungus's poison: suffered, or known by name (a
    /// fungus known is known for what it does). The one place this is
    /// decided, so a delayed poison eaten is recognised (`recognised`) and
    /// popups tell it.
    pub fn knows_poison(&self, key: &str) -> bool {
        self.poisons.contains(key) || self.knows_name(key)
    }

    /// Learns this fungus's poison. Whether it was new.
    pub fn learn_poison(&mut self, key: &str) -> bool {
        self.poisons.insert(key.to_owned())
    }

    /// Whether its herb effect `n` (0 to 3) is known.
    pub fn knows_effect(&self, key: &str, n: usize) -> bool {
        (self.effects.get(key)).is_some_and(|known| known.get(n).copied().unwrap_or(false))
    }

    /// Learns its herb effect `n` (0 to 3): whether it was new. Nothing for
    /// any other `n`.
    pub fn learn_effect(&mut self, key: &str, n: usize) -> bool {
        if n >= EFFECTS {
            return false;
        }
        let known = self.effects.entry(key.to_owned()).or_default();
        !std::mem::replace(&mut known[n], true)
    }

    /// Learns its name, poison and every herb effect: whether anything was
    /// new (dev tools).
    pub fn learn_all(&mut self, key: &str) -> bool {
        let mut new = self.learn_name(key);
        new |= self.learn_poison(key);
        for n in 0..EFFECTS {
            new |= self.learn_effect(key, n);
        }
        new
    }

    /// Forgets everything known of fungi (dev tools).
    pub fn forget_all(&mut self) {
        *self = Self::default();
    }
}
