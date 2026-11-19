//! Crafting: making things, building included. A first draft, for the owner to change.

use crate::sheet::Attribute::{Agility, Intellect, Perception, Strength};

use super::super::draft::{Draft, branch, leaf};

pub(in super::super) const CRAFTING: Draft = branch(
    "crafting",
    "Crafting",
    &[Agility, Intellect],
    &[
        leaf("building", "Building", &[Strength, Intellect]),
        branch(
            "smithing",
            "Smithing",
            &[Strength, Agility],
            &[
                leaf("weapons", "Weaponsmithing", &[]),
                leaf("armour", "Armoursmithing", &[]),
                leaf("tools", "Blacksmithing", &[]),
            ],
        ),
        leaf("leatherwork", "Leatherwork", &[]),
        leaf("tailoring", "Tailoring", &[Agility]),
        leaf("woodwork", "Woodwork", &[]),
        leaf("cooking", "Cooking", &[Perception]),
        // One sub-skill per apparatus (Alchemy plan, owner, 2026-11-19), each
        // learned on first picking its apparatus up: `alchemy::Apparatus`.
        branch(
            "alchemy",
            "Alchemy",
            &[Intellect],
            &[
                leaf("grinding", "Grinding", &[]),
                leaf("infusing", "Infusing", &[]),
                leaf("distilling", "Distilling", &[]),
                leaf("calcining", "Calcining", &[]),
                leaf("filtering", "Filtering", &[]),
                leaf("circulating", "Circulating", &[]),
            ],
        ),
        // Keeping yeast and fermenting drinks (drinks plan).
        leaf("fermenting", "Fermenting", &[Perception]),
    ],
);
