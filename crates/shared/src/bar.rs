//! Bar counters (tavern furniture plan PR 3, owner's design): furniture with
//! places (`furniture::Held`) for drinking vessels, 1 to 3 cells long, 1
//! deep, joining into longer runs, with a gap or a hinged flap to get behind.
//! A single counter keeps its vessels on a shelf underneath; a double one on
//! a raised shelf above, with room underneath for the barkeep to serve. Used
//! like a crafting bench: at the bar, a carried vessel is filled with any
//! drink stored in the same room (`room_storage`: cask racks, cradles, Tun
//! stands, wine and spirit racks and the bar's own places), by the pour rule
//! (`shared::pour`), never from anything lying on the ground. Kept by the
//! server as racks are. See `docs/context/shared/bar.md`.

// Split into files (`cargo xtask lint` keeps files small).
mod counter;
mod storage;
mod vessels;

pub use counter::{BarCounter, BarKind, BarPlace, VESSELS_PER_CELL};
pub use storage::{drinks_in, room_storage, vessels_to_fill};
pub use vessels::BarVessels;

#[cfg(test)]
mod tests;
