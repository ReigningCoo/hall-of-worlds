//! `VegetableLook`: what one item of a vegetable looks like, from the research's
//! findings (shape, colours, skin, inside), for the client's pictures and plant
//! models.

use serde::{Deserialize, Serialize};

use crate::herb::Rgb;

/// The most colours a look may have: one usual and six other varieties.
pub const MOST_COLOURS: usize = 7;

/// What one item of a vegetable looks like.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VegetableLook {
    /// Its colours: the usual one first, then other or old varieties (white,
    /// yellow and purple carrots), which some items of it are.
    pub colours: Vec<Rgb>,
    /// A second colour where its picture puts one: a root's shoulder, leaves'
    /// stalks or midribs, a fruit's calyx, a shoot's scales or tips, a bean's
    /// mottling (on the usual colour only), a cob's husk.
    #[serde(default)]
    pub accent: Option<Rgb>,
    /// Its flesh, where it is cut.
    pub inside: Rgb,
    /// One piece's length and width in millimetres: a root, a leaf, a pod, a
    /// seed, an ear or head of grain.
    pub length: u16,
    pub width: u16,
}

impl VegetableLook {
    /// Whether it makes sense: an error saying what is wrong.
    pub fn check(&self) -> Result<(), String> {
        if self.colours.is_empty() || self.colours.len() > MOST_COLOURS {
            return Err(format!("1 to {MOST_COLOURS} colours"));
        }
        if self.length == 0 || self.width == 0 {
            return Err("a look has a length and width".to_owned());
        }
        Ok(())
    }
}
