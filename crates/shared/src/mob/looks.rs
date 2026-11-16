//! Mobs' looks, rolled from their own hashes: each trait's choices equally likely,
//! unless the kind has weights for it (`WEIGHTS`), as highland cattle's coats.

use super::TRIES;
use crate::creature::CreatureKind;
use crate::sheet::{Appearance, Trait};

/// How likely each choice is, in order, for a kind's trait: highland cattle are
/// mostly red-brown (owner, from the breed's records): 60 red-brown, 22
/// yellow-brown, 6 silvery brown, 6 black, and 6 brindled, split between the two
/// brindles.
const WEIGHTS: &[(CreatureKind, Trait, &[u64])] =
    &[(CreatureKind::Coo, Trait::CoatColour, &[60, 22, 6, 6, 3, 3])];

/// The weights for a kind's trait, if it has any; otherwise all choices are even.
pub(super) fn weights(kind: CreatureKind, which: Trait) -> Option<&'static [u64]> {
    WEIGHTS
        .iter()
        .find(|(of, trait_, _)| *of == kind && *trait_ == which)
        .map(|(_, _, weights)| *weights)
}

/// The choice a roll lands on among these weights.
pub(super) fn weighted(rolled: u64, weights: &[u64]) -> u8 {
    let mut left = rolled % weights.iter().sum::<u64>().max(1);
    let place = weights.iter().position(|&weight| {
        let here = left < weight;
        left -= weight.min(left);
        here
    });
    place.unwrap_or(0) as u8
}

/// A look for a creature of this kind, by the rolls given.
pub(super) fn looks(roll: &impl Fn(i64) -> u64, kind: CreatureKind) -> Appearance {
    let chosen = |(place, &which): (usize, &Trait)| {
        let rolled = roll(TRIES * 2 + 1 + place as i64);
        let choice = match weights(kind, which) {
            Some(weights) => weighted(rolled, weights),
            None => (rolled % which.choices_for(kind).len() as u64) as u8,
        };
        (which, choice)
    };
    Appearance(kind.def().looks.iter().enumerate().map(chosen).collect())
}

#[cfg(test)]
mod tests;
