//! `VesselKind`: the eight vessels, with what each holds, weighs and takes in
//! the pack (Claude's first guesses for the owner to tune).

use serde::{Deserialize, Serialize};

use crate::item::Shape;

/// Which vessel one is. Saved by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum VesselKind {
    /// A lidded pewter tankard: 0.5 L.
    Tankard,
    /// A small earthenware cup: 0.25 L.
    Cup,
    /// A stemmed pewter goblet: 0.2 L.
    Goblet,
    /// A cow horn with a metal rim: 0.3 L.
    DrinkingHorn,
    /// A dark glass bottle with a cork, for wine: 0.75 L. Unlike a potion's
    /// small vial, it can be sealed (`seals`).
    WineBottle,
    /// An earthenware jug or flagon: 1.5 L.
    Jug,
    /// A leather-covered flask with a stopper: 0.5 L.
    Flask,
    /// A sewn leather waterskin: 2 L.
    Waterskin,
}

impl VesselKind {
    /// Smallest first.
    pub const ALL: [Self; 8] = [
        Self::Goblet,
        Self::Cup,
        Self::DrinkingHorn,
        Self::Tankard,
        Self::Flask,
        Self::WineBottle,
        Self::Jug,
        Self::Waterskin,
    ];

    /// "Tankard", "Drinking Horn", ...
    pub fn name(self) -> &'static str {
        match self {
            Self::Tankard => "Tankard",
            Self::Cup => "Cup",
            Self::Goblet => "Goblet",
            Self::DrinkingHorn => "Drinking Horn",
            Self::WineBottle => "Wine Bottle",
            Self::Jug => "Jug",
            Self::Flask => "Flask",
            Self::Waterskin => "Waterskin",
        }
    }

    /// How many millilitres it holds.
    pub const fn most_ml(self) -> u32 {
        match self {
            Self::Tankard => 500,
            Self::Cup => 250,
            Self::Goblet => 200,
            Self::DrinkingHorn => 300,
            Self::WineBottle => 750,
            Self::Jug => 1500,
            Self::Flask => 500,
            Self::Waterskin => 2000,
        }
    }

    /// What it weighs empty, in grams.
    pub const fn empty_grams(self) -> u32 {
        match self {
            Self::Tankard => 450,
            Self::Cup => 200,
            Self::Goblet => 250,
            Self::DrinkingHorn => 250,
            Self::WineBottle => 500,
            Self::Jug => 1100,
            Self::Flask => 300,
            Self::Waterskin => 300,
        }
    }

    /// The cells it takes in the pack.
    pub const fn shape(self) -> Shape {
        match self {
            Self::Tankard | Self::Cup | Self::Goblet | Self::Flask => Shape::rectangle(1, 1),
            Self::DrinkingHorn | Self::WineBottle => Shape::rectangle(1, 2),
            Self::Jug | Self::Waterskin => Shape::rectangle(2, 2),
        }
    }

    /// Whether it can be sealed: a wine bottle, corked when filled from a cask
    /// (drinks decision 15).
    pub const fn seals(self) -> bool {
        matches!(self, Self::WineBottle)
    }
}
