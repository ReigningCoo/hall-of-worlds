//! What a character has found out about fungi (saved on `Sheet::fungi`).
//! For now only which poisons it knows: learned by suffering one (its effect
//! starting, `Sheet::start_pending`). Names and herb effects come with fungi
//! plan PR 7 (Mycology), which widens `knows_poison`.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// What one character knows of fungi, by key.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FungusKnowledge {
    /// Fungi whose poison it knows.
    #[serde(default)]
    pub poisons: BTreeSet<String>,
}

impl FungusKnowledge {
    /// Whether it knows this fungus's poison: the one place this is decided,
    /// so a delayed poison eaten is recognised (`recognised`).
    pub fn knows_poison(&self, key: &str) -> bool {
        self.poisons.contains(key)
    }

    /// Learns this fungus's poison. Whether it was new.
    pub fn learn_poison(&mut self, key: &str) -> bool {
        self.poisons.insert(key.to_owned())
    }
}
