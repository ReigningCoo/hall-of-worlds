//! Magic: a level shallower than weapons, as casting happens less often than attacks.
//! Each effect a spell can hold has its own leaf, and each where (how a spell reaches
//! what it acts on) one under Form (magic plan, `docs/context/magic-system.md`; owner,
//! 2026-11-23). Arcane leans on Intellect, divine on Empathy.

use crate::sheet::Attribute::{Aura, Empathy, Intellect, Perception, Will};

use super::super::draft::{Draft, branch, leaf};

pub(in super::super) const MAGIC: Draft = branch(
    "magic",
    "Magic",
    &[Intellect, Will],
    &[
        branch(
            "arcane",
            "Arcane",
            &[Intellect],
            &[
                leaf("fire", "Fire", &[]),
                leaf("water", "Water", &[]),
                leaf("air", "Air", &[]),
                leaf("earth", "Earth", &[]),
                leaf("void", "Void", &[]),
                leaf("teleport", "Teleport", &[]),
                leaf("haste", "Haste", &[]),
                leaf("levitate", "Levitate", &[]),
                leaf("mana", "Mana", &[]),
            ],
        ),
        branch(
            "divine",
            "Divine",
            &[Empathy],
            &[
                leaf("healing", "Healing", &[]),
                leaf("warding", "Warding", &[Empathy, Aura]),
                leaf("blessing", "Blessing", &[]),
                leaf("curse", "Curse", &[]),
                leaf("poison", "Poison", &[]),
                leaf("calm", "Calm", &[]),
            ],
        ),
        branch(
            "insight",
            "Insight",
            &[Perception, Intellect],
            &[
                leaf("identify", "Identify", &[]),
                leaf("illusion", "Illusion", &[Intellect, Will]),
                leaf("light", "Light", &[]),
            ],
        ),
        branch(
            "form",
            "Form",
            &[Intellect, Will],
            &[
                leaf("touch", "Touch", &[]),
                leaf("missile", "Missile", &[]),
                leaf("surround", "Surround", &[]),
                leaf("beam", "Beam", &[]),
                leaf("point", "Point", &[]),
            ],
        ),
    ],
);
