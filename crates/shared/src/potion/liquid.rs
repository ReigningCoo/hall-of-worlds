//! What a full bottle holds: a `Liquid`, a water, an oil or a potion (brewing
//! plan, owner, 2026-11-17: `docs/context/brewing.md`), and `LiquidKind`, the
//! same without a potion's contents, for asking for one.

use serde::{Deserialize, Serialize};

use super::Contents;

/// A water, as the server decides where a bottle is filled. What each does to
/// a brew comes with brewing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Water {
    /// Anywhere else: changes nothing.
    Plain,
    /// Underground: strengthens harmful doses.
    Cave,
    /// Outside with no sun up and a moon: strengthens helpful doses.
    Moon,
    /// From the sea: takes away the strongest helpful dose.
    Salt,
}

impl Water {
    pub const ALL: [Water; 4] = [Self::Plain, Self::Cave, Self::Moon, Self::Salt];

    /// "Water", "Cave Water", "Moon Water" or "Salt Water".
    pub fn name(self) -> &'static str {
        match self {
            Self::Plain => "Water",
            Self::Cave => "Cave Water",
            Self::Moon => "Moon Water",
            Self::Salt => "Salt Water",
        }
    }
}

/// An oil, pressed from vegetables, nuts or seeds; kept generic. Each will
/// suit one row of the attributes: Vegetable Body, Nut Mind, Seed Spirit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Oil {
    Vegetable,
    Nut,
    Seed,
}

impl Oil {
    pub const ALL: [Oil; 3] = [Self::Vegetable, Self::Nut, Self::Seed];

    /// "Vegetable Oil", "Nut Oil" or "Seed Oil".
    pub fn name(self) -> &'static str {
        match self {
            Self::Vegetable => "Vegetable Oil",
            Self::Nut => "Nut Oil",
            Self::Seed => "Seed Oil",
        }
    }
}

/// What a full bottle holds. A potion's contents are never empty: an empty
/// bottle holds no liquid at all (`BottleForm::liquid` is `None`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Liquid {
    Water(Water),
    Oil(Oil),
    Potion(Contents),
}

impl Liquid {
    /// Its kind, without a potion's contents.
    pub fn kind(&self) -> LiquidKind {
        match self {
            Self::Water(water) => LiquidKind::Water(*water),
            Self::Oil(oil) => LiquidKind::Oil(*oil),
            Self::Potion(_) => LiquidKind::Potion,
        }
    }

    /// A potion's contents; none for a water or an oil.
    pub fn contents(&self) -> Option<&Contents> {
        match self {
            Self::Potion(contents) => Some(contents),
            _ => None,
        }
    }

    /// What a bottle of it is called: "Water", "Nut Oil", "Potion".
    pub fn name(&self) -> &'static str {
        self.kind().name()
    }
}

/// A kind of liquid, a potion without its contents: what Give Item asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiquidKind {
    Water(Water),
    Oil(Oil),
    Potion,
}

impl LiquidKind {
    /// Every kind: the waters, the oils, then a potion.
    pub fn all() -> Vec<LiquidKind> {
        let waters = Water::ALL.map(Self::Water);
        let oils = Oil::ALL.map(Self::Oil);
        waters
            .into_iter()
            .chain(oils)
            .chain([Self::Potion])
            .collect()
    }

    /// "Water", "Seed Oil", "Potion".
    pub fn name(self) -> &'static str {
        match self {
            Self::Water(water) => water.name(),
            Self::Oil(oil) => oil.name(),
            Self::Potion => "Potion",
        }
    }
}
