//! Small kinds of fact about an animal: its class, how rare it is, what it
//! eats, how it feels about water, how often it is kept as a pet.

use serde::{Deserialize, Serialize};

/// What sort of animal it is, for the parts it has (`parts::From`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Class {
    Mammal,
    Bird,
    /// Bees and silkworms: kept as objects (a hive, a rearing tray), never seen
    /// one by one (owner, decision 14). Only their brood or pupae are meat.
    Insect,
}

/// How often one is met: a wild form, feral herds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Never,
    VeryRare,
    Rare,
    Uncommon,
    Common,
    VeryCommon,
}

/// What it eats, broadly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Diet {
    /// Grass and low plants.
    Grazer,
    /// Leaves, twigs and shrubs.
    Browser,
    /// Grazes and browses by season.
    Mixed,
    /// Anything, roots and scraps too.
    Omnivore,
    /// Grain and seed.
    Seeds,
    /// Insects and seed scratched from the ground.
    Forager,
    /// Nectar and pollen.
    Nectar,
    /// One kind of leaf only (the silkworm's white mulberry).
    Leaves,
}

/// How it feels about water, least to most.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Water {
    /// Keeps out of it.
    Avoids,
    /// Only drinks it.
    Drinks,
    /// Stands in shallow water.
    Wades,
    /// Swims when it must.
    Swims,
    /// Bathes and swims by choice.
    Likes,
    /// Would rather be on or in water than on land.
    Prefers,
}

/// How often people keep one as a pet (for the pets task, later).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Often {
    Rare,
    Occasional,
    Common,
    VeryCommon,
}
