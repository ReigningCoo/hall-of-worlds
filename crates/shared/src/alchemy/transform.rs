//! Transformations (Alchemy plan PR 8, owner 2026-11-19): a vegetable as a
//! Retort's solid turning one helpful effect into a magical one. Data, so more
//! can be added: the first is Carrot with Perception making Darkvision. Every
//! helpful dose is replaced by the new effect, as strong as the one it came
//! from; harmful doses stay.

use crate::herb::Stat;
use crate::vegetable::VegetableTag;

/// One transformation: `solid` (a vegetable's key) in a Retort turns a
/// helpful `from` dose into `into`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transformation {
    pub solid: &'static str,
    pub from: Stat,
    pub into: Stat,
}

/// Every transformation.
pub const TRANSFORMATIONS: &[Transformation] = &[Transformation {
    solid: "carrot",
    from: Stat::Perception,
    into: Stat::Darkvision,
}];

/// The transformation this vegetable makes, if any.
pub fn transformation(tag: VegetableTag) -> Option<&'static Transformation> {
    TRANSFORMATIONS
        .iter()
        .find(|change| VegetableTag::of(change.solid) == tag)
}

/// The stat a transformation's effect is made from (Darkvision from
/// Perception), for what is known of it; any other stat is its own.
pub fn made_from(stat: Stat) -> Stat {
    let found = TRANSFORMATIONS.iter().find(|change| change.into == stat);
    found.map_or(stat, |change| change.from)
}
