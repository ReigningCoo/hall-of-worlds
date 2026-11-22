//! Fireplaces (tavern furniture plan PR 4, owner's design): stone fireplaces
//! against a wall, a chimney breast rising behind, 1 to 3 cells long (the 1 ×
//! 3 an inglenook, a settle either side of the fire inside it). Make Campfire
//! lights or feeds one as it does a hearth (`campfire::can_make_fire`), and
//! while lit it gives light as a campfire does (`campfire::LIGHT_CELLS`). A
//! log basket beside the fire holds kindling in places (`Fireside`), and fire
//! tools (poker, tongs, shovel, bellows) are part of its model. It cooks as a
//! worse hearth (owner's): a lit one counts as a hearth for heat
//! (`cooking::Heat::Hearth`, as a town's), but has no Oven and no tools of its
//! own, and no bench's `crafting_bench::BENCH_PERCENT` (Alchemy's only). Kept
//! by the server as other furniture with places (`furniture::Held`). See
//! `docs/context/shared/fireplace.md`.

// Split into files (`cargo xtask lint` keeps files small).
mod faced;
mod fireside;
mod kind;

pub use faced::fireplace_faced;
pub use fireside::Fireside;
pub use kind::{Fireplace, KINDLING_PER_PLACE};

#[cfg(test)]
mod tests;
