//! Crafting benches (owner's design, 2026-11-20): furniture holding the tools
//! of one craft, for anyone to use or take from. Only the Alchemy bench so far.
//!
//! A bench is a `furniture::Furniture::CraftingBench` piece, 1 deep and 1 to
//! 3 long by its craft, back to a wall. Its tools are items kept by the server
//! (`BenchTools`), put on through a panel that never says which kinds a craft
//! takes. Someone within reach facing any cell of it (`bench_faced`) uses its
//! tools before carried ones, and works `BENCH_PERCENT` better (Alchemy:
//! `alchemy::Process::at_bench`). Placed in sites' rooms after furniture, from
//! hashes, so no generator version or golden test moves (`place_benches`).

// Split into files (`cargo xtask lint` keeps files small).
mod craft;
mod faced;
mod place;
mod spot;
mod starting;
mod tools;

pub use craft::Craft;
pub use faced::bench_faced;
pub use place::{bench_percent, crafts_on_level, place_benches};
pub use starting::starting_tools;
pub use tools::{BenchTools, NotTaken};

/// How much better work at a bench comes out, in percent: the purity score ×
/// (100 + this)%, alongside each Pelican's (owner's 10%).
pub const BENCH_PERCENT: i32 = 10;

#[cfg(test)]
mod tests;
