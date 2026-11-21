//! The server's side of a process once planned: whether it works, and luck.

use super::{Planned, capped};
use crate::alchemy::{luck_steps, moved, succeeds};
use crate::item::Quality;

/// What a process gave.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// It didn't work: the ingredients are used up and practice still given,
    /// but nothing is made.
    Failed,
    /// It worked: each output's quality, in `Planned::outputs`' order.
    Made { qualities: Vec<Quality> },
}

/// Settles a planned process with the server's two rolls, each from 0 to
/// `alchemy::CERTAIN` − 1: `chance_roll` against its chance, then `luck_roll`
/// for steps above each output's least quality (`alchemy::luck_steps`), the
/// cap away from a hearth still kept.
pub fn outcome(planned: &Planned, chance_roll: u32, luck_roll: u32) -> Outcome {
    if !succeeds(planned.chance, chance_roll) {
        return Outcome::Failed;
    }
    let luck = luck_steps(luck_roll);
    Outcome::Made {
        qualities: (planned.outputs.iter())
            .map(|output| capped(moved(output.quality, luck), planned.heat))
            .collect(),
    }
}
