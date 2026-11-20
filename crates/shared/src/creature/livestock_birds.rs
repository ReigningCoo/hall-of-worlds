//! Kept birds from the livestock research (`docs/context/research-livestock.md`,
//! livestock plan PR 5): ducks, Muscovy ducks, geese, turkeys, guineafowl, pigeons
//! and quail (chickens are in `bodies.rs`). Their rows in `shared::livestock`
//! name these kinds (the Muscovy's is a variant of the duck's); their bodies'
//! looks are `shared::body::BEASTS`. Attributes, skills and weights are first
//! guesses, a neutral adult's, as the chicken's are.

use super::bodies::CHICKEN;
use super::guesses::medians;
use super::{CreatureDef, Size};
use crate::sheet::{Ability, Trait};
use crate::skill::level_practice;

/// A kept bird: a chicken's body (head, neck, wings, trunk, legs and feet),
/// fluttering, its looks its feathers and their markings.
const fn bird(
    name: &'static str,
    weight: u32,
    size: Size,
    speed: u32,
    attributes: [u16; 4],
    skills: &'static [(&'static str, u32)],
) -> CreatureDef {
    let [strength, agility, toughness, mind] = attributes;
    CreatureDef {
        name,
        weight,
        size,
        inventory: (2, 2),
        body: &CHICKEN,
        speed,
        uses_stairs: false,
        playable: false,
        attributes: medians(strength, agility, toughness, mind),
        bonus_set: &[],
        skills,
        abilities: &[Ability::Flutter],
        looks: &[Trait::FeatherColour, Trait::Markings],
    }
}

/// Ducks (about 3 kg): waddling on land, at home on water.
pub(super) const DUCK: CreatureDef = bird(
    "Duck",
    3_000,
    Size::Tiny,
    60,
    [10, 70, 20, 10],
    &[("athletics/swimming", level_practice(3))],
);

/// Muscovy ducks (drakes about 4.5 kg, ducks half that): quiet, heavy ducks
/// that perch in trees.
pub(super) const MUSCOVY_DUCK: CreatureDef = bird(
    "Muscovy duck",
    3_500,
    Size::Tiny,
    60,
    [14, 70, 24, 10],
    &[
        ("athletics/climbing", level_practice(1)),
        ("athletics/swimming", level_practice(3)),
    ],
);

/// Geese (about 5 kg): loud, bold guards with a hard bite.
pub(super) const GOOSE: CreatureDef = bird(
    "Goose",
    5_000,
    Size::Small,
    70,
    [25, 70, 35, 15],
    &[("athletics/swimming", level_practice(3))],
);

/// Turkeys (about 9 kg; toms twice hens): big, strutting table birds.
pub(super) const TURKEY: CreatureDef = bird(
    "Turkey",
    9_000,
    Size::Small,
    90,
    [35, 70, 40, 12],
    &[("athletics/running", level_practice(2))],
);

/// Guineafowl (about 1.3 kg): watchful, noisy, quick to run.
pub(super) const GUINEAFOWL: CreatureDef = bird(
    "Guineafowl",
    1_300,
    Size::Tiny,
    90,
    [8, 90, 15, 12],
    &[("athletics/running", level_practice(2))],
);

/// Pigeons (about 350 g): quick fliers that find their way home.
pub(super) const PIGEON: CreatureDef = bird("Pigeon", 350, Size::Tiny, 60, [3, 110, 6, 12], &[]);

/// Quail (about 120 g): small, round and hidden in the grass.
pub(super) const QUAIL: CreatureDef = bird("Quail", 120, Size::Tiny, 60, [2, 110, 4, 8], &[]);
