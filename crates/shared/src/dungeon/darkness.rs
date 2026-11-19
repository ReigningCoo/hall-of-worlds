//! Seeing in the dark (Alchemy plan PR 8): how far the light everyone carries
//! reaches, and how far Darkvision lets one see beyond it. Shared, so the
//! server works out a potion's reach and the client draws what it shows.
//! Sight itself (`Sight`) has no darkness rule yet: these only say how far is
//! lit, and walls stop Darkvision as they stop light.

use crate::item::Quality;

/// How far, in cells, the light every player carries reaches.
pub const CARRIED_LIGHT: u32 = 6;

/// The furthest Darkvision reaches, in cells (Claude's first guess, for the
/// owner to tune).
pub const MOST_DARKVISION: u32 = 24;

/// How far a Darkvision dose of `step` lets one see in the dark, in cells,
/// at this quality: the carried light's reach, plus 2 a step, plus a cell a
/// quality step over Poor, so both how strong and how pure the Perception
/// that made it was count; at most `MOST_DARKVISION`. A dose of 0 or less
/// gives none. Claude's first guess, for the owner to tune: a Common potion
/// of 4 steps reaches 15 cells.
pub fn darkvision_reach(step: i16, quality: Quality) -> u32 {
    if step <= 0 {
        return 0;
    }
    let stronger = 2 * u32::from(step.unsigned_abs());
    let purer = quality as u32;
    (CARRIED_LIGHT + stronger + purer).min(MOST_DARKVISION)
}
