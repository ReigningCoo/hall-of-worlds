//! Kept animals from the livestock research (`docs/context/research-livestock.md`,
//! livestock plan PR 4): donkeys, mules, camels, llamas and alpacas, reindeer and
//! rabbits. Their rows in `shared::livestock` name these kinds (the Bactrian
//! camel's and the alpaca's are variants of the dromedary's and the llama's); their
//! bodies' looks are `shared::body::BEASTS`. Attributes, skills and weights are
//! first guesses, a neutral adult's, as cattle's are.

use super::bodies::HORSE;
use super::guesses::medians;
use super::{CreatureDef, Size};
use crate::sheet::{Ability, Trait};
use crate::skill::level_practice;

/// Donkeys (about 220 kg): small, hardy and patient pack animals.
pub(super) const DONKEY: CreatureDef = CreatureDef {
    name: "Donkey",
    weight: 220_000,
    size: Size::Medium,
    inventory: (3, 3),
    // Four legs, a head, a neck and a tail, as a horse's.
    body: &HORSE,
    speed: 110,
    uses_stairs: false,
    playable: false,
    attributes: medians(120, 80, 150, 35),
    bonus_set: &[],
    skills: &[("athletics/running", level_practice(2))],
    abilities: &[Ability::Kick],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Mules (about 420 kg): a donkey's sure feet on a horse's frame, stronger than
/// either.
pub(super) const MULE: CreatureDef = CreatureDef {
    name: "Mule",
    weight: 420_000,
    size: Size::Large,
    inventory: (4, 3),
    body: &HORSE,
    speed: 130,
    uses_stairs: false,
    playable: false,
    attributes: medians(170, 90, 170, 35),
    bonus_set: &[],
    skills: &[
        ("athletics/climbing", level_practice(2)),
        ("athletics/running", level_practice(3)),
    ],
    abilities: &[Ability::Kick, Ability::Trample],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Dromedaries (about 500 kg): one hump, long legs, days without water.
pub(super) const DROMEDARY: CreatureDef = CreatureDef {
    name: "Dromedary",
    weight: 500_000,
    size: Size::Large,
    inventory: (4, 3),
    body: &HORSE,
    speed: 130,
    uses_stairs: false,
    playable: false,
    attributes: medians(170, 80, 190, 30),
    bonus_set: &[],
    skills: &[("athletics/running", level_practice(3))],
    abilities: &[Ability::Kick, Ability::Trample],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Bactrian camels (about 550 kg): two humps and a shaggy winter coat, for cold
/// deserts.
pub(super) const BACTRIAN_CAMEL: CreatureDef = CreatureDef {
    name: "Bactrian camel",
    weight: 550_000,
    size: Size::Large,
    inventory: (4, 3),
    body: &HORSE,
    speed: 120,
    uses_stairs: false,
    playable: false,
    attributes: medians(180, 70, 200, 30),
    bonus_set: &[],
    skills: &[("athletics/running", level_practice(2))],
    abilities: &[Ability::Kick, Ability::Trample],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Llamas (about 150 kg): woolly pack animals of the mountains, quick to spit.
pub(super) const LLAMA: CreatureDef = CreatureDef {
    name: "Llama",
    weight: 150_000,
    size: Size::Medium,
    inventory: (3, 2),
    body: &HORSE,
    speed: 120,
    uses_stairs: false,
    playable: false,
    attributes: medians(100, 100, 120, 35),
    bonus_set: &[],
    skills: &[
        ("athletics/climbing", level_practice(2)),
        ("athletics/running", level_practice(2)),
    ],
    abilities: &[Ability::Kick],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Alpacas (about 65 kg): smaller than llamas, kept for their fine fleece.
pub(super) const ALPACA: CreatureDef = CreatureDef {
    name: "Alpaca",
    weight: 65_000,
    size: Size::Small,
    inventory: (2, 2),
    body: &HORSE,
    speed: 110,
    uses_stairs: false,
    playable: false,
    attributes: medians(60, 90, 80, 30),
    bonus_set: &[],
    skills: &[("athletics/climbing", level_practice(2))],
    abilities: &[Ability::Kick],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Reindeer (about 120 kg): herded deer of the far north, both sexes antlered.
pub(super) const REINDEER: CreatureDef = CreatureDef {
    name: "Reindeer",
    weight: 120_000,
    size: Size::Medium,
    inventory: (3, 2),
    body: &HORSE,
    speed: 140,
    uses_stairs: false,
    playable: false,
    attributes: medians(100, 120, 130, 25),
    bonus_set: &[],
    skills: &[
        ("athletics/running", level_practice(3)),
        ("athletics/swimming", level_practice(3)),
    ],
    abilities: &[Ability::Kick],
    looks: &[Trait::CoatColour, Trait::Markings],
};

/// Rabbits (about 2.5 kg): quick, timid and always watching, best at dawn and
/// dusk.
pub(super) const RABBIT: CreatureDef = CreatureDef {
    name: "Rabbit",
    weight: 2_500,
    size: Size::Tiny,
    inventory: (2, 2),
    body: &HORSE,
    speed: 130,
    uses_stairs: false,
    playable: false,
    attributes: medians(8, 140, 15, 15),
    bonus_set: &[],
    skills: &[("athletics/running", level_practice(3))],
    abilities: &[Ability::Kick],
    looks: &[Trait::CoatColour, Trait::Markings],
};
