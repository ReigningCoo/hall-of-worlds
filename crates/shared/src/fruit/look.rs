//! How a fruit looks: data for the client's pictures and models (PRs 2 and 4 of
//! the fruit plan), unused by the rules.

use serde::{Deserialize, Serialize};

use crate::herb::Rgb;

/// A fruit's look.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FruitLook {
    pub shape: FruitShape,
    /// About how big one fruit is across or along, in millimetres.
    pub size: u16,
    /// Its colour ripe, and before.
    pub ripe: Rgb,
    pub unripe: Rgb,
    pub skin: Skin,
}

/// One fruit's overall shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FruitShape {
    Round,
    Oval,
    /// Long or curved (banana, carob), or oblong.
    Long,
    Pear,
    /// A cluster of drupelets (raspberry, mulberry).
    Aggregate,
    Pod,
    Nut,
}

/// What the outside of a fruit is like.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Skin {
    Smooth,
    /// A dusty waxy bloom (sloe, blueberry).
    Bloom,
    Downy,
    /// Russeted, netted or warty.
    Rough,
    Spiny,
    /// A thick rind or leathery peel.
    Rind,
    Husk,
    /// A hard shell (nuts).
    Shell,
}
