//! Cellar casks (drinks plan PR 9): castles and towers may have one room of
//! larger casks, on their basement if they have one, else their ground floor.
//!
//! Where they stand, and their sizes and woods, come from hashes of the
//! site's seed (`derive("cellar")`), after furniture, so no generator, golden
//! test or piece of furniture moves. What is in them is rolled by the server
//! the first time the level is visited (`stock`), as the casks are items with
//! the server's ids; they then lie on the ground like anything dropped,
//! blocking no one, for anyone to tap, fill, ferment in or carry off.

// Split into files (`cargo xtask lint` keeps files small).
mod contents;
mod place;

pub use contents::stock;
pub use place::{CellarCask, cellar_casks, cellar_level, cellar_percent};

#[cfg(test)]
mod tests;
