//! A beast's body at each stage of its life (livestock plan PR 6): young
//! (`young.rs`), juvenile halfway to grown, and grown by its gender, a Neutral
//! one with a castrate's look where it has one (`castrates.rs`). For looks
//! only, as `Layout::of_gender` is.

use super::super::Layout;
use super::Beast;
use super::castrates::castrate_body;
use super::young::Young;
use crate::creature::CreatureKind;
use crate::gender::Gender;
use crate::livestock::Stage;

/// How far a juvenile is from its young body to its grown one, in thousandths.
pub const JUVENILE_GROWN: i32 = 500;

impl Layout {
    /// A kind's body for a creature of this gender at this stage of its life:
    /// a beast's young and juvenile bodies, and a castrate's for a Neutral one
    /// where it has a look; anything else as `Layout::of_gender`.
    pub fn of_stage(kind: CreatureKind, gender: Gender, stage: Stage) -> Option<Layout> {
        let grown = match gender {
            Gender::Neutral => castrate_body(kind).or_else(|| Layout::of(kind)),
            _ => Layout::of_gender(kind, gender),
        }?;
        let Some(beast) = Beast::of(kind) else {
            return Some(grown);
        };
        let young = || Young::of(kind, &beast).body(&beast.layout());
        Some(match stage {
            Stage::Adult => grown,
            Stage::Young => young(),
            Stage::Juvenile => Layout::blend(&young(), &grown, JUVENILE_GROWN),
        })
    }
}

#[cfg(test)]
mod tests;
