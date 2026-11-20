//! Kept animals' coats, from the creature research's colours: cattle's (with the
//! aurochs's), highland cattle's, and (livestock plan PR 3) sheep's fleeces,
//! goats', pigs', water buffalo's and yaks', and (PR 4) donkeys', mules', camels',
//! llamas', alpacas', reindeer's and rabbits'. Saved by place, so new ones go at
//! the end.

use super::looks::{Choice, brindled, colour};
use crate::creature::CreatureKind;
use crate::gender::Gender;

/// A kept animal's coats, or `None` for a kind that isn't one (or a horse, whose
/// are `looks.rs`'s).
pub(super) fn livestock_coat(kind: CreatureKind) -> Option<&'static [Choice]> {
    use CreatureKind::*;
    Some(match kind {
        Cattle => CATTLE_COAT,
        Coo => COO_COAT,
        Sheep => SHEEP_COAT,
        Goat => GOAT_COAT,
        Pig => PIG_COAT,
        WaterBuffalo => BUFFALO_COAT,
        Yak => YAK_COAT,
        Donkey => DONKEY_COAT,
        Mule => MULE_COAT,
        Dromedary => DROMEDARY_COAT,
        BactrianCamel => BACTRIAN_COAT,
        Llama => LLAMA_COAT,
        Alpaca => ALPACA_COAT,
        Reindeer => REINDEER_COAT,
        Rabbit => RABBIT_COAT,
        _ => return None,
    })
}

const CATTLE_COAT: &[Choice] = &[
    colour("Black", [38, 34, 32]),
    colour("Brown", [110, 72, 46]),
    colour("Red", [146, 70, 42]),
    colour("White", [232, 228, 218]),
    colour("Dun", [176, 146, 104]),
    // An aurochs's (wild cattle; mobs don't roll these for kept cattle): bulls
    // near black, cows a lighter red-brown.
    colour("Aurochs", [34, 28, 26]).only(Gender::Male),
    colour("Aurochs", [150, 82, 48]).only(Gender::Female),
];

/// Highland cattle: one long coat, mostly reddish brown; mobs roll these with the
/// weights in `mob::looks`.
const COO_COAT: &[Choice] = &[
    colour("Red-brown", [140, 70, 40]),
    colour("Yellow-brown", [186, 140, 82]),
    colour("Silvery brown", [166, 150, 134]),
    colour("Black", [38, 33, 30]),
    brindled("Red-brown brindle", [140, 70, 40], [44, 34, 30]),
    brindled("Yellow-brown brindle", [186, 140, 82], [126, 62, 36]),
];

const SHEEP_COAT: &[Choice] = &[
    colour("White", [236, 232, 220]),
    colour("Cream", [222, 208, 178]),
    colour("Grey", [150, 146, 140]),
    colour("Brown", [112, 78, 52]),
    colour("Black", [44, 40, 38]),
];

const GOAT_COAT: &[Choice] = &[
    colour("White", [238, 236, 230]),
    colour("Black", [40, 36, 34]),
    colour("Brown", [118, 80, 50]),
    colour("Tan", [178, 138, 92]),
    colour("Grey", [140, 136, 130]),
];

const PIG_COAT: &[Choice] = &[
    colour("Pink", [232, 180, 168]),
    colour("Black", [46, 42, 44]),
    colour("Ginger", [182, 96, 48]),
    colour("Sandy", [210, 170, 118]),
    colour("Grey-brown", [96, 86, 78]),
];

const BUFFALO_COAT: &[Choice] = &[
    colour("Dark grey", [70, 70, 74]),
    colour("Black", [36, 34, 36]),
    colour("Slate", [100, 104, 112]),
    colour("Brownish grey", [92, 76, 66]),
];

const YAK_COAT: &[Choice] = &[
    colour("Black", [36, 32, 30]),
    colour("Dark brown", [70, 50, 38]),
    colour("Brown", [110, 80, 58]),
    colour("White", [230, 226, 218]),
    colour("Grey", [132, 128, 124]),
];

/// Donkeys: mostly grey-dun; mobs roll these with the weights in `mob::looks`.
const DONKEY_COAT: &[Choice] = &[
    colour("Grey-dun", [138, 128, 118]),
    colour("Brown", [104, 76, 56]),
    colour("Black", [44, 40, 38]),
    colour("White", [230, 226, 218]),
    colour("Dun", [170, 144, 108]),
];

/// Mules: any horse's or donkey's colour, bay and brown the commonest.
const MULE_COAT: &[Choice] = &[
    colour("Bay", [120, 66, 36]),
    colour("Brown", [96, 66, 46]),
    colour("Black", [40, 36, 34]),
    colour("Grey", [150, 146, 140]),
    colour("Chestnut", [146, 78, 40]),
    colour("Dun", [170, 140, 100]),
];

const DROMEDARY_COAT: &[Choice] = &[
    colour("Tan", [190, 150, 100]),
    colour("Sandy", [208, 178, 130]),
    colour("Brown", [130, 92, 60]),
    colour("Dark brown", [84, 60, 44]),
    colour("White", [232, 224, 206]),
];

const BACTRIAN_COAT: &[Choice] = &[
    colour("Sandy beige", [192, 160, 116]),
    colour("Brown", [120, 86, 58]),
    colour("Dark brown", [76, 56, 42]),
    colour("Grey-beige", [170, 156, 136]),
];

const LLAMA_COAT: &[Choice] = &[
    colour("White", [236, 232, 222]),
    colour("Brown", [116, 80, 54]),
    colour("Black", [42, 38, 36]),
    colour("Grey", [140, 136, 130]),
    colour("Fawn", [196, 158, 110]),
];

const ALPACA_COAT: &[Choice] = &[
    colour("White", [238, 234, 224]),
    colour("Cream", [226, 210, 176]),
    colour("Fawn", [200, 162, 114]),
    colour("Brown", [112, 76, 52]),
    colour("Black", [40, 36, 34]),
    colour("Grey", [146, 142, 136]),
];

const REINDEER_COAT: &[Choice] = &[
    colour("Grey-brown", [116, 98, 80]),
    colour("Brown", [96, 72, 52]),
    colour("Dark brown", [64, 50, 40]),
    colour("Pale grey", [176, 170, 160]),
    colour("White", [228, 226, 220]),
];

const RABBIT_COAT: &[Choice] = &[
    colour("Wild brown", [128, 100, 74]),
    colour("Grey", [130, 126, 120]),
    colour("White", [238, 236, 232]),
    colour("Black", [40, 38, 38]),
    colour("Fawn", [200, 160, 110]),
];
