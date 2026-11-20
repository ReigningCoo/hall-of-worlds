//! Kept animals' coats, from the creature research's colours: cattle's (with the
//! aurochs's), highland cattle's, and (livestock plan PR 3) sheep's fleeces,
//! goats', pigs', water buffalo's and yaks'. Saved by place, so new ones go at the
//! end.

use super::looks::{Choice, brindled, colour};
use crate::gender::Gender;

pub(super) const CATTLE_COAT: &[Choice] = &[
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
pub(super) const COO_COAT: &[Choice] = &[
    colour("Red-brown", [140, 70, 40]),
    colour("Yellow-brown", [186, 140, 82]),
    colour("Silvery brown", [166, 150, 134]),
    colour("Black", [38, 33, 30]),
    brindled("Red-brown brindle", [140, 70, 40], [44, 34, 30]),
    brindled("Yellow-brown brindle", [186, 140, 82], [126, 62, 36]),
];

pub(super) const SHEEP_COAT: &[Choice] = &[
    colour("White", [236, 232, 220]),
    colour("Cream", [222, 208, 178]),
    colour("Grey", [150, 146, 140]),
    colour("Brown", [112, 78, 52]),
    colour("Black", [44, 40, 38]),
];

pub(super) const GOAT_COAT: &[Choice] = &[
    colour("White", [238, 236, 230]),
    colour("Black", [40, 36, 34]),
    colour("Brown", [118, 80, 50]),
    colour("Tan", [178, 138, 92]),
    colour("Grey", [140, 136, 130]),
];

pub(super) const PIG_COAT: &[Choice] = &[
    colour("Pink", [232, 180, 168]),
    colour("Black", [46, 42, 44]),
    colour("Ginger", [182, 96, 48]),
    colour("Sandy", [210, 170, 118]),
    colour("Grey-brown", [96, 86, 78]),
];

pub(super) const BUFFALO_COAT: &[Choice] = &[
    colour("Dark grey", [70, 70, 74]),
    colour("Black", [36, 34, 36]),
    colour("Slate", [100, 104, 112]),
    colour("Brownish grey", [92, 76, 66]),
];

pub(super) const YAK_COAT: &[Choice] = &[
    colour("Black", [36, 32, 30]),
    colour("Dark brown", [70, 50, 38]),
    colour("Brown", [110, 80, 58]),
    colour("White", [230, 226, 218]),
    colour("Grey", [132, 128, 124]),
];
