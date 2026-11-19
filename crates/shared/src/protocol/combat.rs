//! What becomes of an ability used: for `ServerMessage::AbilityLanded`.

use serde::{Deserialize, Serialize};

use crate::item::Quality;
use crate::vitals::Points;

/// How an ability's wind-up ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Landed {
    /// It did this much harm, and the one hit has this much of their Health
    /// left, per mille (for health bars over tokens).
    Hit { harm: Points, left: u16 },
    /// Its target was no longer there to be reached (`shared::combat::in_reach`).
    Fizzled,
    /// It was called off before it landed.
    Cancelled,
    /// It gave this much Health (some of it lost if beyond the most), and the
    /// one healed has this much left, per mille.
    Healed { amount: Points, left: u16 },
    /// A kick landed on a door (`target` is `None`): this much of it is left,
    /// per mille; 0 means it broke.
    Door { left: u16 },
    /// A bottle was filled with this water (Collect Water: `target` is
    /// `None`).
    Filled(crate::potion::Water),
    /// A bottle was filled with this oil (Press Oil: `target` is `None`).
    Pressed(crate::potion::Oil),
    /// One run of an Alchemy process ended (`target` is `None`): what came
    /// of it, and how many runs of the order are left.
    Alchemy { made: Alchemised, left: u32 },
    /// Fruit was pressed (Press Juice: `target` is `None`): this many
    /// millilitres of juice went into a cask or vessel.
    Juiced(u32),
    /// A blow of Fell Tree landed (`target` is `None`): the tree is `done`
    /// per mille felled, 1000 when it fell.
    Chopped { done: u16 },
    /// A log was split (Cut Kindling: `target` is `None`) into this many
    /// pieces of kindling.
    Split(u32),
    /// A cask was set fermenting (Ferment: `target` is `None`): ready in this
    /// many game days.
    Fermented { days: u32 },
}

/// What one run of an Alchemy process made (`Landed::Alchemy`). The
/// ingredients are used up whatever it was.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alchemised {
    /// The chance roll failed.
    Failed,
    /// It worked, but came to nothing worth keeping.
    Nothing,
    /// It made this many of something at this quality: a Paste or Ointment,
    /// or bottles of potion (`ItemKind::Bottle`).
    Made {
        kind: crate::item::ItemKind,
        count: u32,
        quality: Quality,
    },
}
