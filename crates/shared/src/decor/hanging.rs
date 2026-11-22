//! What hangs on a wall's face.

use serde::{Deserialize, Serialize};

use super::SCONCE_LIGHT_CELLS;

/// One thing hanging on a wall. Each takes one cell's width of wall; how it
/// looks beyond that (which beast, which weapons, colours) is the client's,
/// from a hash of where it hangs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Hanging {
    /// A stag's antlers on a shield-shaped board.
    Antlers,
    /// A boar's head on a board.
    BoarHead,
    /// A painted shield.
    Shield,
    /// Two weapons crossed behind a small shield.
    CrossedWeapons,
    /// Bundles of herbs drying on pegs.
    HerbBundles,
    /// Hams hanging from pegs.
    Hams,
    /// Strings of onions on pegs.
    Onions,
    /// A woven tapestry on a rod.
    Tapestry,
    /// A painted sign on a bracket.
    Sign,
    /// A board of coat pegs.
    CoatPegs,
    /// A notice board with notes pinned to it.
    NoticeBoard,
    /// A candle sconce: a fixed light.
    Sconce,
}

impl Hanging {
    pub const ALL: [Self; 12] = [
        Self::Antlers,
        Self::BoarHead,
        Self::Shield,
        Self::CrossedWeapons,
        Self::HerbBundles,
        Self::Hams,
        Self::Onions,
        Self::Tapestry,
        Self::Sign,
        Self::CoatPegs,
        Self::NoticeBoard,
        Self::Sconce,
    ];

    /// Its name, for Place Furniture and hovering.
    pub fn name(self) -> &'static str {
        match self {
            Self::Antlers => "Antlers",
            Self::BoarHead => "Boar's Head",
            Self::Shield => "Shield",
            Self::CrossedWeapons => "Crossed Weapons",
            Self::HerbBundles => "Bundles of Herbs",
            Self::Hams => "Hams",
            Self::Onions => "Strings of Onions",
            Self::Tapestry => "Tapestry",
            Self::Sign => "Sign",
            Self::CoatPegs => "Coat Pegs",
            Self::NoticeBoard => "Notice Board",
            Self::Sconce => "Sconce",
        }
    }

    /// How far its light reaches, in cells, if it gives any.
    pub fn light_cells(self) -> Option<u32> {
        (self == Self::Sconce).then_some(SCONCE_LIGHT_CELLS)
    }
}
