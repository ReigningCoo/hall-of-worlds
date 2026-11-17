//! `FruitTag`: a fruit named by a number, for effects' sources, which must be
//! `Copy` (`bonus::Source`).

use serde::{Deserialize, Serialize};

use crate::rng::Seed;

/// A fruit's key hashed: the same on both sides and in every build, so an
/// effect saved or sent names its fruit (`Fruits::by_tag`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FruitTag(pub u64);

impl FruitTag {
    /// The tag of the fruit with this key.
    pub fn of(key: &str) -> Self {
        Self(Seed::new(0).derive("fruit").derive(key).value())
    }
}
