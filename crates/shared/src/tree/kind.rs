//! `TreeKind`: the wild trees, one row each in `table.rs`.

use serde::{Deserialize, Serialize};

/// A kind of wild tree, as a `Cell::Tree` in the wilderness or an oasis is
/// (`TreeKind::roll`). Fruit trees are their fruit's (`shared::fruit`). Saved by
/// name, so new ones can go anywhere.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TreeKind {
    ScotsPine,
    NorwaySpruce,
    SilverFir,
    Larch,
    DouglasFir,
    RedCedar,
    CedarOfLebanon,
    Cypress,
    SilverBirch,
    Aspen,
    Oak,
    Beech,
    Ash,
    Sycamore,
    Linden,
    WychElm,
    WhiteWillow,
    BlackAlder,
    HolmOak,
    UmbrellaThorn,
    CamelThorn,
    Mesquite,
    Mopane,
    Teak,
    Mahogany,
    Kapok,
    Ebony,
}

/// Needles or leaves: the cover's `narrow` chooses between them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Group {
    Conifer,
    Broadleaf,
}

/// How often a kind comes up where it suits: its weight in the roll.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Commonness {
    Common,
    Uncommon,
    Rare,
}

impl Commonness {
    /// Its weight in the roll: common 4, uncommon 2, rare 1.
    pub const fn weight(self) -> u64 {
        match self {
            Self::Common => 4,
            Self::Uncommon => 2,
            Self::Rare => 1,
        }
    }
}
