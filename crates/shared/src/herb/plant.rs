//! `PlantLook`: what a herb's plant looks like, for the client's models. Each
//! plant varies a little from this by a seed of its own.

use serde::{Deserialize, Serialize};

/// What a herb's plant looks like fully grown.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlantLook {
    pub shape: PlantShape,
    /// How tall it grows, in cm.
    pub height: u16,
    /// How wide it grows, in cm.
    pub spread: u16,
    pub leaf: Rgb,
    pub leaf_size: PartSize,
    /// Its flowers' colour: `None` if they are nothing to see.
    pub flower: Option<Rgb>,
    pub flower_size: PartSize,
    /// What sort of plant it is ("Evergreen shrub").
    pub form: String,
    /// How it looks, in words.
    pub description: String,
}

/// The overall shape a model is built on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PlantShape {
    /// Leaves from the ground.
    Rosette,
    /// An upright stem with flowers up it.
    Spike,
    /// Woody, with many branches.
    Bush,
    /// Upright stems with flat umbrellas of flowers.
    Umbel,
    /// Climbing or trailing.
    Vine,
    /// Grass, reed or sedge.
    Grass,
    /// Floating or standing in water.
    Aquatic,
    Fern,
    Succulent,
    /// A small or coppiced tree.
    Tree,
    /// A leafy upright herb.
    Clump,
    /// A low mat over the ground.
    Creeper,
}

/// How big leaves or flowers are, roughly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PartSize {
    Tiny,
    Small,
    Medium,
    Large,
}

/// A colour: red, green and blue, 0 to 255.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rgb(pub u8, pub u8, pub u8);
