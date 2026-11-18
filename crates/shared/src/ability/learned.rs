//! `Learned`: the abilities one character has learned, rather than knowing them
//! by its body or from a skill. Saved on its sheet (`Sheet::abilities`).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::Ability;
use crate::item::Inventory;

/// The abilities a character has learned, kept for good: none in a save from
/// before learning. Saved as a list of names.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Learned(BTreeSet<Ability>);

impl Learned {
    /// Whether it has learned this.
    pub fn knows(&self, ability: Ability) -> bool {
        self.0.contains(&ability)
    }

    /// Learns it: `true` if it is new.
    pub fn learn(&mut self, ability: Ability) -> bool {
        self.0.insert(ability)
    }

    /// Learns what carrying these items teaches: Collect Water on a first
    /// empty bottle (brewing plan, decision 5). `true` if anything is new.
    pub fn learn_from(&mut self, carried: &Inventory) -> bool {
        carried.empty_bottle().is_some() && self.learn(Ability::CollectWater)
    }

    /// Every ability learned, in the enum's order.
    pub fn iter(&self) -> impl Iterator<Item = Ability> + '_ {
        self.0.iter().copied()
    }
}
