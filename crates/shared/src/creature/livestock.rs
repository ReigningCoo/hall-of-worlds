//! Kept animals from the livestock research (`docs/context/research-livestock.md`,
//! livestock plan PR 3): sheep, goats, pigs, water buffalo and yaks. Their rows in
//! `shared::livestock` name these kinds; their bodies' looks are
//! `shared::body::BEASTS`. Attributes, skills and weights are first guesses, a
//! neutral adult's (a wether, a barrow, a steer), as cattle's are.

use super::bodies::HORSE;
use super::guesses::medians;
use super::{CreatureDef, Size};
use crate::sheet::{Ability, Trait};
use crate::skill::level_practice;

/// Sheep (ewes about 60 kg, rams 120): woolly grazers in flocks, quick to run.
pub(super) const SHEEP: CreatureDef = CreatureDef {
    name: "Sheep",
    weight: 70_000,
    size: Size::Small,
    inventory: (2, 2),
    // Four legs, a head, a neck and a tail, as a horse's.
    body: &HORSE,
    speed: 110,
    uses_stairs: false,
    playable: false,
    attributes: medians(60, 80, 70, 15),
    bonus_set: &[],
    skills: &[("athletics/running", level_practice(2))],
    abilities: &[Ability::Kick],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Goats (about 50 kg): sure-footed browsers that climb anything.
pub(super) const GOAT: CreatureDef = CreatureDef {
    name: "Goat",
    weight: 50_000,
    size: Size::Small,
    inventory: (2, 2),
    body: &HORSE,
    speed: 120,
    uses_stairs: false,
    playable: false,
    attributes: medians(60, 120, 75, 25),
    bonus_set: &[],
    skills: &[
        ("athletics/climbing", level_practice(4)),
        ("athletics/running", level_practice(2)),
    ],
    abilities: &[Ability::Kick],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Pigs (sows about 180 kg, boars more): rooting omnivores, swimming well.
pub(super) const PIG: CreatureDef = CreatureDef {
    name: "Pig",
    weight: 180_000,
    size: Size::Medium,
    inventory: (3, 2),
    body: &HORSE,
    speed: 100,
    uses_stairs: false,
    playable: false,
    attributes: medians(110, 60, 120, 35),
    bonus_set: &[],
    skills: &[
        ("athletics/running", level_practice(1)),
        ("athletics/swimming", level_practice(2)),
    ],
    abilities: &[Ability::Kick, Ability::Trample],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Water buffalo (about 600 kg): heavy oxen of wet lands, at home in mud and water.
pub(super) const WATER_BUFFALO: CreatureDef = CreatureDef {
    name: "Water buffalo",
    weight: 600_000,
    size: Size::Large,
    inventory: (4, 3),
    body: &HORSE,
    speed: 80,
    uses_stairs: false,
    playable: false,
    attributes: medians(180, 35, 170, 20),
    bonus_set: &[],
    skills: &[("athletics/swimming", level_practice(3))],
    abilities: &[Ability::Kick, Ability::Trample],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Yaks (cows about 250 kg, bulls 500): shaggy oxen of the high cold.
pub(super) const YAK: CreatureDef = CreatureDef {
    name: "Yak",
    weight: 400_000,
    size: Size::Large,
    inventory: (4, 3),
    body: &HORSE,
    speed: 85,
    uses_stairs: false,
    playable: false,
    attributes: medians(160, 60, 180, 20),
    bonus_set: &[],
    skills: &[
        ("athletics/climbing", level_practice(2)),
        ("athletics/running", level_practice(1)),
    ],
    abilities: &[Ability::Kick, Ability::Trample],
    looks: &[Trait::CoatColour, Trait::Markings],
};
