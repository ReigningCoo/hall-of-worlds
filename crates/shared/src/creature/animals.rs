//! Animals from the creature research (`docs/context/research-creatures.md`) that
//! aren't people: what they are like and what they can carry. Their bodies' looks are
//! `shared::body::BEASTS`. Attributes, skills and weights are first guesses.

use super::bodies::HORSE;
use super::guesses::medians;
use super::{CreatureDef, Size};
use crate::sheet::{Ability, Trait};
use crate::skill::level_practice;

/// Cows, bulls and oxen: heavy, slow and calm.
pub(super) const CATTLE: CreatureDef = CreatureDef {
    name: "Cattle",
    weight: 650_000,
    size: Size::Large,
    inventory: (4, 3),
    // Four legs, a head, a neck and a tail, as a horse's.
    body: &HORSE,
    speed: 90,
    uses_stairs: false,
    playable: false,
    attributes: medians(170, 40, 160, 20),
    bonus_set: &[],
    skills: &[
        ("athletics/running", level_practice(1)),
        ("athletics/swimming", level_practice(1)),
    ],
    abilities: &[Ability::Kick, Ability::Trample],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Highland cattle ("coo" in Scots): shaggy, long-horned and hardy in the cold, a
/// little smaller than other cattle (cows about 450 kg, bulls 650 kg). Their coats are
/// one colour, or brindled: no markings.
pub(super) const COO: CreatureDef = CreatureDef {
    name: "Coo",
    weight: 550_000,
    size: Size::Large,
    inventory: (4, 3),
    body: &HORSE,
    speed: 90,
    uses_stairs: false,
    playable: false,
    attributes: medians(165, 45, 170, 20),
    bonus_set: &[],
    skills: &[
        ("athletics/running", level_practice(1)),
        ("athletics/swimming", level_practice(1)),
    ],
    abilities: &[Ability::Kick, Ability::Trample],
    looks: &[Trait::CoatColour],
};

/// Grey hunters in packs: quick, with keen noses, seeing in the dark.
pub(super) const WOLF: CreatureDef = CreatureDef {
    name: "Wolf",
    weight: 45_000,
    size: Size::Medium,
    inventory: (2, 2),
    body: &HORSE,
    speed: 140,
    uses_stairs: false,
    playable: false,
    attributes: medians(90, 140, 90, 40),
    bonus_set: &[],
    skills: &[
        ("athletics/running", level_practice(4)),
        ("stealth/sneaking", level_practice(3)),
        ("athletics/swimming", level_practice(1)),
    ],
    abilities: &[Ability::NightVision],
    looks: &[Trait::CoatColour, Trait::Markings],
};
