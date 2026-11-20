//! Kinds of creature: the same for players and, later, mobs. What each kind's
//! body is like, and so what it can wear. See `docs/context/shared/creature.md`.
//!
//! A table here, as with kinds of item. Different bodies use different locations
//! (`Forelegs` isn't `Legs`), so trousers don't fit a horse, and a centaur is a
//! human top on those.

// Split into files (`cargo xtask lint` keeps files small).
mod animals;
mod bodies;
mod def;
mod guesses;
mod kind;
mod layer;
mod livestock;
mod livestock_birds;
mod livestock_more;
mod names;
mod peoples;
mod senses;
mod size;
mod taurs;

pub use def::{BodyPart, CreatureDef};
pub use kind::CreatureKind;
pub use layer::{Layer, Location};
pub(crate) use names::three_alike;
pub use senses::{Acuity, DarkSight, Senses, Temper};
pub use size::Size;

impl CreatureKind {
    /// Every kind, alphabetically.
    pub const ALL: [Self; 40] = [
        Self::Alpaca,
        Self::BactrianCamel,
        Self::Beggar,
        Self::Birdfolk,
        Self::Cattle,
        Self::Centaur,
        Self::Chicken,
        Self::Coo,
        Self::Donkey,
        Self::Drider,
        Self::Dromedary,
        Self::Duck,
        Self::Dwarf,
        Self::Elf,
        Self::Giant,
        Self::Goat,
        Self::Goblin,
        Self::Goose,
        Self::Guineafowl,
        Self::Halfling,
        Self::Horse,
        Self::Human,
        Self::Kobold,
        Self::Lizardfolk,
        Self::Llama,
        Self::Minotaur,
        Self::Mule,
        Self::MuscovyDuck,
        Self::Orc,
        Self::Pig,
        Self::Pigeon,
        Self::Quail,
        Self::Rabbit,
        Self::Reindeer,
        Self::ScorpionMan,
        Self::Sheep,
        Self::Turkey,
        Self::WaterBuffalo,
        Self::Wolf,
        Self::Yak,
    ];
}

#[cfg(test)]
mod tests;
