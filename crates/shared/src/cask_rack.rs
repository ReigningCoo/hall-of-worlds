//! Cask racks (tavern furniture plan PR 1, owner's design): furniture against
//! a wall, 1 deep, each made for one cask size (`cask::CaskSize`). Pins and
//! Firkins lie three layers high and Kilderkins two, in a pyramid (two a cell
//! on the bottom, the next row in the grooves); each Barrel to Butt has a
//! cradle of its own, one a cell; a Tun is only ever installed on its stand,
//! two cells long, lying along the wall with its tap to the room. Sealed casks
//! on any of them age `RACK_AGEING_PERCENT` as fast, by the ground's world
//! time. What is on one is kept by the server as a crafting bench's tools are
//! (`furniture::Held`). See `docs/context/shared/cask_rack.md`.

// Split into files (`cargo xtask lint` keeps files small).
mod casks;
mod rack;

pub use casks::RackCasks;
pub use rack::{CaskRack, RackKind, RackPlace};

/// How fast sealed casks on a rack, cradle or stand age, in percent of on
/// the ground (owner's: 10% faster). Fermenting and the opened clock to
/// vinegar are unchanged.
pub const RACK_AGEING_PERCENT: u64 = 110;

#[cfg(test)]
mod tests;
