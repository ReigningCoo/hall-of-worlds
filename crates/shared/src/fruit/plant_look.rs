//! How a fruit's plant looks: data for the client's plant models (fruit plan
//! PR 4), unused by the rules. Measured from the research
//! (`docs/context/research-fruit.md`), as herbs' `PlantLook` is.

use serde::{Deserialize, Serialize};

use crate::herb::{PartSize, Rgb};

/// A fruit plant's look, besides its family (`FruitDef::plant`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FruitPlantLook {
    /// How tall it grows, in centimetres, as usually seen (a vine's support).
    pub height: u16,
    /// How wide one plant spreads, in centimetres.
    pub spread: u16,
    pub leaf: Rgb,
    pub leaf_size: PartSize,
    pub flower: Rgb,
    pub flower_size: PartSize,
    /// Where its fruit hangs.
    pub fruits: FruitPosition,
}

/// Where on its plant a fruit hangs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FruitPosition {
    /// In clusters or bunches along the twigs.
    Clusters,
    /// One or two at a time along the twigs.
    Single,
    /// At the tips of stems, arms or pads.
    Tips,
    /// On the trunk and the big branches (cacao, jackfruit).
    Trunk,
    /// In bunches at or under the crown (palms, banana, papaya).
    Crown,
    /// Lying on the ground (melons).
    Ground,
}
