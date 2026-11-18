//! How a vegetable's plant looks: data for the client's plant models
//! (vegetable plan PR 4), unused by the rules. From the research's findings
//! (`docs/context/research-vegetables.md`), as fruit's `FruitPlantLook` is.

use serde::{Deserialize, Serialize};

use crate::herb::{PartSize, Rgb};

/// A vegetable plant's look, besides its shape (`VegetableDef::plant`) and
/// the part picked (`VegetableDef::look`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VegetablePlantLook {
    /// How tall it stands as a crop, in centimetres (a climber's pole; not a
    /// flower stalk sent up in its second year).
    pub height: u16,
    /// How wide one plant spreads, in centimetres.
    pub spread: u16,
    pub leaf: Rgb,
    pub leaf_size: PartSize,
    /// Its flowers', or a grass's flowering heads'.
    pub flower: Rgb,
    pub flower_size: PartSize,
}

impl VegetablePlantLook {
    /// Whether it makes sense: an error saying what is wrong.
    pub fn check(&self) -> Result<(), String> {
        if self.height == 0 || self.spread == 0 {
            return Err("a plant has a height and spread".to_owned());
        }
        Ok(())
    }
}
