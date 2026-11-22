//! Rugs: just for looks, on floor under whatever stands there.

use serde::{Deserialize, Serialize};

/// The longest side a rug may have, in cells.
pub const RUG_MOST: u8 = 4;

/// A rug's look.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rug {
    pub pattern: RugPattern,
    pub colour: RugColour,
}

/// How a rug is woven.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RugPattern {
    /// One colour with a fringe.
    Plain,
    /// Bands across it.
    Striped,
    /// A wide border round a plain field.
    Bordered,
    /// A diamond in the middle and a border.
    Medallion,
    /// An oval of braided rags.
    Braided,
}

/// A rug's main colour, from dyes of the time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RugColour {
    /// Madder red.
    Madder,
    /// Indigo or woad blue.
    Indigo,
    /// Weld yellow.
    Weld,
    /// Green, weld over woad.
    Green,
    /// Walnut brown.
    Walnut,
    /// Undyed wool.
    Undyed,
}

impl RugPattern {
    pub const ALL: [Self; 5] = [
        Self::Plain,
        Self::Striped,
        Self::Bordered,
        Self::Medallion,
        Self::Braided,
    ];
}

impl RugColour {
    pub const ALL: [Self; 6] = [
        Self::Madder,
        Self::Indigo,
        Self::Weld,
        Self::Green,
        Self::Walnut,
        Self::Undyed,
    ];
}
