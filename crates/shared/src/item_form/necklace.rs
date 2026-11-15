//! Necklaces: `NecklaceForm`, its strand (`Strand`: a leather cord, a chain or a
//! torc), how long it hangs (`NecklaceLength`) and what hangs from it (`Ornament`).
//! Its settings are rings' (`Setting`), each gem a whole item.

use serde::{Deserialize, Serialize};

use crate::item_form::Setting;
use crate::material::Material;

/// How a leather cord is made.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CordStyle {
    Plain,
    Braided,
    Knotted,
}

/// How thick a chain's links are.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Thickness {
    Fine,
    Medium,
    Heavy,
}

/// How a chain's links lie: drawn as its texture only.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Link {
    Cable,
    Rope,
    Box,
    Snake,
}

/// The shaped ends of a torc, either side of its opening at the front.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TorcEnd {
    Plain,
    Ball,
    Buffer,
    Scroll,
    /// A beast's head, such as a boar's or a wolf's.
    Beast,
}

/// What goes round the neck.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Strand {
    /// A leather thong.
    Cord { leather: Material, style: CordStyle },
    /// A chain of the necklace's metal.
    Chain { thickness: Thickness, link: Link },
    /// A rigid bar of the necklace's metal, twisted, open at the front.
    Torc { ends: TorcEnd },
}

/// How far down it hangs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NecklaceLength {
    /// Close round the neck.
    Choker,
    /// On the collarbones.
    Collarbone,
    /// On the chest.
    Chest,
    /// Down to the breastbone's end or lower.
    Long,
}

/// A small shape of metal hung on a strand. Nothing religious (owner's decision).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Charm {
    Disc,
    Teardrop,
    Leaf,
    Crescent,
    Acorn,
    Key,
    /// An animal's tooth, cast in metal.
    Tooth,
    Knot,
    Shield,
}

/// The outline of an amulet's medallion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Medallion {
    Round,
    Oval,
    Octagon,
}

/// What hangs from the strand, of the necklace's metal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ornament {
    None,
    /// One setting hung on a bail.
    Pendant,
    /// A medallion: one gem in the middle, or several round its rim.
    Amulet(Medallion),
    /// Several small settings spaced along the front of a chain.
    Stations,
    Charm(Charm),
}

/// One necklace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NecklaceForm {
    /// A chain's or torc's metal, and every ornament's (a cord's too).
    pub metal: Material,
    pub strand: Strand,
    pub length: NecklaceLength,
    pub ornament: Ornament,
    /// A pendant's one; an amulet's, by `angle` round its rim (one alone is in the
    /// middle); stations' by `angle` round the neck from the front. None otherwise.
    pub settings: Vec<Setting>,
}

impl Charm {
    pub const ALL: [Self; 9] = [
        Self::Disc,
        Self::Teardrop,
        Self::Leaf,
        Self::Crescent,
        Self::Acorn,
        Self::Key,
        Self::Tooth,
        Self::Knot,
        Self::Shield,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Disc => "Disc",
            Self::Teardrop => "Teardrop",
            Self::Leaf => "Leaf",
            Self::Crescent => "Crescent",
            Self::Acorn => "Acorn",
            Self::Key => "Key",
            Self::Tooth => "Tooth",
            Self::Knot => "Knot",
            Self::Shield => "Shield",
        }
    }
}
