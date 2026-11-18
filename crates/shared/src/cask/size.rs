//! `CaskSize`: the eight sizes, from the English brewers' casks (each about
//! twice the one before, rounded to whole litres: research in
//! `docs/context/research-drinks.md`).

use serde::{Deserialize, Serialize};

use crate::item::Shape;

/// How big a cask is. Saved by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CaskSize {
    Pin,
    Firkin,
    Kilderkin,
    Barrel,
    Hogshead,
    Puncheon,
    Butt,
    Tun,
}

impl CaskSize {
    /// Smallest first.
    pub const ALL: [Self; 8] = [
        Self::Pin,
        Self::Firkin,
        Self::Kilderkin,
        Self::Barrel,
        Self::Hogshead,
        Self::Puncheon,
        Self::Butt,
        Self::Tun,
    ];

    /// "Pin", "Firkin", ...
    pub fn name(self) -> &'static str {
        match self {
            Self::Pin => "Pin",
            Self::Firkin => "Firkin",
            Self::Kilderkin => "Kilderkin",
            Self::Barrel => "Barrel",
            Self::Hogshead => "Hogshead",
            Self::Puncheon => "Puncheon",
            Self::Butt => "Butt",
            Self::Tun => "Tun",
        }
    }

    /// How many litres it holds.
    pub const fn litres(self) -> u32 {
        match self {
            Self::Pin => 20,
            Self::Firkin => 40,
            Self::Kilderkin => 80,
            Self::Barrel => 160,
            Self::Hogshead => 240,
            Self::Puncheon => 320,
            Self::Butt => 480,
            Self::Tun => 960,
        }
    }

    /// What one of oak weighs empty, in grams: rough guesses, from wine and
    /// spirit barrels' weights (a 225 litre barrique about 50 kg).
    pub const fn oak_grams(self) -> u32 {
        match self {
            Self::Pin => 8_000,
            Self::Firkin => 14_000,
            Self::Kilderkin => 24_000,
            Self::Barrel => 45_000,
            Self::Hogshead => 60_000,
            Self::Puncheon => 80_000,
            Self::Butt => 105_000,
            Self::Tun => 150_000,
        }
    }

    /// The cells it takes, lying on its side: longer than wide, a little
    /// bigger with each size. Big casks simply exist as items for now; how
    /// they are moved is for later (owner).
    pub const fn shape(self) -> Shape {
        match self {
            Self::Pin => Shape::rectangle(2, 3),
            Self::Firkin => Shape::rectangle(3, 3),
            Self::Kilderkin => Shape::rectangle(3, 4),
            Self::Barrel => Shape::rectangle(4, 5),
            Self::Hogshead => Shape::rectangle(5, 5),
            Self::Puncheon => Shape::rectangle(5, 6),
            Self::Butt => Shape::rectangle(6, 7),
            Self::Tun => Shape::rectangle(7, 8),
        }
    }
}
