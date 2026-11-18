//! `VegetableTag`: a vegetable named by a number, for effects' sources, which
//! must be `Copy` (`bonus::Source`).

use serde::{Deserialize, Serialize};

use crate::rng::Seed;

/// A vegetable's key hashed: the same on both sides and in every build, so an
/// effect saved or sent names its vegetable (`Vegetables::by_tag`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VegetableTag(pub u64);

impl VegetableTag {
    /// The tag of the vegetable with this key.
    pub fn of(key: &str) -> Self {
        Self(Seed::new(0).derive("vegetable").derive(key).value())
    }
}
