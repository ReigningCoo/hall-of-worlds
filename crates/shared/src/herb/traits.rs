//! A herb's plainer properties: how common it is, where it grows and what part
//! of it is used.

use serde::{Deserialize, Serialize};

/// How common a herb is where it can grow.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Abundance {
    Common,
    Uncommon,
    Rare,
}

/// The yearly average temperatures a herb grows in, in °C: the climate's own
/// measure (`climate::Climate`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Temperature {
    pub min: i16,
    pub max: i16,
}

impl Temperature {
    /// Whether a place this warm suits it.
    pub fn suits(self, celsius: i16) -> bool {
        (self.min..=self.max).contains(&celsius)
    }
}

/// The part of a herb that is used, which decides what its item looks like.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Part {
    Leaves,
    /// Flowering tops, or the whole herb in flower.
    Tops,
    Flowers,
    Buds,
    Stigmas,
    Fronds,
    Gel,
    Seeds,
    Pods,
    Berries,
    Fruit,
    Bark,
    Resin,
    Root,
    Rhizome,
    Tuber,
    Bulb,
    Stalks,
}
