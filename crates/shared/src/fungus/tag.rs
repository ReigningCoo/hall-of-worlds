//! `FungusTag`: a fungus named by a number, for effects' sources, which must
//! be `Copy` (`bonus::Source`).

use serde::{Deserialize, Serialize};

use crate::rng::Seed;

/// A fungus's key hashed: the same on both sides and in every build, so an
/// effect saved or sent names its fungus (`Fungi::by_tag`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FungusTag(pub u64);

impl FungusTag {
    /// The tag of the fungus with this key.
    pub fn of(key: &str) -> Self {
        Self(Seed::new(0).derive("fungus").derive(key).value())
    }
}
