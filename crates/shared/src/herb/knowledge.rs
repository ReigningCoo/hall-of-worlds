//! `HerbKnowledge`: what a character has found out about each herb, kept on
//! their `Sheet`. It only changes what is shown: an unknown herb still works.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{EFFECTS, Effect, HerbDef};

/// What is known of one herb.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownHerb {
    /// Its name (picking or buying one teaches it).
    #[serde(default)]
    pub name: bool,
    /// Each of its four effects, in its order (eating one raw teaches the first).
    #[serde(default)]
    pub effects: [bool; EFFECTS],
}

/// What a character knows of herbs, by key: those not here aren't known at all.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HerbKnowledge(BTreeMap<String, KnownHerb>);

impl HerbKnowledge {
    /// What is known of the herb with this key.
    pub fn of(&self, key: &str) -> KnownHerb {
        self.0.get(key).copied().unwrap_or_default()
    }

    pub fn knows_name(&self, key: &str) -> bool {
        self.of(key).name
    }

    /// Whether its effect `n` (0 to 3) is known.
    pub fn knows_effect(&self, key: &str, n: usize) -> bool {
        self.of(key).effects.get(n).copied().unwrap_or(false)
    }

    /// Learns its name: whether it was new.
    pub fn learn_name(&mut self, key: &str) -> bool {
        let known = self.0.entry(key.to_owned()).or_default();
        !std::mem::replace(&mut known.name, true)
    }

    /// Learns its effect `n` (0 to 3): whether it was new. Nothing for any other `n`.
    pub fn learn_effect(&mut self, key: &str, n: usize) -> bool {
        if n >= EFFECTS {
            return false;
        }
        let known = self.0.entry(key.to_owned()).or_default();
        !std::mem::replace(&mut known.effects[n], true)
    }

    /// The effects of `def` that are known, in its order.
    pub fn known_effects<'a>(&self, def: &'a HerbDef) -> impl Iterator<Item = &'a Effect> {
        let known = self.of(&def.key).effects;
        def.effects
            .iter()
            .zip(known)
            .filter_map(|(e, known)| known.then_some(e))
    }

    /// Whether nothing at all is known.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
