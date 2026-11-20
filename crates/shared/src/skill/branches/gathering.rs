//! Gathering: taking what the land gives. A first draft, for the owner to change.

use crate::sheet::Attribute::{Intellect, Perception, Strength, Toughness};

use super::super::draft::{Draft, branch, leaf};

pub(in super::super) const GATHERING: Draft = branch(
    "gathering",
    "Gathering",
    &[Perception, Toughness],
    &[
        leaf("foraging", "Foraging", &[Perception]),
        branch(
            "farming",
            "Farming",
            &[Toughness],
            &[
                leaf("crops", "Crops", &[]),
                leaf("orchards", "Orchards", &[]),
                leaf("livestock", "Livestock", &[]),
                leaf("herbs", "Herbs", &[]),
            ],
        ),
        branch(
            "mining",
            "Mining",
            &[Strength, Toughness],
            &[
                leaf("stone", "Stone", &[]),
                leaf("ore", "Ore", &[]),
                leaf("gems", "Gems", &[]),
            ],
        ),
        // Felling trees and splitting logs (woodcutting plan): Fell Tree, Cut Kindling.
        leaf("woodcutting", "Woodcutting", &[Strength, Toughness]),
        leaf("prospecting", "Prospecting", &[Perception, Intellect]),
        leaf("herbalism", "Herbalism", &[Intellect, Perception]),
        // Knowing fungi (fungi plan PR 7): picking one teaches its name by a check.
        leaf("mycology", "Mycology", &[Intellect, Perception]),
        // Finding and collecting water (owner, 2026-11-17): practised by Collect Water.
        leaf("water", "Water", &[Perception]),
    ],
);
